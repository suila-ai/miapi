//! ダミーのデータを入れた Misskey サーバから採取した応答（`scripts/update-fixtures.sh`）が、
//! 選んだバージョンの生成済みの型にデシリアライズできることを確かめる。

mod common;

use common::fixture;
use common::version::{Client, api, types};
use miapi::Error;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn parse<T: serde::de::DeserializeOwned>(json: &str) -> T {
    serde_json::from_str(json).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn meta_with_detail_is_meta_detailed() {
    let meta: api::meta::Response = parse(&fixture("meta"));
    let api::meta::Response::MetaDetailed(meta) = meta else {
        panic!("MetaDetailed のはずが {meta:?} だった");
    };
    assert_eq!(meta.name.as_deref(), Some("Example Misskey"));
    // fixture 用のサーバは初期設定ウィザードを終えたときと同じ状態にしてある。
    assert_eq!(meta.policies.rate_limit_factor, 0.3);
    let features = meta
        .features
        .expect("MetaDetailed には features があるはず");
    assert_eq!(features.local_timeline, None);
}

#[test]
fn timelines_are_notes() {
    let notes: api::notes::local_timeline::Response = parse(&fixture("notes-local-timeline"));
    assert!(!notes.is_empty());
    assert_eq!(notes[0].visibility, types::NoteVisibility::Public);
    let json = fixture("notes-show");
    let note: types::Note = parse(&json);
    let renote = note.renote.as_deref().expect("notes-show はリノートのはず");
    assert_eq!(renote.user.host.as_deref(), Some("remote.example.com"));
    let raw: serde_json::Value = parse(&json);
    let created_at =
        chrono::DateTime::parse_from_rfc3339(raw["createdAt"].as_str().unwrap()).unwrap();
    assert_eq!(note.created_at, created_at);
    let reencoded = serde_json::to_string(&note).unwrap();
    let again: types::Note = parse(&reencoded);
    assert_eq!(again, note);
}

#[test]
fn users_are_detailed_not_me() {
    let users: api::users::Response = parse(&fixture("users"));
    assert_eq!(users.len(), 3);
    for user in &users {
        assert!(
            matches!(user, types::UserDetailed::UserDetailedNotMe(_)),
            "{user:?}"
        );
    }
    let shown: api::users::show::Response = parse(&fixture("users-show"));
    let api::users::show::Response::UserDetailedNotMe(user) = shown else {
        panic!("単一の UserDetailedNotMe のはずが {shown:?} だった");
    };
    assert_eq!(user.username, "alice");
    assert_eq!(user.is_following, None);
}

#[test]
fn union_variants_after_the_first_are_reachable() {
    // `users/show` は `userIds` を渡すと配列を返す。
    let listed: api::users::show::Response = parse(&format!("[{}]", fixture("users-show")));
    assert!(
        matches!(&listed, api::users::show::Response::Array(users) if users.len() == 1),
        "{listed:?}"
    );
    // ノートの `user` は UserLite で、`users/search` に `detail: false` を渡したときの要素と同じ形である。
    let note: serde_json::Value = parse(&fixture("notes-show"));
    let lite: types::User = parse(&note["user"].to_string());
    assert!(matches!(lite, types::User::UserLite(_)), "{lite:?}");
    let detailed: types::User = parse(&fixture("users-show"));
    assert!(
        matches!(detailed, types::User::UserDetailedNotMe(_)),
        "{detailed:?}"
    );
}

#[test]
fn own_account_is_me_detailed() {
    let me: api::i::Response = parse(&fixture("i"));
    assert_eq!(me.username, "alice");
    assert_eq!(me.policies.rate_limit_factor, 0.3);
    assert_eq!(me.unread_announcements.len(), 1);
    assert_eq!(me.unread_announcements[0].for_you, None);
}

#[test]
fn other_public_endpoints() {
    let instances: api::federation::instances::Response = parse(&fixture("federation-instances"));
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].host, "remote.example.com");
    let emojis: api::emojis::Response = parse(&fixture("emojis"));
    assert_eq!(emojis.emojis.len(), 2);
    let trend: api::hashtags::trend::Response = parse(&fixture("hashtags-trend"));
    assert!(!trend.is_empty());
}

