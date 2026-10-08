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
    // fixture 用のサーバは、初期設定ウィザードを終えたときと同じ状態にしてある。
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
    let note: types::Note = parse(&fixture("notes-show"));
    let renote = note.renote.as_deref().expect("notes-show はリノートのはず");
    assert_eq!(renote.user.host.as_deref(), Some("remote.example.com"));
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
    // `users/show` は、`userIds` を渡すと配列を返す。
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
    let client = Client::new(&server.uri(), None);
    match client.notes().show().note_id("nonexistent").await {
        Err(Error::Api(error)) => {
            assert_eq!(error.code, "NO_SUCH_NOTE");
            assert_eq!(error.status, 400);
        }
        other => panic!("想定外の結果: {other:?}"),
    }
}
