#![doc = "エンドポイントのビルダー。パスのセグメントごとに 1 つのモジュールがある。"]
pub mod admin;
pub mod announcements;
pub mod antennas;
pub mod ap;
pub mod app;
pub mod auth;
pub mod blocking;
pub mod bubble_game;
pub mod channels;
pub mod charts;
pub mod clips;
pub mod drive;
pub mod email_address;
pub mod emoji;
pub mod emojis;
pub mod endpoint;
pub mod endpoints;
pub mod export_custom_emojis;
pub mod federation;
pub mod fetch_external_resources;
pub mod fetch_rss;
pub mod flash;
pub mod following;
pub mod gallery;
pub mod get_avatar_decorations;
pub mod get_online_users_count;
pub mod hashtags;
pub mod i;
pub mod invite;
pub mod meta;
pub mod miauth;
pub mod mute;
pub mod my;
pub mod notes;
pub mod notifications;
pub mod page_push;
pub mod pages;
pub mod ping;
pub mod pinned_users;
pub mod promo;
pub mod renote_mute;
pub mod request_reset_password;
pub mod reset_db;
pub mod reset_password;
pub mod retention;
pub mod reversi;
pub mod roles;
pub mod server_info;
pub mod stats;
pub mod sw;
pub mod test;
pub mod username;
pub mod users;
impl super::Client {
    #[doc = "`admin/` 以下のエンドポイント"]
    pub fn admin(&self) -> admin::Call<'_> {
        admin::Call::new(self)
    }
    #[doc = "`announcements` と、`announcements/` 以下のエンドポイント"]
    pub fn announcements(&self) -> announcements::Call<'_> {
        announcements::Call::new(self)
    }
    #[doc = "`antennas/` 以下のエンドポイント"]
    pub fn antennas(&self) -> antennas::Call<'_> {
        antennas::Call::new(self)
    }
    #[doc = "`ap/` 以下のエンドポイント"]
    pub fn ap(&self) -> ap::Call<'_> {
        ap::Call::new(self)
    }
    #[doc = "`app/` 以下のエンドポイント"]
    pub fn app(&self) -> app::Call<'_> {
        app::Call::new(self)
    }
    #[doc = "`auth/` 以下のエンドポイント"]
    pub fn auth(&self) -> auth::Call<'_> {
        auth::Call::new(self)
    }
    #[doc = "`blocking/` 以下のエンドポイント"]
    pub fn blocking(&self) -> blocking::Call<'_> {
        blocking::Call::new(self)
    }
    #[doc = "`channels/` 以下のエンドポイント"]
    pub fn channels(&self) -> channels::Call<'_> {
        channels::Call::new(self)
    }
    #[doc = "`charts/` 以下のエンドポイント"]
    pub fn charts(&self) -> charts::Call<'_> {
        charts::Call::new(self)
    }
    #[doc = "`clips/` 以下のエンドポイント"]
    pub fn clips(&self) -> clips::Call<'_> {
        clips::Call::new(self)
    }
    #[doc = "`drive` と、`drive/` 以下のエンドポイント"]
    pub fn drive(&self) -> drive::Call<'_> {
        drive::Call::new(self)
    }
    #[doc = "`email-address/` 以下のエンドポイント"]
    pub fn email_address(&self) -> email_address::Call<'_> {
        email_address::Call::new(self)
    }
    #[doc = "`endpoint`"]
    pub fn endpoint(&self) -> endpoint::Call<'_> {
        endpoint::Call::new(self)
    }
    #[doc = "`endpoints`"]
    pub fn endpoints(&self) -> endpoints::Call<'_> {
        endpoints::Call::new(self)
    }
    #[doc = "`export-custom-emojis`"]
    pub fn export_custom_emojis(&self) -> export_custom_emojis::Call<'_> {
        export_custom_emojis::Call::new(self)
    }
    #[doc = "`federation/` 以下のエンドポイント"]
    pub fn federation(&self) -> federation::Call<'_> {
        federation::Call::new(self)
    }
    #[doc = "`following/` 以下のエンドポイント"]
    pub fn following(&self) -> following::Call<'_> {
        following::Call::new(self)
    }
    #[doc = "`gallery/` 以下のエンドポイント"]
    pub fn gallery(&self) -> gallery::Call<'_> {
        gallery::Call::new(self)
    }
    #[doc = "`get-online-users-count`"]
    pub fn get_online_users_count(&self) -> get_online_users_count::Call<'_> {
        get_online_users_count::Call::new(self)
    }
    #[doc = "`get-avatar-decorations`"]
    pub fn get_avatar_decorations(&self) -> get_avatar_decorations::Call<'_> {
        get_avatar_decorations::Call::new(self)
    }
    #[doc = "`hashtags/` 以下のエンドポイント"]
    pub fn hashtags(&self) -> hashtags::Call<'_> {
        hashtags::Call::new(self)
    }
    #[doc = "`i` と、`i/` 以下のエンドポイント"]
    pub fn i(&self) -> i::Call<'_> {
        i::Call::new(self)
    }
    #[doc = "`invite/` 以下のエンドポイント"]
    pub fn invite(&self) -> invite::Call<'_> {
        invite::Call::new(self)
    }
    #[doc = "`meta`"]
    pub fn meta(&self) -> meta::Call<'_> {
        meta::Call::new(self)
    }
    #[doc = "`emojis`"]
    pub fn emojis(&self) -> emojis::Call<'_> {
        emojis::Call::new(self)
    }
    #[doc = "`emoji`"]
    pub fn emoji(&self) -> emoji::Call<'_> {
        emoji::Call::new(self)
    }
    #[doc = "`miauth/` 以下のエンドポイント"]
    pub fn miauth(&self) -> miauth::Call<'_> {
        miauth::Call::new(self)
    }
    #[doc = "`mute/` 以下のエンドポイント"]
    pub fn mute(&self) -> mute::Call<'_> {
        mute::Call::new(self)
    }
    #[doc = "`renote-mute/` 以下のエンドポイント"]
    pub fn renote_mute(&self) -> renote_mute::Call<'_> {
        renote_mute::Call::new(self)
    }
    #[doc = "`my/` 以下のエンドポイント"]
    pub fn my(&self) -> my::Call<'_> {
        my::Call::new(self)
    }
    #[doc = "`notes` と、`notes/` 以下のエンドポイント"]
    pub fn notes(&self) -> notes::Call<'_> {
        notes::Call::new(self)
    }
    #[doc = "`notifications/` 以下のエンドポイント"]
    pub fn notifications(&self) -> notifications::Call<'_> {
        notifications::Call::new(self)
    }
    #[doc = "`page-push`"]
    pub fn page_push(&self) -> page_push::Call<'_> {
        page_push::Call::new(self)
    }
    #[doc = "`pages/` 以下のエンドポイント"]
    pub fn pages(&self) -> pages::Call<'_> {
        pages::Call::new(self)
    }
    #[doc = "`flash/` 以下のエンドポイント"]
    pub fn flash(&self) -> flash::Call<'_> {
        flash::Call::new(self)
    }
    #[doc = "`ping`"]
    pub fn ping(&self) -> ping::Call<'_> {
        ping::Call::new(self)
    }
    #[doc = "`pinned-users`"]
    pub fn pinned_users(&self) -> pinned_users::Call<'_> {
        pinned_users::Call::new(self)
    }
    #[doc = "`promo/` 以下のエンドポイント"]
    pub fn promo(&self) -> promo::Call<'_> {
        promo::Call::new(self)
    }
    #[doc = "`roles/` 以下のエンドポイント"]
    pub fn roles(&self) -> roles::Call<'_> {
        roles::Call::new(self)
    }
    #[doc = "`request-reset-password`"]
    pub fn request_reset_password(&self) -> request_reset_password::Call<'_> {
        request_reset_password::Call::new(self)
    }
    #[doc = "`reset-db`"]
    pub fn reset_db(&self) -> reset_db::Call<'_> {
        reset_db::Call::new(self)
    }
    #[doc = "`reset-password`"]
    pub fn reset_password(&self) -> reset_password::Call<'_> {
        reset_password::Call::new(self)
    }
    #[doc = "`server-info`"]
    pub fn server_info(&self) -> server_info::Call<'_> {
        server_info::Call::new(self)
    }
    #[doc = "`stats`"]
    pub fn stats(&self) -> stats::Call<'_> {
        stats::Call::new(self)
    }
    #[doc = "`sw/` 以下のエンドポイント"]
    pub fn sw(&self) -> sw::Call<'_> {
        sw::Call::new(self)
    }
    #[doc = "`test`"]
    pub fn test(&self) -> test::Call<'_> {
        test::Call::new(self)
    }
    #[doc = "`username/` 以下のエンドポイント"]
    pub fn username(&self) -> username::Call<'_> {
        username::Call::new(self)
    }
    #[doc = "`users` と、`users/` 以下のエンドポイント"]
    pub fn users(&self) -> users::Call<'_> {
        users::Call::new(self)
    }
    #[doc = "`fetch-rss`"]
    pub fn fetch_rss(&self) -> fetch_rss::Call<'_> {
        fetch_rss::Call::new(self)
    }
    #[doc = "`fetch-external-resources`"]
    pub fn fetch_external_resources(&self) -> fetch_external_resources::Call<'_> {
        fetch_external_resources::Call::new(self)
    }
    #[doc = "`retention`"]
    pub fn retention(&self) -> retention::Call<'_> {
        retention::Call::new(self)
    }
    #[doc = "`bubble-game/` 以下のエンドポイント"]
    pub fn bubble_game(&self) -> bubble_game::Call<'_> {
        bubble_game::Call::new(self)
    }
    #[doc = "`reversi/` 以下のエンドポイント"]
    pub fn reversi(&self) -> reversi::Call<'_> {
        reversi::Call::new(self)
    }
}