#[tokio::test]
async fn error_fixture_maps_to_api_error() {
    let server = MockServer::start().await;
    Mock::given(path("/api/notes/show"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_raw(fixture("error-no-such-note"), "application/json"),
        )
        .mount(&server)
        .await;
    let client = Client::new(&server.uri(), None).unwrap();
    match client.notes().show().note_id("nonexistent").await {
        Err(Error::Api(error)) => {
            assert_eq!(error.code, "NO_SUCH_NOTE");
            assert_eq!(error.status, 400);
        }
        other => panic!("想定外の結果: {other:?}"),
    }
}

/// ストリーミングのイベントの fixture（`stream-*.json`。チャンネルごとに、`{type, body}` を type ごと（noteUpdated は本文の type ごと）に 1 件ずつ並べた配列）が、
/// 選んだバージョンの生成済みの `Event` / `BroadcastEvent` にデシリアライズできることを確かめる。
/// serverStats と queueStats は、fixture 用のサーバ（NODE_ENV=test）が統計を取らないので fixture が無い。
#[cfg(feature = "streaming")]
mod streaming {
    use std::fmt::Debug;

    use super::{fixture, parse};
    use crate::common::version::streaming::{self, BroadcastEvent, types};

    /// fixture の全イベントが読めることを確かめて返す。選んだバージョンの fixture なら、`Unrecognized` に
    /// 落ちたものが無いことも確かめる。別のバージョンの応答（`MIAPI_FIXTURES`）では、知らない値が
    /// `Unrecognized` に入るのが正しい動作なので、確かめない。
    fn events<T: serde::de::DeserializeOwned + Debug>(name: &str) -> Vec<T> {
        let events: Vec<T> = parse(&fixture(name));
        assert!(!events.is_empty(), "{name} が空");
        if std::env::var_os("MIAPI_FIXTURES").is_none() {
            for event in &events {
                let debug = format!("{event:?}");
                assert!(
                    !debug.contains("Unrecognized"),
                    "{name}: バインディングが知らないイベント: {debug}"
                );
            }
        }
        events
    }

    /// fixture に、`pick` に当たるイベントがあることを確かめる。ほかのイベントが並んでいてもよい
    /// （別のバージョンの応答では `Unrecognized` も）。
    fn assert_has<T: serde::de::DeserializeOwned + Debug>(name: &str, pick: fn(&T) -> bool) {
        let events = events::<T>(name);
        assert!(events.iter().any(pick), "{name}: {events:?}");
    }

    #[test]
    fn timelines_deliver_notes() {
        use streaming::{
            antenna, channel, global_timeline, hashtag, home_timeline, hybrid_timeline,
            local_timeline, user_list,
        };
        assert_has("stream-local-timeline", |e: &local_timeline::Event| {
            matches!(e, local_timeline::Event::Note(_))
        });
        assert_has("stream-global-timeline", |e: &global_timeline::Event| {
            matches!(e, global_timeline::Event::Note(_))
        });
        assert_has("stream-home-timeline", |e: &home_timeline::Event| {
            matches!(e, home_timeline::Event::Note(_))
        });
        assert_has("stream-hybrid-timeline", |e: &hybrid_timeline::Event| {
            matches!(e, hybrid_timeline::Event::Note(_))
        });
        assert_has("stream-hashtag", |e: &hashtag::Event| {
            matches!(e, hashtag::Event::Note(_))
        });
        assert_has("stream-antenna", |e: &antenna::Event| {
            matches!(e, antenna::Event::Note(_))
        });
        assert_has("stream-user-list", |e: &user_list::Event| {
            matches!(e, user_list::Event::Note(_))
        });
        assert_has("stream-channel", |e: &channel::Event| {
            matches!(e, channel::Event::Note(_))
        });
    }

    #[test]
    fn main_events() {
        use streaming::main::Event;
        let events = events::<Event>("stream-main");
        let has = |pick: fn(&Event) -> bool| events.iter().any(pick);
        assert!(has(|e| matches!(e, Event::Notification(_))));
        assert!(has(|e| matches!(e, Event::UnreadNotification(_))));
        assert!(has(|e| matches!(e, Event::Mention(_))));
        assert!(has(|e| matches!(e, Event::Reply(_))));
        assert!(has(|e| matches!(e, Event::Renote(_))));
        assert!(has(|e| matches!(e, Event::Follow(_))));
        assert!(has(|e| matches!(e, Event::Followed(_))));
        assert!(has(|e| matches!(e, Event::Unfollow(_))));
        assert!(has(|e| matches!(e, Event::MeUpdated(_))));
        assert!(has(|e| matches!(e, Event::DriveFileCreated(_))));
        assert!(has(|e| matches!(e, Event::ReadAllNotifications)));
        assert!(has(|e| matches!(e, Event::RegistryUpdated(_))));
        // URL からのアップロードで marker を省くと、marker は null で届く。
        assert!(has(
            |e| matches!(e, Event::UrlUploadFinished(finished) if finished.marker.is_none())
        ));
    }

    #[test]
    fn drive_and_admin_events() {
        use streaming::drive::Event;
        let drive = events::<Event>("stream-drive");
        let has = |pick: fn(&Event) -> bool| drive.iter().any(pick);
        assert!(has(|e| matches!(e, Event::FileCreated(_))));
        assert!(has(
            |e| matches!(e, Event::FileDeleted(id) if !id.is_empty())
        ));
        assert!(has(|e| matches!(e, Event::FolderUpdated(_))));
        assert_has("stream-admin", |e: &streaming::admin::Event| match e {
            streaming::admin::Event::NewAbuseUserReport(report) => report.comment == "dummy report",
            _ => false,
        });
    }

    #[test]
    fn broadcast_events() {
        use types::NoteUpdatedEvent;
        let events = events::<BroadcastEvent>("stream-broadcast");
        let has = |pick: fn(&BroadcastEvent) -> bool| events.iter().any(pick);
        // カスタム絵文字のリアクションには、絵文字の名前と URL が付く。
        assert!(
            has(|e| matches!(
                e,
                BroadcastEvent::NoteUpdated(NoteUpdatedEvent::Reacted(reacted))
                    if reacted.body.emoji.as_ref().is_some_and(|emoji| emoji.name == "emoji1@.")
            )),
            "{events:?}"
        );
        assert!(has(|e| matches!(
            e,
            BroadcastEvent::NoteUpdated(NoteUpdatedEvent::Unreacted(_))
        )));
        assert!(has(|e| matches!(
            e,
            BroadcastEvent::NoteUpdated(NoteUpdatedEvent::PollVoted(_))
        )));
        assert!(has(|e| matches!(
            e,
            BroadcastEvent::NoteUpdated(NoteUpdatedEvent::Deleted(_))
        )));
        assert!(has(|e| matches!(e, BroadcastEvent::EmojiAdded(_))));
        assert!(has(|e| matches!(e, BroadcastEvent::EmojiUpdated(_))));
        assert!(has(|e| matches!(e, BroadcastEvent::EmojiDeleted(_))));
        assert!(has(|e| matches!(e, BroadcastEvent::AnnouncementCreated(_))));
    }
}
