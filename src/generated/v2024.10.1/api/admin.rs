#![doc = "`admin/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`admin/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`admin/meta`"]
    pub fn meta(&self) -> meta::Call<'a> {
        meta::Call::new(self.client)
    }
    #[doc = "`admin/abuse-user-reports`"]
    pub fn abuse_user_reports(&self) -> abuse_user_reports::Call<'a> {
        abuse_user_reports::Call::new(self.client)
    }
    #[doc = "`admin/abuse-report/` 以下のエンドポイント"]
    pub fn abuse_report(&self) -> abuse_report::Call<'a> {
        abuse_report::Call::new(self.client)
    }
    #[doc = "`admin/accounts/` 以下のエンドポイント"]
    pub fn accounts(&self) -> accounts::Call<'a> {
        accounts::Call::new(self.client)
    }
    #[doc = "`admin/ad/` 以下のエンドポイント"]
    pub fn ad(&self) -> ad::Call<'a> {
        ad::Call::new(self.client)
    }
    #[doc = "`admin/announcements/` 以下のエンドポイント"]
    pub fn announcements(&self) -> announcements::Call<'a> {
        announcements::Call::new(self.client)
    }
    #[doc = "`admin/avatar-decorations/` 以下のエンドポイント"]
    pub fn avatar_decorations(&self) -> avatar_decorations::Call<'a> {
        avatar_decorations::Call::new(self.client)
    }
    #[doc = "`admin/delete-all-files-of-a-user`"]
    pub fn delete_all_files_of_a_user(&self) -> delete_all_files_of_a_user::Call<'a> {
        delete_all_files_of_a_user::Call::new(self.client)
    }
    #[doc = "`admin/unset-user-avatar`"]
    pub fn unset_user_avatar(&self) -> unset_user_avatar::Call<'a> {
        unset_user_avatar::Call::new(self.client)
    }
    #[doc = "`admin/unset-user-banner`"]
    pub fn unset_user_banner(&self) -> unset_user_banner::Call<'a> {
        unset_user_banner::Call::new(self.client)
    }
    #[doc = "`admin/drive/` 以下のエンドポイント"]
    pub fn drive(&self) -> drive::Call<'a> {
        drive::Call::new(self.client)
    }
    #[doc = "`admin/emoji/` 以下のエンドポイント"]
    pub fn emoji(&self) -> emoji::Call<'a> {
        emoji::Call::new(self.client)
    }
    #[doc = "`admin/federation/` 以下のエンドポイント"]
    pub fn federation(&self) -> federation::Call<'a> {
        federation::Call::new(self.client)
    }
    #[doc = "`admin/get-index-stats`"]
    pub fn get_index_stats(&self) -> get_index_stats::Call<'a> {
        get_index_stats::Call::new(self.client)
    }
    #[doc = "`admin/get-table-stats`"]
    pub fn get_table_stats(&self) -> get_table_stats::Call<'a> {
        get_table_stats::Call::new(self.client)
    }
    #[doc = "`admin/get-user-ips`"]
    pub fn get_user_ips(&self) -> get_user_ips::Call<'a> {
        get_user_ips::Call::new(self.client)
    }
    #[doc = "`admin/invite/` 以下のエンドポイント"]
    pub fn invite(&self) -> invite::Call<'a> {
        invite::Call::new(self.client)
    }
    #[doc = "`admin/promo/` 以下のエンドポイント"]
    pub fn promo(&self) -> promo::Call<'a> {
        promo::Call::new(self.client)
    }
    #[doc = "`admin/queue/` 以下のエンドポイント"]
    pub fn queue(&self) -> queue::Call<'a> {
        queue::Call::new(self.client)
    }
    #[doc = "`admin/relays/` 以下のエンドポイント"]
    pub fn relays(&self) -> relays::Call<'a> {
        relays::Call::new(self.client)
    }
    #[doc = "`admin/reset-password`"]
    pub fn reset_password(&self) -> reset_password::Call<'a> {
        reset_password::Call::new(self.client)
    }
    #[doc = "`admin/resolve-abuse-user-report`"]
    pub fn resolve_abuse_user_report(&self) -> resolve_abuse_user_report::Call<'a> {
        resolve_abuse_user_report::Call::new(self.client)
    }
    #[doc = "`admin/forward-abuse-user-report`"]
    pub fn forward_abuse_user_report(&self) -> forward_abuse_user_report::Call<'a> {
        forward_abuse_user_report::Call::new(self.client)
    }
    #[doc = "`admin/update-abuse-user-report`"]
    pub fn update_abuse_user_report(&self) -> update_abuse_user_report::Call<'a> {
        update_abuse_user_report::Call::new(self.client)
    }
    #[doc = "`admin/send-email`"]
    pub fn send_email(&self) -> send_email::Call<'a> {
        send_email::Call::new(self.client)
    }
    #[doc = "`admin/server-info`"]
    pub fn server_info(&self) -> server_info::Call<'a> {
        server_info::Call::new(self.client)
    }
    #[doc = "`admin/show-moderation-logs`"]
    pub fn show_moderation_logs(&self) -> show_moderation_logs::Call<'a> {
        show_moderation_logs::Call::new(self.client)
    }
    #[doc = "`admin/show-user`"]
    pub fn show_user(&self) -> show_user::Call<'a> {
        show_user::Call::new(self.client)
    }
    #[doc = "`admin/show-users`"]
    pub fn show_users(&self) -> show_users::Call<'a> {
        show_users::Call::new(self.client)
    }
    #[doc = "`admin/suspend-user`"]
    pub fn suspend_user(&self) -> suspend_user::Call<'a> {
        suspend_user::Call::new(self.client)
    }
    #[doc = "`admin/unsuspend-user`"]
    pub fn unsuspend_user(&self) -> unsuspend_user::Call<'a> {
        unsuspend_user::Call::new(self.client)
    }
    #[doc = "`admin/update-meta`"]
    pub fn update_meta(&self) -> update_meta::Call<'a> {
        update_meta::Call::new(self.client)
    }
    #[doc = "`admin/delete-account`"]
    pub fn delete_account(&self) -> delete_account::Call<'a> {
        delete_account::Call::new(self.client)
    }
    #[doc = "`admin/update-user-note`"]
    pub fn update_user_note(&self) -> update_user_note::Call<'a> {
        update_user_note::Call::new(self.client)
    }
    #[doc = "`admin/roles/` 以下のエンドポイント"]
    pub fn roles(&self) -> roles::Call<'a> {
        roles::Call::new(self.client)
    }
    #[doc = "`admin/system-webhook/` 以下のエンドポイント"]
    pub fn system_webhook(&self) -> system_webhook::Call<'a> {
        system_webhook::Call::new(self.client)
    }
}
#[doc = "`admin/meta`"]
pub mod meta {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "cacheRemoteFiles")]
        pub cache_remote_files: bool,
        #[serde(rename = "cacheRemoteSensitiveFiles")]
        pub cache_remote_sensitive_files: bool,
        #[serde(rename = "emailRequiredForSignup")]
        pub email_required_for_signup: bool,
        #[serde(rename = "enableHcaptcha")]
        pub enable_hcaptcha: bool,
        #[serde(rename = "hcaptchaSiteKey")]
        pub hcaptcha_site_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableMcaptcha")]
        pub enable_mcaptcha: bool,
        #[serde(rename = "mcaptchaSiteKey")]
        pub mcaptcha_site_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "mcaptchaInstanceUrl")]
        pub mcaptcha_instance_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableRecaptcha")]
        pub enable_recaptcha: bool,
        #[serde(rename = "recaptchaSiteKey")]
        pub recaptcha_site_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableTurnstile")]
        pub enable_turnstile: bool,
        #[serde(rename = "turnstileSiteKey")]
        pub turnstile_site_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableTestcaptcha")]
        pub enable_testcaptcha: bool,
        #[serde(rename = "swPublickey")]
        pub sw_publickey: ::std::option::Option<::std::string::String>,
        #[serde(rename = "mascotImageUrl")]
        pub mascot_image_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "bannerUrl")]
        pub banner_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "serverErrorImageUrl")]
        pub server_error_image_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "infoImageUrl")]
        pub info_image_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "notFoundImageUrl")]
        pub not_found_image_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "iconUrl")]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "app192IconUrl")]
        pub app192_icon_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "app512IconUrl")]
        pub app512_icon_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableEmail")]
        pub enable_email: bool,
        #[serde(rename = "enableServiceWorker")]
        pub enable_service_worker: bool,
        #[serde(rename = "translatorAvailable")]
        pub translator_available: bool,
        #[serde(rename = "silencedHosts", default)]
        pub silenced_hosts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(rename = "mediaSilencedHosts")]
        pub media_silenced_hosts: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "pinnedUsers")]
        pub pinned_users: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "hiddenTags")]
        pub hidden_tags: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "blockedHosts")]
        pub blocked_hosts: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "sensitiveWords")]
        pub sensitive_words: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "prohibitedWords")]
        pub prohibited_words: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "prohibitedWordsForNameOfUser")]
        pub prohibited_words_for_name_of_user: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "bannedEmailDomains", default)]
        pub banned_email_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(rename = "preservedUsernames")]
        pub preserved_usernames: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "hcaptchaSecretKey")]
        pub hcaptcha_secret_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "mcaptchaSecretKey")]
        pub mcaptcha_secret_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "recaptchaSecretKey")]
        pub recaptcha_secret_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "turnstileSecretKey")]
        pub turnstile_secret_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "sensitiveMediaDetection")]
        pub sensitive_media_detection: ::std::string::String,
        #[serde(rename = "sensitiveMediaDetectionSensitivity")]
        pub sensitive_media_detection_sensitivity: ::std::string::String,
        #[serde(rename = "setSensitiveFlagAutomatically")]
        pub set_sensitive_flag_automatically: bool,
        #[serde(rename = "enableSensitiveMediaDetectionForVideos")]
        pub enable_sensitive_media_detection_for_videos: bool,
        #[serde(rename = "proxyAccountId")]
        pub proxy_account_id: ::std::option::Option<::std::string::String>,
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(rename = "smtpSecure")]
        pub smtp_secure: bool,
        #[serde(rename = "smtpHost")]
        pub smtp_host: ::std::option::Option<::std::string::String>,
        #[serde(rename = "smtpPort")]
        pub smtp_port: ::std::option::Option<f64>,
        #[serde(rename = "smtpUser")]
        pub smtp_user: ::std::option::Option<::std::string::String>,
        #[serde(rename = "smtpPass")]
        pub smtp_pass: ::std::option::Option<::std::string::String>,
        #[serde(rename = "swPrivateKey")]
        pub sw_private_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "useObjectStorage")]
        pub use_object_storage: bool,
        #[serde(rename = "objectStorageBaseUrl")]
        pub object_storage_base_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageBucket")]
        pub object_storage_bucket: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStoragePrefix")]
        pub object_storage_prefix: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageEndpoint")]
        pub object_storage_endpoint: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageRegion")]
        pub object_storage_region: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStoragePort")]
        pub object_storage_port: ::std::option::Option<f64>,
        #[serde(rename = "objectStorageAccessKey")]
        pub object_storage_access_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageSecretKey")]
        pub object_storage_secret_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageUseSSL")]
        pub object_storage_use_ssl: bool,
        #[serde(rename = "objectStorageUseProxy")]
        pub object_storage_use_proxy: bool,
        #[serde(rename = "objectStorageSetPublicRead")]
        pub object_storage_set_public_read: bool,
        #[serde(rename = "enableIpLogging")]
        pub enable_ip_logging: bool,
        #[serde(rename = "enableActiveEmailValidation")]
        pub enable_active_email_validation: bool,
        #[serde(rename = "enableVerifymailApi")]
        pub enable_verifymail_api: bool,
        #[serde(rename = "verifymailAuthKey")]
        pub verifymail_auth_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableTruemailApi")]
        pub enable_truemail_api: bool,
        #[serde(rename = "truemailInstance")]
        pub truemail_instance: ::std::option::Option<::std::string::String>,
        #[serde(rename = "truemailAuthKey")]
        pub truemail_auth_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "enableChartsForRemoteUser")]
        pub enable_charts_for_remote_user: bool,
        #[serde(rename = "enableChartsForFederatedInstances")]
        pub enable_charts_for_federated_instances: bool,
        #[serde(rename = "enableStatsForFederatedInstances")]
        pub enable_stats_for_federated_instances: bool,
        #[serde(rename = "enableServerMachineStats")]
        pub enable_server_machine_stats: bool,
        #[serde(rename = "enableIdenticonGeneration")]
        pub enable_identicon_generation: bool,
        #[serde(rename = "manifestJsonOverride")]
        pub manifest_json_override: ::std::string::String,
        pub policies: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "enableFanoutTimeline")]
        pub enable_fanout_timeline: bool,
        #[serde(rename = "enableFanoutTimelineDbFallback")]
        pub enable_fanout_timeline_db_fallback: bool,
        #[serde(rename = "perLocalUserUserTimelineCacheMax")]
        pub per_local_user_user_timeline_cache_max: f64,
        #[serde(rename = "perRemoteUserUserTimelineCacheMax")]
        pub per_remote_user_user_timeline_cache_max: f64,
        #[serde(rename = "perUserHomeTimelineCacheMax")]
        pub per_user_home_timeline_cache_max: f64,
        #[serde(rename = "perUserListTimelineCacheMax")]
        pub per_user_list_timeline_cache_max: f64,
        #[serde(rename = "enableReactionsBuffering")]
        pub enable_reactions_buffering: bool,
        #[serde(rename = "notesPerOneAd")]
        pub notes_per_one_ad: f64,
        #[serde(rename = "backgroundImageUrl")]
        pub background_image_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "deeplAuthKey")]
        pub deepl_auth_key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "deeplIsPro")]
        pub deepl_is_pro: bool,
        #[serde(rename = "defaultDarkTheme")]
        pub default_dark_theme: ::std::option::Option<::std::string::String>,
        #[serde(rename = "defaultLightTheme")]
        pub default_light_theme: ::std::option::Option<::std::string::String>,
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(rename = "disableRegistration")]
        pub disable_registration: bool,
        #[serde(rename = "impressumUrl")]
        pub impressum_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "maintainerEmail")]
        pub maintainer_email: ::std::option::Option<::std::string::String>,
        #[serde(rename = "maintainerName")]
        pub maintainer_name: ::std::option::Option<::std::string::String>,
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(rename = "shortName")]
        pub short_name: ::std::option::Option<::std::string::String>,
        #[serde(rename = "objectStorageS3ForcePathStyle")]
        pub object_storage_s3_force_path_style: bool,
        #[serde(rename = "privacyPolicyUrl")]
        pub privacy_policy_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "inquiryUrl")]
        pub inquiry_url: ::std::option::Option<::std::string::String>,
        #[serde(rename = "repositoryUrl")]
        pub repository_url: ::std::option::Option<::std::string::String>,
        #[doc = "\\[Deprecated\\] Use \"urlPreviewSummaryProxyUrl\" instead."]
        #[serde(rename = "summalyProxy")]
        pub summaly_proxy: ::std::option::Option<::std::string::String>,
        #[serde(rename = "themeColor")]
        pub theme_color: ::std::option::Option<::std::string::String>,
        #[serde(rename = "tosUrl")]
        pub tos_url: ::std::option::Option<::std::string::String>,
        pub uri: ::std::string::String,
        pub version: ::std::string::String,
        #[serde(rename = "urlPreviewEnabled")]
        pub url_preview_enabled: bool,
        #[serde(rename = "urlPreviewTimeout")]
        pub url_preview_timeout: f64,
        #[serde(rename = "urlPreviewMaximumContentLength")]
        pub url_preview_maximum_content_length: f64,
        #[serde(rename = "urlPreviewRequireContentLength")]
        pub url_preview_require_content_length: bool,
        #[serde(rename = "urlPreviewUserAgent")]
        pub url_preview_user_agent: ::std::option::Option<::std::string::String>,
        #[serde(rename = "urlPreviewSummaryProxyUrl")]
        pub url_preview_summary_proxy_url: ::std::option::Option<::std::string::String>,
        pub federation: ::std::string::String,
        #[serde(rename = "federationHosts")]
        pub federation_hosts: ::std::vec::Vec<::std::string::String>,
    }
    #[doc = "`admin/meta`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:meta*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/meta";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`admin/meta` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:meta*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/abuse-user-reports`"]
pub mod abuse_user_reports {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestReporterOrigin {
        #[serde(rename = "combined")]
        Combined,
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestTargetUserOrigin {
        #[serde(rename = "combined")]
        Combined,
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseItemResolvedAs {
        #[serde(rename = "accept")]
        Accept,
        #[serde(rename = "reject")]
        Reject,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::Utc>,
        pub comment: ::std::string::String,
        pub resolved: bool,
        #[serde(rename = "reporterId")]
        pub reporter_id: ::std::string::String,
        #[serde(rename = "targetUserId")]
        pub target_user_id: ::std::string::String,
        #[serde(rename = "assigneeId")]
        pub assignee_id: ::std::option::Option<::std::string::String>,
        pub reporter: crate::generated::v2024_10_1::types::UserDetailedNotMe,
        #[serde(rename = "targetUser")]
        pub target_user: crate::generated::v2024_10_1::types::UserDetailedNotMe,
        pub assignee: ::std::option::Option<crate::generated::v2024_10_1::types::UserDetailedNotMe>,
        pub forwarded: bool,
        #[serde(rename = "resolvedAs")]
        pub resolved_as: ::std::option::Option<ResponseItemResolvedAs>,
        #[serde(rename = "moderationNote")]
        pub moderation_note: ::std::string::String,
    }
    #[doc = "`admin/abuse-user-reports`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-user-reports*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub state: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "reporterOrigin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reporter_origin: ::std::option::Option<RequestReporterOrigin>,
        #[serde(
            rename = "targetUserOrigin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub target_user_origin: ::std::option::Option<RequestTargetUserOrigin>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/abuse-user-reports";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`admin/abuse-user-reports` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-user-reports*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        state: ::std::option::Option<::std::option::Option<::std::string::String>>,
        reporter_origin: ::std::option::Option<RequestReporterOrigin>,
        target_user_origin: ::std::option::Option<RequestTargetUserOrigin>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                state: ::std::option::Option::None,
                reporter_origin: ::std::option::Option::None,
                target_user_origin: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn state(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.state = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`state` を `null` として送る。"]
        pub fn state_null(mut self) -> Self {
            self.state = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn reporter_origin(
            mut self,
            value: impl ::std::convert::Into<RequestReporterOrigin>,
        ) -> Self {
            self.reporter_origin = ::std::option::Option::Some(value.into());
            self
        }
        pub fn target_user_origin(
            mut self,
            value: impl ::std::convert::Into<RequestTargetUserOrigin>,
        ) -> Self {
            self.target_user_origin = ::std::option::Option::Some(value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                state: self.state,
                reporter_origin: self.reporter_origin,
                target_user_origin: self.target_user_origin,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/abuse-report/` 以下のエンドポイント"]
pub mod abuse_report {
    #[doc = "`admin/abuse-report/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/abuse-report/notification-recipient/` 以下のエンドポイント"]
        pub fn notification_recipient(&self) -> notification_recipient::Call<'a> {
            notification_recipient::Call::new(self.client)
        }
    }
    #[doc = "`admin/abuse-report/notification-recipient/` 以下のエンドポイント"]
    pub mod notification_recipient {
        #[doc = "`admin/abuse-report/notification-recipient/` 以下のエンドポイント"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {
            #[doc = "`admin/abuse-report/notification-recipient/list`"]
            pub fn list(&self) -> list::Call<'a> {
                list::Call::new(self.client)
            }
            #[doc = "`admin/abuse-report/notification-recipient/show`"]
            pub fn show(&self) -> show::Call<'a> {
                show::Call::new(self.client)
            }
            #[doc = "`admin/abuse-report/notification-recipient/create`"]
            pub fn create(&self) -> create::Call<'a> {
                create::Call::new(self.client)
            }
            #[doc = "`admin/abuse-report/notification-recipient/update`"]
            pub fn update(&self) -> update::Call<'a> {
                update::Call::new(self.client)
            }
            #[doc = "`admin/abuse-report/notification-recipient/delete`"]
            pub fn delete(&self) -> delete::Call<'a> {
                delete::Call::new(self.client)
            }
        }
        #[doc = "`admin/abuse-report/notification-recipient/list`"]
        pub mod list {
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub enum RequestMethodItem {
                #[serde(rename = "email")]
                Email,
                #[serde(rename = "webhook")]
                Webhook,
                #[doc = "このバージョンのバインディングが知らない値。"]
                #[serde(untagged)]
                Unrecognized(::std::string::String),
            }
            #[doc = "`admin/abuse-report/notification-recipient/list`"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub method: ::std::option::Option<::std::vec::Vec<RequestMethodItem>>,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "admin/abuse-report/notification-recipient/list";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response = ::std::vec::Vec<
                crate::generated::v2024_10_1::types::AbuseReportNotificationRecipient,
            >;
            #[doc = "`admin/abuse-report/notification-recipient/list` のビルダー。"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*"]
            pub struct Call<'a> {
                client: &'a crate::generated::v2024_10_1::Client,
                method: ::std::option::Option<::std::vec::Vec<RequestMethodItem>>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                    Call {
                        client,
                        method: ::std::option::Option::None,
                    }
                }
            }
            impl<'a> Call<'a> {
                pub fn method(
                    mut self,
                    value: impl ::std::convert::Into<::std::vec::Vec<RequestMethodItem>>,
                ) -> Self {
                    self.method = ::std::option::Option::Some(value.into());
                    self
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        method: self.method,
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`admin/abuse-report/notification-recipient/show`"]
        pub mod show {
            #[doc = "`admin/abuse-report/notification-recipient/show`"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                pub id: ::std::string::String,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "admin/abuse-report/notification-recipient/show";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response =
                crate::generated::v2024_10_1::types::AbuseReportNotificationRecipient;
            #[doc = "`admin/abuse-report/notification-recipient/show` のビルダー。"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<'a, S = (crate::builder::Unset,)> {
                client: &'a crate::generated::v2024_10_1::Client,
                id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                    Call {
                        client,
                        id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0> Call<'a, (S0,)> {
                pub fn id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set,)> {
                    Call {
                        client: self.client,
                        id: ::std::option::Option::Some(value.into()),
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        id: self
                            .id
                            .expect("required parameter is set by the type state"),
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`admin/abuse-report/notification-recipient/create`"]
        pub mod create {
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub enum RequestMethod {
                #[serde(rename = "email")]
                Email,
                #[serde(rename = "webhook")]
                Webhook,
                #[doc = "このバージョンのバインディングが知らない値。"]
                #[serde(untagged)]
                Unrecognized(::std::string::String),
            }
            #[doc = "`admin/abuse-report/notification-recipient/create`"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub struct Request {
                #[serde(rename = "isActive")]
                pub is_active: bool,
                pub name: ::std::string::String,
                pub method: RequestMethod,
                #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
                pub user_id: ::std::option::Option<::std::string::String>,
                #[serde(
                    rename = "systemWebhookId",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub system_webhook_id: ::std::option::Option<::std::string::String>,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "admin/abuse-report/notification-recipient/create";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response =
                crate::generated::v2024_10_1::types::AbuseReportNotificationRecipient;
            #[doc = "`admin/abuse-report/notification-recipient/create` のビルダー。"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `is_active`、`name`、`method`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<
                'a,
                S = (
                    crate::builder::Unset,
                    crate::builder::Unset,
                    crate::builder::Unset,
                ),
            > {
                client: &'a crate::generated::v2024_10_1::Client,
                is_active: ::std::option::Option<bool>,
                name: ::std::option::Option<::std::string::String>,
                method: ::std::option::Option<RequestMethod>,
                user_id: ::std::option::Option<::std::string::String>,
                system_webhook_id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                    Call {
                        client,
                        is_active: ::std::option::Option::None,
                        name: ::std::option::Option::None,
                        method: ::std::option::Option::None,
                        user_id: ::std::option::Option::None,
                        system_webhook_id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
                pub fn user_id(
                    mut self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
                    self.user_id = ::std::option::Option::Some(value.into());
                    self
                }
                pub fn system_webhook_id(
                    mut self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
                    self.system_webhook_id = ::std::option::Option::Some(value.into());
                    self
                }
                pub fn is_active(
                    self,
                    value: impl ::std::convert::Into<bool>,
                ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                    Call {
                        client: self.client,
                        is_active: ::std::option::Option::Some(value.into()),
                        name: self.name,
                        method: self.method,
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn name(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                    Call {
                        client: self.client,
                        is_active: self.is_active,
                        name: ::std::option::Option::Some(value.into()),
                        method: self.method,
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn method(
                    self,
                    value: impl ::std::convert::Into<RequestMethod>,
                ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                    Call {
                        client: self.client,
                        is_active: self.is_active,
                        name: self.name,
                        method: ::std::option::Option::Some(value.into()),
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture
                for Call<
                    'a,
                    (
                        crate::builder::Set,
                        crate::builder::Set,
                        crate::builder::Set,
                    ),
                >
            {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        is_active: self
                            .is_active
                            .expect("required parameter is set by the type state"),
                        name: self
                            .name
                            .expect("required parameter is set by the type state"),
                        method: self
                            .method
                            .expect("required parameter is set by the type state"),
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`admin/abuse-report/notification-recipient/update`"]
        pub mod update {
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub enum RequestMethod {
                #[serde(rename = "email")]
                Email,
                #[serde(rename = "webhook")]
                Webhook,
                #[doc = "このバージョンのバインディングが知らない値。"]
                #[serde(untagged)]
                Unrecognized(::std::string::String),
            }
            #[doc = "`admin/abuse-report/notification-recipient/update`"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub struct Request {
                pub id: ::std::string::String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                pub name: ::std::string::String,
                pub method: RequestMethod,
                #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
                pub user_id: ::std::option::Option<::std::string::String>,
                #[serde(
                    rename = "systemWebhookId",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub system_webhook_id: ::std::option::Option<::std::string::String>,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "admin/abuse-report/notification-recipient/update";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response =
                crate::generated::v2024_10_1::types::AbuseReportNotificationRecipient;
            #[doc = "`admin/abuse-report/notification-recipient/update` のビルダー。"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `id`、`is_active`、`name`、`method`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<
                'a,
                S = (
                    crate::builder::Unset,
                    crate::builder::Unset,
                    crate::builder::Unset,
                    crate::builder::Unset,
                ),
            > {
                client: &'a crate::generated::v2024_10_1::Client,
                id: ::std::option::Option<::std::string::String>,
                is_active: ::std::option::Option<bool>,
                name: ::std::option::Option<::std::string::String>,
                method: ::std::option::Option<RequestMethod>,
                user_id: ::std::option::Option<::std::string::String>,
                system_webhook_id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                    Call {
                        client,
                        id: ::std::option::Option::None,
                        is_active: ::std::option::Option::None,
                        name: ::std::option::Option::None,
                        method: ::std::option::Option::None,
                        user_id: ::std::option::Option::None,
                        system_webhook_id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0, S1, S2, S3> Call<'a, (S0, S1, S2, S3)> {
                pub fn user_id(
                    mut self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
                    self.user_id = ::std::option::Option::Some(value.into());
                    self
                }
                pub fn system_webhook_id(
                    mut self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
                    self.system_webhook_id = ::std::option::Option::Some(value.into());
                    self
                }
                pub fn id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set, S1, S2, S3)> {
                    Call {
                        client: self.client,
                        id: ::std::option::Option::Some(value.into()),
                        is_active: self.is_active,
                        name: self.name,
                        method: self.method,
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn is_active(
                    self,
                    value: impl ::std::convert::Into<bool>,
                ) -> Call<'a, (S0, crate::builder::Set, S2, S3)> {
                    Call {
                        client: self.client,
                        id: self.id,
                        is_active: ::std::option::Option::Some(value.into()),
                        name: self.name,
                        method: self.method,
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn name(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (S0, S1, crate::builder::Set, S3)> {
                    Call {
                        client: self.client,
                        id: self.id,
                        is_active: self.is_active,
                        name: ::std::option::Option::Some(value.into()),
                        method: self.method,
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn method(
                    self,
                    value: impl ::std::convert::Into<RequestMethod>,
                ) -> Call<'a, (S0, S1, S2, crate::builder::Set)> {
                    Call {
                        client: self.client,
                        id: self.id,
                        is_active: self.is_active,
                        name: self.name,
                        method: ::std::option::Option::Some(value.into()),
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture
                for Call<
                    'a,
                    (
                        crate::builder::Set,
                        crate::builder::Set,
                        crate::builder::Set,
                        crate::builder::Set,
                    ),
                >
            {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        id: self
                            .id
                            .expect("required parameter is set by the type state"),
                        is_active: self
                            .is_active
                            .expect("required parameter is set by the type state"),
                        name: self
                            .name
                            .expect("required parameter is set by the type state"),
                        method: self
                            .method
                            .expect("required parameter is set by the type state"),
                        user_id: self.user_id,
                        system_webhook_id: self.system_webhook_id,
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`admin/abuse-report/notification-recipient/delete`"]
        pub mod delete {
            #[doc = "`admin/abuse-report/notification-recipient/delete`"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                pub id: ::std::string::String,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "admin/abuse-report/notification-recipient/delete";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response = ();
            #[doc = "`admin/abuse-report/notification-recipient/delete` のビルダー。"]
            #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<'a, S = (crate::builder::Unset,)> {
                client: &'a crate::generated::v2024_10_1::Client,
                id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                    Call {
                        client,
                        id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0> Call<'a, (S0,)> {
                pub fn id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set,)> {
                    Call {
                        client: self.client,
                        id: ::std::option::Option::Some(value.into()),
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        id: self
                            .id
                            .expect("required parameter is set by the type state"),
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
    }
}
#[doc = "`admin/accounts/` 以下のエンドポイント"]
pub mod accounts {
    #[doc = "`admin/accounts/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/accounts/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/accounts/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/accounts/find-by-email`"]
        pub fn find_by_email(&self) -> find_by_email::Call<'a> {
            find_by_email::Call::new(self.client)
        }
    }
    #[doc = "`admin/accounts/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            pub name: ::std::option::Option<::std::string::String>,
            pub username: ::std::string::String,
            #[doc = "The local host is represented with `null`."]
            pub host: ::std::option::Option<::std::string::String>,
            #[serde(rename = "avatarUrl")]
            pub avatar_url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "avatarBlurhash")]
            pub avatar_blurhash: ::std::option::Option<::std::string::String>,
            #[serde(rename = "avatarDecorations")]
            pub avatar_decorations:
                ::std::vec::Vec<crate::generated::v2024_10_1::types::UserLiteAvatarDecorationsItem>,
            #[serde(rename = "isBot", default)]
            pub is_bot: ::std::option::Option<bool>,
            #[serde(rename = "isCat", default)]
            pub is_cat: ::std::option::Option<bool>,
            #[serde(default)]
            pub instance:
                ::std::option::Option<crate::generated::v2024_10_1::types::UserLiteInstance>,
            pub emojis: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            #[serde(rename = "onlineStatus")]
            pub online_status: crate::generated::v2024_10_1::types::UserLiteOnlineStatus,
            #[serde(rename = "badgeRoles", default)]
            pub badge_roles: ::std::option::Option<
                ::std::vec::Vec<crate::generated::v2024_10_1::types::UserLiteBadgeRolesItem>,
            >,
            pub url: ::std::option::Option<::std::string::String>,
            pub uri: ::std::option::Option<::std::string::String>,
            #[serde(rename = "movedTo")]
            pub moved_to: ::std::option::Option<::std::string::String>,
            #[serde(rename = "alsoKnownAs")]
            pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "updatedAt")]
            pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            #[serde(rename = "lastFetchedAt")]
            pub last_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            #[serde(rename = "bannerUrl")]
            pub banner_url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "bannerBlurhash")]
            pub banner_blurhash: ::std::option::Option<::std::string::String>,
            #[serde(rename = "isLocked")]
            pub is_locked: bool,
            #[serde(rename = "isSilenced")]
            pub is_silenced: bool,
            #[serde(rename = "isSuspended")]
            pub is_suspended: bool,
            pub description: ::std::option::Option<::std::string::String>,
            pub location: ::std::option::Option<::std::string::String>,
            pub birthday: ::std::option::Option<::std::string::String>,
            pub lang: ::std::option::Option<::std::string::String>,
            pub fields: ::std::vec::Vec<
                crate::generated::v2024_10_1::types::UserDetailedNotMeOnlyFieldsItem,
            >,
            #[serde(rename = "verifiedLinks")]
            pub verified_links: ::std::vec::Vec<::std::string::String>,
            #[serde(rename = "followersCount")]
            pub followers_count: f64,
            #[serde(rename = "followingCount")]
            pub following_count: f64,
            #[serde(rename = "notesCount")]
            pub notes_count: f64,
            #[serde(rename = "pinnedNoteIds")]
            pub pinned_note_ids: ::std::vec::Vec<::std::string::String>,
            #[serde(rename = "pinnedNotes")]
            pub pinned_notes: ::std::vec::Vec<crate::generated::v2024_10_1::types::Note>,
            #[serde(rename = "pinnedPageId")]
            pub pinned_page_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "pinnedPage")]
            pub pinned_page: ::std::option::Option<crate::generated::v2024_10_1::types::Page>,
            #[serde(rename = "publicReactions")]
            pub public_reactions: bool,
            #[serde(rename = "followingVisibility")]
            pub following_visibility:
                crate::generated::v2024_10_1::types::UserDetailedNotMeOnlyFollowingVisibility,
            #[serde(rename = "followersVisibility")]
            pub followers_visibility:
                crate::generated::v2024_10_1::types::UserDetailedNotMeOnlyFollowersVisibility,
            pub roles: ::std::vec::Vec<crate::generated::v2024_10_1::types::RoleLite>,
            #[serde(rename = "followedMessage")]
            pub followed_message: ::std::option::Option<::std::string::String>,
            pub memo: ::std::option::Option<::std::string::String>,
            #[serde(rename = "moderationNote", default)]
            pub moderation_note: ::std::option::Option<::std::string::String>,
            #[serde(rename = "twoFactorEnabled")]
            pub two_factor_enabled: bool,
            #[serde(rename = "usePasswordLessLogin")]
            pub use_password_less_login: bool,
            #[serde(rename = "securityKeys")]
            pub security_keys: bool,
            #[serde(rename = "isFollowing", default)]
            pub is_following: ::std::option::Option<bool>,
            #[serde(rename = "isFollowed", default)]
            pub is_followed: ::std::option::Option<bool>,
            #[serde(rename = "hasPendingFollowRequestFromYou", default)]
            pub has_pending_follow_request_from_you: ::std::option::Option<bool>,
            #[serde(rename = "hasPendingFollowRequestToYou", default)]
            pub has_pending_follow_request_to_you: ::std::option::Option<bool>,
            #[serde(rename = "isBlocking", default)]
            pub is_blocking: ::std::option::Option<bool>,
            #[serde(rename = "isBlocked", default)]
            pub is_blocked: ::std::option::Option<bool>,
            #[serde(rename = "isMuted", default)]
            pub is_muted: ::std::option::Option<bool>,
            #[serde(rename = "isRenoteMuted", default)]
            pub is_renote_muted: ::std::option::Option<bool>,
            #[serde(default)]
            pub notify: ::std::option::Option<
                crate::generated::v2024_10_1::types::UserDetailedNotMeOnlyNotify,
            >,
            #[serde(rename = "withReplies", default)]
            pub with_replies: ::std::option::Option<bool>,
            #[serde(rename = "avatarId")]
            pub avatar_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "bannerId")]
            pub banner_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "isModerator")]
            pub is_moderator: ::std::option::Option<bool>,
            #[serde(rename = "isAdmin")]
            pub is_admin: ::std::option::Option<bool>,
            #[serde(rename = "injectFeaturedNote")]
            pub inject_featured_note: bool,
            #[serde(rename = "receiveAnnouncementEmail")]
            pub receive_announcement_email: bool,
            #[serde(rename = "alwaysMarkNsfw")]
            pub always_mark_nsfw: bool,
            #[serde(rename = "autoSensitive")]
            pub auto_sensitive: bool,
            #[serde(rename = "carefulBot")]
            pub careful_bot: bool,
            #[serde(rename = "autoAcceptFollowed")]
            pub auto_accept_followed: bool,
            #[serde(rename = "noCrawle")]
            pub no_crawle: bool,
            #[serde(rename = "preventAiLearning")]
            pub prevent_ai_learning: bool,
            #[serde(rename = "isExplorable")]
            pub is_explorable: bool,
            #[serde(rename = "isDeleted")]
            pub is_deleted: bool,
            #[serde(rename = "twoFactorBackupCodesStock")]
            pub two_factor_backup_codes_stock:
                crate::generated::v2024_10_1::types::MeDetailedOnlyTwoFactorBackupCodesStock,
            #[serde(rename = "hideOnlineStatus")]
            pub hide_online_status: bool,
            #[serde(rename = "hasUnreadSpecifiedNotes")]
            pub has_unread_specified_notes: bool,
            #[serde(rename = "hasUnreadMentions")]
            pub has_unread_mentions: bool,
            #[serde(rename = "hasUnreadAnnouncement")]
            pub has_unread_announcement: bool,
            #[serde(rename = "unreadAnnouncements")]
            pub unread_announcements:
                ::std::vec::Vec<crate::generated::v2024_10_1::types::Announcement>,
            #[serde(rename = "hasUnreadAntenna")]
            pub has_unread_antenna: bool,
            #[serde(rename = "hasUnreadChannel")]
            pub has_unread_channel: bool,
            #[serde(rename = "hasUnreadNotification")]
            pub has_unread_notification: bool,
            #[serde(rename = "hasPendingReceivedFollowRequest")]
            pub has_pending_received_follow_request: bool,
            #[serde(rename = "unreadNotificationsCount")]
            pub unread_notifications_count: f64,
            #[serde(rename = "mutedWords")]
            pub muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
            #[serde(rename = "hardMutedWords")]
            pub hard_muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
            #[serde(rename = "mutedInstances")]
            pub muted_instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(rename = "notificationRecieveConfig")]
            pub notification_recieve_config:
                crate::generated::v2024_10_1::types::MeDetailedOnlyNotificationRecieveConfig,
            #[serde(rename = "emailNotificationTypes")]
            pub email_notification_types: ::std::vec::Vec<::std::string::String>,
            pub achievements: ::std::vec::Vec<
                crate::generated::v2024_10_1::types::MeDetailedOnlyAchievementsItem,
            >,
            #[serde(rename = "loggedInDays")]
            pub logged_in_days: f64,
            pub policies: crate::generated::v2024_10_1::types::RolePolicies,
            #[serde(default)]
            pub email: ::std::option::Option<::std::string::String>,
            #[serde(rename = "emailVerified", default)]
            pub email_verified: ::std::option::Option<bool>,
            #[serde(rename = "securityKeysList", default)]
            pub security_keys_list: ::std::option::Option<
                ::std::vec::Vec<
                    crate::generated::v2024_10_1::types::MeDetailedOnlySecurityKeysListItem,
                >,
            >,
            pub token: ::std::string::String,
        }
        #[doc = "`admin/accounts/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub username: ::std::string::String,
            pub password: ::std::string::String,
            #[serde(
                rename = "setupPassword",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub setup_password: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/accounts/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/accounts/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `username`、`password`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            username: ::std::option::Option<::std::string::String>,
            password: ::std::option::Option<::std::string::String>,
            setup_password: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    username: ::std::option::Option::None,
                    password: ::std::option::Option::None,
                    setup_password: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn setup_password(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.setup_password =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`setupPassword` を `null` として送る。"]
            pub fn setup_password_null(mut self) -> Self {
                self.setup_password = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn username(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    username: ::std::option::Option::Some(value.into()),
                    password: self.password,
                    setup_password: self.setup_password,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    username: self.username,
                    password: ::std::option::Option::Some(value.into()),
                    setup_password: self.setup_password,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    username: self
                        .username
                        .expect("required parameter is set by the type state"),
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    setup_password: self.setup_password,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/accounts/delete`"]
    pub mod delete {
        #[doc = "`admin/accounts/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/accounts/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/accounts/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/accounts/find-by-email`"]
    pub mod find_by_email {
        #[doc = "`admin/accounts/find-by-email`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub email: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/accounts/find-by-email";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::UserDetailedNotMe;
        #[doc = "`admin/accounts/find-by-email` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `email`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            email: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    email: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn email(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    email: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    email: self
                        .email
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/ad/` 以下のエンドポイント"]
pub mod ad {
    #[doc = "`admin/ad/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/ad/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/ad/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/ad/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/ad/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`admin/ad/create`"]
    pub mod create {
        #[doc = "`admin/ad/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub url: ::std::string::String,
            pub memo: ::std::string::String,
            pub place: ::std::string::String,
            pub priority: ::std::string::String,
            pub ratio: i64,
            #[serde(rename = "expiresAt")]
            pub expires_at: i64,
            #[serde(rename = "startsAt")]
            pub starts_at: i64,
            #[serde(rename = "imageUrl")]
            pub image_url: ::std::string::String,
            #[serde(rename = "dayOfWeek")]
            pub day_of_week: i64,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/ad/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::Ad;
        #[doc = "`admin/ad/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `url`、`memo`、`place`、`priority`、`ratio`、`expires_at`、`starts_at`、`image_url`、`day_of_week`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            url: ::std::option::Option<::std::string::String>,
            memo: ::std::option::Option<::std::string::String>,
            place: ::std::option::Option<::std::string::String>,
            priority: ::std::option::Option<::std::string::String>,
            ratio: ::std::option::Option<i64>,
            expires_at: ::std::option::Option<i64>,
            starts_at: ::std::option::Option<i64>,
            image_url: ::std::option::Option<::std::string::String>,
            day_of_week: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    url: ::std::option::Option::None,
                    memo: ::std::option::Option::None,
                    place: ::std::option::Option::None,
                    priority: ::std::option::Option::None,
                    ratio: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                    starts_at: ::std::option::Option::None,
                    image_url: ::std::option::Option::None,
                    day_of_week: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2, S3, S4, S5, S6, S7, S8> Call<'a, (S0, S1, S2, S3, S4, S5, S6, S7, S8)> {
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4, S5, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: ::std::option::Option::Some(value.into()),
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn memo(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4, S5, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: ::std::option::Option::Some(value.into()),
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn place(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4, S5, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: ::std::option::Option::Some(value.into()),
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn priority(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4, S5, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: ::std::option::Option::Some(value.into()),
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn ratio(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set, S5, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: ::std::option::Option::Some(value.into()),
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn expires_at(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, S1, S2, S3, S4, crate::builder::Set, S6, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: ::std::option::Option::Some(value.into()),
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn starts_at(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, S1, S2, S3, S4, S5, crate::builder::Set, S7, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: ::std::option::Option::Some(value.into()),
                    image_url: self.image_url,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn image_url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, S3, S4, S5, S6, crate::builder::Set, S8)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: ::std::option::Option::Some(value.into()),
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn day_of_week(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, S1, S2, S3, S4, S5, S6, S7, crate::builder::Set)> {
                Call {
                    client: self.client,
                    url: self.url,
                    memo: self.memo,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    image_url: self.image_url,
                    day_of_week: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    memo: self
                        .memo
                        .expect("required parameter is set by the type state"),
                    place: self
                        .place
                        .expect("required parameter is set by the type state"),
                    priority: self
                        .priority
                        .expect("required parameter is set by the type state"),
                    ratio: self
                        .ratio
                        .expect("required parameter is set by the type state"),
                    expires_at: self
                        .expires_at
                        .expect("required parameter is set by the type state"),
                    starts_at: self
                        .starts_at
                        .expect("required parameter is set by the type state"),
                    image_url: self
                        .image_url
                        .expect("required parameter is set by the type state"),
                    day_of_week: self
                        .day_of_week
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/ad/delete`"]
    pub mod delete {
        #[doc = "`admin/ad/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/ad/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/ad/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/ad/list`"]
    pub mod list {
        #[doc = "`admin/ad/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:ad*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub publishing: ::std::option::Option<::std::option::Option<bool>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/ad/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Ad>;
        #[doc = "`admin/ad/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:ad*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            publishing: ::std::option::Option<::std::option::Option<bool>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    publishing: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn publishing(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.publishing =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`publishing` を `null` として送る。"]
            pub fn publishing_null(mut self) -> Self {
                self.publishing = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    publishing: self.publishing,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/ad/update`"]
    pub mod update {
        #[doc = "`admin/ad/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub memo: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
            pub image_url: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub place: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub priority: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub ratio: ::std::option::Option<i64>,
            #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
            pub expires_at: ::std::option::Option<i64>,
            #[serde(rename = "startsAt", default, skip_serializing_if = "Option::is_none")]
            pub starts_at: ::std::option::Option<i64>,
            #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
            pub day_of_week: ::std::option::Option<i64>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/ad/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/ad/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:ad*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            memo: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
            image_url: ::std::option::Option<::std::string::String>,
            place: ::std::option::Option<::std::string::String>,
            priority: ::std::option::Option<::std::string::String>,
            ratio: ::std::option::Option<i64>,
            expires_at: ::std::option::Option<i64>,
            starts_at: ::std::option::Option<i64>,
            day_of_week: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    memo: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    image_url: ::std::option::Option::None,
                    place: ::std::option::Option::None,
                    priority: ::std::option::Option::None,
                    ratio: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                    starts_at: ::std::option::Option::None,
                    day_of_week: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn memo(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.memo = ::std::option::Option::Some(value.into());
                self
            }
            pub fn url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.url = ::std::option::Option::Some(value.into());
                self
            }
            pub fn image_url(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.image_url = ::std::option::Option::Some(value.into());
                self
            }
            pub fn place(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.place = ::std::option::Option::Some(value.into());
                self
            }
            pub fn priority(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.priority = ::std::option::Option::Some(value.into());
                self
            }
            pub fn ratio(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.ratio = ::std::option::Option::Some(value.into());
                self
            }
            pub fn expires_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.expires_at = ::std::option::Option::Some(value.into());
                self
            }
            pub fn starts_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.starts_at = ::std::option::Option::Some(value.into());
                self
            }
            pub fn day_of_week(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.day_of_week = ::std::option::Option::Some(value.into());
                self
            }
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    memo: self.memo,
                    url: self.url,
                    image_url: self.image_url,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    day_of_week: self.day_of_week,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                    memo: self.memo,
                    url: self.url,
                    image_url: self.image_url,
                    place: self.place,
                    priority: self.priority,
                    ratio: self.ratio,
                    expires_at: self.expires_at,
                    starts_at: self.starts_at,
                    day_of_week: self.day_of_week,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/announcements/` 以下のエンドポイント"]
pub mod announcements {
    #[doc = "`admin/announcements/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/announcements/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/announcements/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/announcements/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/announcements/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`admin/announcements/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestIcon {
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "success")]
            Success,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestDisplay {
            #[serde(rename = "normal")]
            Normal,
            #[serde(rename = "banner")]
            Banner,
            #[serde(rename = "dialog")]
            Dialog,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "updatedAt")]
            pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            pub title: ::std::string::String,
            pub text: ::std::string::String,
            #[serde(rename = "imageUrl")]
            pub image_url: ::std::option::Option<::std::string::String>,
        }
        #[doc = "`admin/announcements/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub title: ::std::string::String,
            pub text: ::std::string::String,
            #[serde(rename = "imageUrl")]
            pub image_url: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub icon: ::std::option::Option<RequestIcon>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub display: ::std::option::Option<RequestDisplay>,
            #[serde(
                rename = "forExistingUsers",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub for_existing_users: ::std::option::Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub silence: ::std::option::Option<bool>,
            #[serde(
                rename = "needConfirmationToRead",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub need_confirmation_to_read: ::std::option::Option<bool>,
            #[serde(
                rename = "userId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/announcements/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/announcements/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `title`、`text`、`image_url`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            title: ::std::option::Option<::std::string::String>,
            text: ::std::option::Option<::std::string::String>,
            image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            icon: ::std::option::Option<RequestIcon>,
            display: ::std::option::Option<RequestDisplay>,
            for_existing_users: ::std::option::Option<bool>,
            silence: ::std::option::Option<bool>,
            need_confirmation_to_read: ::std::option::Option<bool>,
            user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    title: ::std::option::Option::None,
                    text: ::std::option::Option::None,
                    image_url: ::std::option::Option::None,
                    icon: ::std::option::Option::None,
                    display: ::std::option::Option::None,
                    for_existing_users: ::std::option::Option::None,
                    silence: ::std::option::Option::None,
                    need_confirmation_to_read: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
            pub fn icon(mut self, value: impl ::std::convert::Into<RequestIcon>) -> Self {
                self.icon = ::std::option::Option::Some(value.into());
                self
            }
            pub fn display(mut self, value: impl ::std::convert::Into<RequestDisplay>) -> Self {
                self.display = ::std::option::Option::Some(value.into());
                self
            }
            pub fn for_existing_users(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.for_existing_users = ::std::option::Option::Some(value.into());
                self
            }
            pub fn silence(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.silence = ::std::option::Option::Some(value.into());
                self
            }
            pub fn need_confirmation_to_read(
                mut self,
                value: impl ::std::convert::Into<bool>,
            ) -> Self {
                self.need_confirmation_to_read = ::std::option::Option::Some(value.into());
                self
            }
            pub fn user_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.user_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`userId` を `null` として送る。"]
            pub fn user_id_null(mut self) -> Self {
                self.user_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn title(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                Call {
                    client: self.client,
                    title: ::std::option::Option::Some(value.into()),
                    text: self.text,
                    image_url: self.image_url,
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn text(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                Call {
                    client: self.client,
                    title: self.title,
                    text: ::std::option::Option::Some(value.into()),
                    image_url: self.image_url,
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn image_url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    title: self.title,
                    text: self.text,
                    image_url: ::std::option::Option::Some(::std::option::Option::Some(
                        value.into(),
                    )),
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            #[doc = "`imageUrl` を `null` として送る。"]
            pub fn image_url_null(self) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    title: self.title,
                    text: self.text,
                    image_url: ::std::option::Option::Some(::std::option::Option::None),
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    title: self
                        .title
                        .expect("required parameter is set by the type state"),
                    text: self
                        .text
                        .expect("required parameter is set by the type state"),
                    image_url: self
                        .image_url
                        .expect("required parameter is set by the type state"),
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    user_id: self.user_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/announcements/delete`"]
    pub mod delete {
        #[doc = "`admin/announcements/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/announcements/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/announcements/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/announcements/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestStatus {
            #[serde(rename = "all")]
            All,
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "archived")]
            Archived,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "updatedAt")]
            pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            pub text: ::std::string::String,
            pub title: ::std::string::String,
            #[serde(rename = "imageUrl")]
            pub image_url: ::std::option::Option<::std::string::String>,
            pub reads: f64,
        }
        #[doc = "`admin/announcements/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:announcements*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "userId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: ::std::option::Option<RequestStatus>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/announcements/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/announcements/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:announcements*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            status: ::std::option::Option<RequestStatus>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn user_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.user_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`userId` を `null` として送る。"]
            pub fn user_id_null(mut self) -> Self {
                self.user_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn status(mut self, value: impl ::std::convert::Into<RequestStatus>) -> Self {
                self.status = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    user_id: self.user_id,
                    status: self.status,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/announcements/update`"]
    pub mod update {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestIcon {
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "success")]
            Success,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestDisplay {
            #[serde(rename = "normal")]
            Normal,
            #[serde(rename = "banner")]
            Banner,
            #[serde(rename = "dialog")]
            Dialog,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/announcements/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub title: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub text: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "imageUrl",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub icon: ::std::option::Option<RequestIcon>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub display: ::std::option::Option<RequestDisplay>,
            #[serde(
                rename = "forExistingUsers",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub for_existing_users: ::std::option::Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub silence: ::std::option::Option<bool>,
            #[serde(
                rename = "needConfirmationToRead",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub need_confirmation_to_read: ::std::option::Option<bool>,
            #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
            pub is_active: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/announcements/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/announcements/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:announcements*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            title: ::std::option::Option<::std::string::String>,
            text: ::std::option::Option<::std::string::String>,
            image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            icon: ::std::option::Option<RequestIcon>,
            display: ::std::option::Option<RequestDisplay>,
            for_existing_users: ::std::option::Option<bool>,
            silence: ::std::option::Option<bool>,
            need_confirmation_to_read: ::std::option::Option<bool>,
            is_active: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    title: ::std::option::Option::None,
                    text: ::std::option::Option::None,
                    image_url: ::std::option::Option::None,
                    icon: ::std::option::Option::None,
                    display: ::std::option::Option::None,
                    for_existing_users: ::std::option::Option::None,
                    silence: ::std::option::Option::None,
                    need_confirmation_to_read: ::std::option::Option::None,
                    is_active: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn title(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.title = ::std::option::Option::Some(value.into());
                self
            }
            pub fn text(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.text = ::std::option::Option::Some(value.into());
                self
            }
            pub fn image_url(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.image_url =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`imageUrl` を `null` として送る。"]
            pub fn image_url_null(mut self) -> Self {
                self.image_url = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn icon(mut self, value: impl ::std::convert::Into<RequestIcon>) -> Self {
                self.icon = ::std::option::Option::Some(value.into());
                self
            }
            pub fn display(mut self, value: impl ::std::convert::Into<RequestDisplay>) -> Self {
                self.display = ::std::option::Option::Some(value.into());
                self
            }
            pub fn for_existing_users(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.for_existing_users = ::std::option::Option::Some(value.into());
                self
            }
            pub fn silence(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.silence = ::std::option::Option::Some(value.into());
                self
            }
            pub fn need_confirmation_to_read(
                mut self,
                value: impl ::std::convert::Into<bool>,
            ) -> Self {
                self.need_confirmation_to_read = ::std::option::Option::Some(value.into());
                self
            }
            pub fn is_active(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_active = ::std::option::Option::Some(value.into());
                self
            }
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    title: self.title,
                    text: self.text,
                    image_url: self.image_url,
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    is_active: self.is_active,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                    title: self.title,
                    text: self.text,
                    image_url: self.image_url,
                    icon: self.icon,
                    display: self.display,
                    for_existing_users: self.for_existing_users,
                    silence: self.silence,
                    need_confirmation_to_read: self.need_confirmation_to_read,
                    is_active: self.is_active,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/avatar-decorations/` 以下のエンドポイント"]
pub mod avatar_decorations {
    #[doc = "`admin/avatar-decorations/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/avatar-decorations/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/avatar-decorations/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/avatar-decorations/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/avatar-decorations/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`admin/avatar-decorations/create`"]
    pub mod create {
        #[doc = "`admin/avatar-decorations/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            pub description: ::std::string::String,
            pub url: ::std::string::String,
            #[serde(
                rename = "roleIdsThatCanBeUsedThisDecoration",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub role_ids_that_can_be_used_this_decoration:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/avatar-decorations/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/avatar-decorations/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`description`、`url`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
            role_ids_that_can_be_used_this_decoration:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    role_ids_that_can_be_used_this_decoration: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
            pub fn role_ids_that_can_be_used_this_decoration(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.role_ids_that_can_be_used_this_decoration =
                    ::std::option::Option::Some(value.into());
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    description: self.description,
                    url: self.url,
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                Call {
                    client: self.client,
                    name: self.name,
                    description: ::std::option::Option::Some(value.into()),
                    url: self.url,
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    url: ::std::option::Option::Some(value.into()),
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    description: self
                        .description
                        .expect("required parameter is set by the type state"),
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/avatar-decorations/delete`"]
    pub mod delete {
        #[doc = "`admin/avatar-decorations/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/avatar-decorations/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/avatar-decorations/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/avatar-decorations/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "updatedAt")]
            pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            pub name: ::std::string::String,
            pub description: ::std::string::String,
            pub url: ::std::string::String,
            #[serde(rename = "roleIdsThatCanBeUsedThisDecoration")]
            pub role_ids_that_can_be_used_this_decoration: ::std::vec::Vec<::std::string::String>,
        }
        #[doc = "`admin/avatar-decorations/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:avatar-decorations*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "userId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/avatar-decorations/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/avatar-decorations/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:avatar-decorations*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn user_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.user_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`userId` を `null` として送る。"]
            pub fn user_id_null(mut self) -> Self {
                self.user_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    user_id: self.user_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/avatar-decorations/update`"]
    pub mod update {
        #[doc = "`admin/avatar-decorations/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "roleIdsThatCanBeUsedThisDecoration",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub role_ids_that_can_be_used_this_decoration:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/avatar-decorations/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/avatar-decorations/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
            role_ids_that_can_be_used_this_decoration:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    role_ids_that_can_be_used_this_decoration: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn name(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.name = ::std::option::Option::Some(value.into());
                self
            }
            pub fn description(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.description = ::std::option::Option::Some(value.into());
                self
            }
            pub fn url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.url = ::std::option::Option::Some(value.into());
                self
            }
            pub fn role_ids_that_can_be_used_this_decoration(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.role_ids_that_can_be_used_this_decoration =
                    ::std::option::Option::Some(value.into());
                self
            }
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    description: self.description,
                    url: self.url,
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    description: self.description,
                    url: self.url,
                    role_ids_that_can_be_used_this_decoration: self
                        .role_ids_that_can_be_used_this_decoration,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/delete-all-files-of-a-user`"]
pub mod delete_all_files_of_a_user {
    #[doc = "`admin/delete-all-files-of-a-user`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:delete-all-files-of-a-user*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/delete-all-files-of-a-user";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/delete-all-files-of-a-user` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:delete-all-files-of-a-user*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/unset-user-avatar`"]
pub mod unset_user_avatar {
    #[doc = "`admin/unset-user-avatar`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unset-user-avatar*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/unset-user-avatar";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/unset-user-avatar` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unset-user-avatar*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/unset-user-banner`"]
pub mod unset_user_banner {
    #[doc = "`admin/unset-user-banner`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unset-user-banner*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/unset-user-banner";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/unset-user-banner` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unset-user-banner*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/drive/` 以下のエンドポイント"]
pub mod drive {
    #[doc = "`admin/drive/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/drive/clean-remote-files`"]
        pub fn clean_remote_files(&self) -> clean_remote_files::Call<'a> {
            clean_remote_files::Call::new(self.client)
        }
        #[doc = "`admin/drive/cleanup`"]
        pub fn cleanup(&self) -> cleanup::Call<'a> {
            cleanup::Call::new(self.client)
        }
        #[doc = "`admin/drive/files`"]
        pub fn files(&self) -> files::Call<'a> {
            files::Call::new(self.client)
        }
        #[doc = "`admin/drive/show-file`"]
        pub fn show_file(&self) -> show_file::Call<'a> {
            show_file::Call::new(self.client)
        }
    }
    #[doc = "`admin/drive/clean-remote-files`"]
    pub mod clean_remote_files {
        #[doc = "`admin/drive/clean-remote-files`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/drive/clean-remote-files";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/drive/clean-remote-files` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/drive/cleanup`"]
    pub mod cleanup {
        #[doc = "`admin/drive/cleanup`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/drive/cleanup";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/drive/cleanup` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/drive/files`"]
    pub mod files {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOrigin {
            #[serde(rename = "combined")]
            Combined,
            #[serde(rename = "local")]
            Local,
            #[serde(rename = "remote")]
            Remote,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/drive/files`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "userId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub origin: ::std::option::Option<RequestOrigin>,
            #[doc = "The local host is represented with `null`."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/drive/files";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::DriveFile>;
        #[doc = "`admin/drive/files` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
            origin: ::std::option::Option<RequestOrigin>,
            hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    r#type: ::std::option::Option::None,
                    origin: ::std::option::Option::None,
                    hostname: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn user_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.user_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`userId` を `null` として送る。"]
            pub fn user_id_null(mut self) -> Self {
                self.user_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn r#type(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.r#type =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`type` を `null` として送る。"]
            pub fn type_null(mut self) -> Self {
                self.r#type = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn origin(mut self, value: impl ::std::convert::Into<RequestOrigin>) -> Self {
                self.origin = ::std::option::Option::Some(value.into());
                self
            }
            #[doc = "The local host is represented with `null`."]
            pub fn hostname(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.hostname =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`hostname` を `null` として送る。"]
            pub fn hostname_null(mut self) -> Self {
                self.hostname = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    user_id: self.user_id,
                    r#type: self.r#type,
                    origin: self.origin,
                    hostname: self.hostname,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/drive/show-file`"]
    pub mod show_file {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseProperties {
            #[serde(default)]
            pub width: ::std::option::Option<f64>,
            #[serde(default)]
            pub height: ::std::option::Option<f64>,
            #[serde(default)]
            pub orientation: ::std::option::Option<f64>,
            #[serde(rename = "avgColor", default)]
            pub avg_color: ::std::option::Option<::std::string::String>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "userId")]
            pub user_id: ::std::option::Option<::std::string::String>,
            #[doc = "The local host is represented with `null`."]
            #[serde(rename = "userHost")]
            pub user_host: ::std::option::Option<::std::string::String>,
            pub md5: ::std::string::String,
            pub name: ::std::string::String,
            pub r#type: ::std::string::String,
            pub size: f64,
            pub comment: ::std::option::Option<::std::string::String>,
            pub blurhash: ::std::option::Option<::std::string::String>,
            pub properties: ResponseProperties,
            #[serde(rename = "storedInternal")]
            pub stored_internal: ::std::option::Option<bool>,
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "thumbnailUrl")]
            pub thumbnail_url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "webpublicUrl")]
            pub webpublic_url: ::std::option::Option<::std::string::String>,
            #[serde(rename = "accessKey")]
            pub access_key: ::std::option::Option<::std::string::String>,
            #[serde(rename = "thumbnailAccessKey")]
            pub thumbnail_access_key: ::std::option::Option<::std::string::String>,
            #[serde(rename = "webpublicAccessKey")]
            pub webpublic_access_key: ::std::option::Option<::std::string::String>,
            pub uri: ::std::option::Option<::std::string::String>,
            pub src: ::std::option::Option<::std::string::String>,
            #[serde(rename = "folderId")]
            pub folder_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "isSensitive")]
            pub is_sensitive: bool,
            #[serde(rename = "isLink")]
            pub is_link: bool,
        }
        #[doc = "`admin/drive/show-file`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:drive*"]
        #[doc = "必須: `fileId` / `url` のいずれか。"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
            pub file_id: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/drive/show-file";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/drive/show-file` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            file_id: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    file_id: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn file_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.file_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.url = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_id: self.file_id,
                    url: self.url,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/emoji/` 以下のエンドポイント"]
pub mod emoji {
    #[doc = "`admin/emoji/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/emoji/add-aliases-bulk`"]
        pub fn add_aliases_bulk(&self) -> add_aliases_bulk::Call<'a> {
            add_aliases_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/add`"]
        pub fn add(&self) -> add::Call<'a> {
            add::Call::new(self.client)
        }
        #[doc = "`admin/emoji/copy`"]
        pub fn copy(&self) -> copy::Call<'a> {
            copy::Call::new(self.client)
        }
        #[doc = "`admin/emoji/delete-bulk`"]
        pub fn delete_bulk(&self) -> delete_bulk::Call<'a> {
            delete_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/emoji/import-zip`"]
        pub fn import_zip(&self) -> import_zip::Call<'a> {
            import_zip::Call::new(self.client)
        }
        #[doc = "`admin/emoji/list-remote`"]
        pub fn list_remote(&self) -> list_remote::Call<'a> {
            list_remote::Call::new(self.client)
        }
        #[doc = "`admin/emoji/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/emoji/remove-aliases-bulk`"]
        pub fn remove_aliases_bulk(&self) -> remove_aliases_bulk::Call<'a> {
            remove_aliases_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/set-aliases-bulk`"]
        pub fn set_aliases_bulk(&self) -> set_aliases_bulk::Call<'a> {
            set_aliases_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/set-category-bulk`"]
        pub fn set_category_bulk(&self) -> set_category_bulk::Call<'a> {
            set_category_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/set-license-bulk`"]
        pub fn set_license_bulk(&self) -> set_license_bulk::Call<'a> {
            set_license_bulk::Call::new(self.client)
        }
        #[doc = "`admin/emoji/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`admin/emoji/add-aliases-bulk`"]
    pub mod add_aliases_bulk {
        #[doc = "`admin/emoji/add-aliases-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
            pub aliases: ::std::vec::Vec<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/add-aliases-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/add-aliases-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`、`aliases`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    aliases: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    aliases: self.aliases,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn aliases(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    ids: self.ids,
                    aliases: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                    aliases: self
                        .aliases
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/add`"]
    pub mod add {
        #[doc = "`admin/emoji/add`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
            #[doc = "Use `null` to reset the category."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub category: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub license: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
            #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
            pub local_only: ::std::option::Option<bool>,
            #[serde(
                rename = "roleIdsThatCanBeUsedThisEmojiAsReaction",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub role_ids_that_can_be_used_this_emoji_as_reaction:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/add";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::EmojiDetailed;
        #[doc = "`admin/emoji/add` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            name: ::std::option::Option<::std::string::String>,
            file_id: ::std::option::Option<::std::string::String>,
            category: ::std::option::Option<::std::option::Option<::std::string::String>>,
            aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            license: ::std::option::Option<::std::option::Option<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            local_only: ::std::option::Option<bool>,
            role_ids_that_can_be_used_this_emoji_as_reaction:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    category: ::std::option::Option::None,
                    aliases: ::std::option::Option::None,
                    license: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    local_only: ::std::option::Option::None,
                    role_ids_that_can_be_used_this_emoji_as_reaction: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "Use `null` to reset the category."]
            pub fn category(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.category =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`category` を `null` として送る。"]
            pub fn category_null(mut self) -> Self {
                self.category = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn aliases(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.aliases = ::std::option::Option::Some(value.into());
                self
            }
            pub fn license(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.license =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`license` を `null` として送る。"]
            pub fn license_null(mut self) -> Self {
                self.license = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn is_sensitive(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_sensitive = ::std::option::Option::Some(value.into());
                self
            }
            pub fn local_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.local_only = ::std::option::Option::Some(value.into());
                self
            }
            pub fn role_ids_that_can_be_used_this_emoji_as_reaction(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.role_ids_that_can_be_used_this_emoji_as_reaction =
                    ::std::option::Option::Some(value.into());
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    file_id: self.file_id,
                    category: self.category,
                    aliases: self.aliases,
                    license: self.license,
                    is_sensitive: self.is_sensitive,
                    local_only: self.local_only,
                    role_ids_that_can_be_used_this_emoji_as_reaction: self
                        .role_ids_that_can_be_used_this_emoji_as_reaction,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    name: self.name,
                    file_id: ::std::option::Option::Some(value.into()),
                    category: self.category,
                    aliases: self.aliases,
                    license: self.license,
                    is_sensitive: self.is_sensitive,
                    local_only: self.local_only,
                    role_ids_that_can_be_used_this_emoji_as_reaction: self
                        .role_ids_that_can_be_used_this_emoji_as_reaction,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                    category: self.category,
                    aliases: self.aliases,
                    license: self.license,
                    is_sensitive: self.is_sensitive,
                    local_only: self.local_only,
                    role_ids_that_can_be_used_this_emoji_as_reaction: self
                        .role_ids_that_can_be_used_this_emoji_as_reaction,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/copy`"]
    pub mod copy {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
        }
        #[doc = "`admin/emoji/copy`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "emojiId")]
            pub emoji_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/copy";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/emoji/copy` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `emoji_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            emoji_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    emoji_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn emoji_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    emoji_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    emoji_id: self
                        .emoji_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/delete-bulk`"]
    pub mod delete_bulk {
        #[doc = "`admin/emoji/delete-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/delete-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/delete-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/delete`"]
    pub mod delete {
        #[doc = "`admin/emoji/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/import-zip`"]
    pub mod import_zip {
        #[doc = "`admin/emoji/import-zip`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/import-zip";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/import-zip` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            file_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    file_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    file_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/list-remote`"]
    pub mod list_remote {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub aliases: ::std::vec::Vec<::std::string::String>,
            pub name: ::std::string::String,
            pub category: ::std::option::Option<::std::string::String>,
            #[doc = "The local host is represented with `null`."]
            pub host: ::std::option::Option<::std::string::String>,
            pub url: ::std::string::String,
        }
        #[doc = "`admin/emoji/list-remote`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub query: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[doc = "Use `null` to represent the local host."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/list-remote";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/emoji/list-remote` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            query: ::std::option::Option<::std::option::Option<::std::string::String>>,
            host: ::std::option::Option<::std::option::Option<::std::string::String>>,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    query: ::std::option::Option::None,
                    host: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn query(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.query = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`query` を `null` として送る。"]
            pub fn query_null(mut self) -> Self {
                self.query = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            #[doc = "Use `null` to represent the local host."]
            pub fn host(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.host = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`host` を `null` として送る。"]
            pub fn host_null(mut self) -> Self {
                self.host = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    query: self.query,
                    host: self.host,
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub aliases: ::std::vec::Vec<::std::string::String>,
            pub name: ::std::string::String,
            pub category: ::std::option::Option<::std::string::String>,
            #[doc = "The local host is represented with `null`. The field exists for compatibility with other API endpoints that return files."]
            pub host: ::std::option::Option<::std::string::String>,
            pub url: ::std::string::String,
        }
        #[doc = "`admin/emoji/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub query: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/emoji/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            query: ::std::option::Option<::std::option::Option<::std::string::String>>,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    query: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn query(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.query = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`query` を `null` として送る。"]
            pub fn query_null(mut self) -> Self {
                self.query = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    query: self.query,
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/remove-aliases-bulk`"]
    pub mod remove_aliases_bulk {
        #[doc = "`admin/emoji/remove-aliases-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
            pub aliases: ::std::vec::Vec<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/remove-aliases-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/remove-aliases-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`、`aliases`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    aliases: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    aliases: self.aliases,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn aliases(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    ids: self.ids,
                    aliases: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                    aliases: self
                        .aliases
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/set-aliases-bulk`"]
    pub mod set_aliases_bulk {
        #[doc = "`admin/emoji/set-aliases-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
            pub aliases: ::std::vec::Vec<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/set-aliases-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/set-aliases-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`、`aliases`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    aliases: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    aliases: self.aliases,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn aliases(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    ids: self.ids,
                    aliases: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                    aliases: self
                        .aliases
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/set-category-bulk`"]
    pub mod set_category_bulk {
        #[doc = "`admin/emoji/set-category-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
            #[doc = "Use `null` to reset the category."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub category: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/set-category-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/set-category-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            category: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    category: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "Use `null` to reset the category."]
            pub fn category(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.category =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`category` を `null` として送る。"]
            pub fn category_null(mut self) -> Self {
                self.category = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    category: self.category,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                    category: self.category,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/set-license-bulk`"]
    pub mod set_license_bulk {
        #[doc = "`admin/emoji/set-license-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub ids: ::std::vec::Vec<::std::string::String>,
            #[doc = "Use `null` to reset the license."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub license: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/set-license-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/set-license-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `ids`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            license: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    ids: ::std::option::Option::None,
                    license: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "Use `null` to reset the license."]
            pub fn license(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.license =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`license` を `null` として送る。"]
            pub fn license_null(mut self) -> Self {
                self.license = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    ids: ::std::option::Option::Some(value.into()),
                    license: self.license,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    ids: self
                        .ids
                        .expect("required parameter is set by the type state"),
                    license: self.license,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/emoji/update`"]
    pub mod update {
        #[doc = "`admin/emoji/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        #[doc = "必須: `id` / `name` のいずれか。"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub id: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
            pub file_id: ::std::option::Option<::std::string::String>,
            #[doc = "Use `null` to reset the category."]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub category: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub license: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
            #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
            pub local_only: ::std::option::Option<bool>,
            #[serde(
                rename = "roleIdsThatCanBeUsedThisEmojiAsReaction",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub role_ids_that_can_be_used_this_emoji_as_reaction:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/emoji/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/emoji/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:emoji*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            file_id: ::std::option::Option<::std::string::String>,
            category: ::std::option::Option<::std::option::Option<::std::string::String>>,
            aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            license: ::std::option::Option<::std::option::Option<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            local_only: ::std::option::Option<bool>,
            role_ids_that_can_be_used_this_emoji_as_reaction:
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    category: ::std::option::Option::None,
                    aliases: ::std::option::Option::None,
                    license: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    local_only: ::std::option::Option::None,
                    role_ids_that_can_be_used_this_emoji_as_reaction: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn name(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.name = ::std::option::Option::Some(value.into());
                self
            }
            pub fn file_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.file_id = ::std::option::Option::Some(value.into());
                self
            }
            #[doc = "Use `null` to reset the category."]
            pub fn category(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.category =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`category` を `null` として送る。"]
            pub fn category_null(mut self) -> Self {
                self.category = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn aliases(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.aliases = ::std::option::Option::Some(value.into());
                self
            }
            pub fn license(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.license =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`license` を `null` として送る。"]
            pub fn license_null(mut self) -> Self {
                self.license = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn is_sensitive(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_sensitive = ::std::option::Option::Some(value.into());
                self
            }
            pub fn local_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.local_only = ::std::option::Option::Some(value.into());
                self
            }
            pub fn role_ids_that_can_be_used_this_emoji_as_reaction(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.role_ids_that_can_be_used_this_emoji_as_reaction =
                    ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self.id,
                    name: self.name,
                    file_id: self.file_id,
                    category: self.category,
                    aliases: self.aliases,
                    license: self.license,
                    is_sensitive: self.is_sensitive,
                    local_only: self.local_only,
                    role_ids_that_can_be_used_this_emoji_as_reaction: self
                        .role_ids_that_can_be_used_this_emoji_as_reaction,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/federation/` 以下のエンドポイント"]
pub mod federation {
    #[doc = "`admin/federation/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/federation/delete-all-files`"]
        pub fn delete_all_files(&self) -> delete_all_files::Call<'a> {
            delete_all_files::Call::new(self.client)
        }
        #[doc = "`admin/federation/refresh-remote-instance-metadata`"]
        pub fn refresh_remote_instance_metadata(
            &self,
        ) -> refresh_remote_instance_metadata::Call<'a> {
            refresh_remote_instance_metadata::Call::new(self.client)
        }
        #[doc = "`admin/federation/remove-all-following`"]
        pub fn remove_all_following(&self) -> remove_all_following::Call<'a> {
            remove_all_following::Call::new(self.client)
        }
        #[doc = "`admin/federation/update-instance`"]
        pub fn update_instance(&self) -> update_instance::Call<'a> {
            update_instance::Call::new(self.client)
        }
    }
    #[doc = "`admin/federation/delete-all-files`"]
    pub mod delete_all_files {
        #[doc = "`admin/federation/delete-all-files`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub host: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/federation/delete-all-files";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/federation/delete-all-files` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            host: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    host: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn host(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    host: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    host: self
                        .host
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/federation/refresh-remote-instance-metadata`"]
    pub mod refresh_remote_instance_metadata {
        #[doc = "`admin/federation/refresh-remote-instance-metadata`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub host: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/federation/refresh-remote-instance-metadata";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/federation/refresh-remote-instance-metadata` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            host: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    host: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn host(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    host: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    host: self
                        .host
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/federation/remove-all-following`"]
    pub mod remove_all_following {
        #[doc = "`admin/federation/remove-all-following`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub host: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/federation/remove-all-following";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/federation/remove-all-following` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            host: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    host: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn host(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    host: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    host: self
                        .host
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/federation/update-instance`"]
    pub mod update_instance {
        #[doc = "`admin/federation/update-instance`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub host: ::std::string::String,
            #[serde(
                rename = "isSuspended",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_suspended: ::std::option::Option<bool>,
            #[serde(
                rename = "moderationNote",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub moderation_note: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/federation/update-instance";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/federation/update-instance` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:federation*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            host: ::std::option::Option<::std::string::String>,
            is_suspended: ::std::option::Option<bool>,
            moderation_note: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    host: ::std::option::Option::None,
                    is_suspended: ::std::option::Option::None,
                    moderation_note: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn is_suspended(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_suspended = ::std::option::Option::Some(value.into());
                self
            }
            pub fn moderation_note(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.moderation_note = ::std::option::Option::Some(value.into());
                self
            }
            pub fn host(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    host: ::std::option::Option::Some(value.into()),
                    is_suspended: self.is_suspended,
                    moderation_note: self.moderation_note,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    host: self
                        .host
                        .expect("required parameter is set by the type state"),
                    is_suspended: self.is_suspended,
                    moderation_note: self.moderation_note,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/get-index-stats`"]
pub mod get_index_stats {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub tablename: ::std::string::String,
        pub indexname: ::std::string::String,
    }
    #[doc = "`admin/get-index-stats`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:index-stats*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/get-index-stats";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`admin/get-index-stats` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:index-stats*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/get-table-stats`"]
pub mod get_table_stats {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseValue {
        pub count: f64,
        pub size: f64,
    }
    #[doc = "`admin/get-table-stats`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:table-stats*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/get-table-stats";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::collections::HashMap<::std::string::String, ResponseValue>;
    #[doc = "`admin/get-table-stats` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:table-stats*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/get-user-ips`"]
pub mod get_user_ips {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub ip: ::std::string::String,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::Utc>,
    }
    #[doc = "`admin/get-user-ips`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:user-ips*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/get-user-ips";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`admin/get-user-ips` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:user-ips*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/invite/` 以下のエンドポイント"]
pub mod invite {
    #[doc = "`admin/invite/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/invite/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/invite/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
    }
    #[doc = "`admin/invite/create`"]
    pub mod create {
        #[doc = "`admin/invite/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:invite-codes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub count: ::std::option::Option<i64>,
            #[serde(
                rename = "expiresAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expires_at: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/invite/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::InviteCode>;
        #[doc = "`admin/invite/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:invite-codes*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            count: ::std::option::Option<i64>,
            expires_at: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    count: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn count(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.count = ::std::option::Option::Some(value.into());
                self
            }
            pub fn expires_at(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.expires_at =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`expiresAt` を `null` として送る。"]
            pub fn expires_at_null(mut self) -> Self {
                self.expires_at = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    count: self.count,
                    expires_at: self.expires_at,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/invite/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestType {
            #[serde(rename = "unused")]
            Unused,
            #[serde(rename = "used")]
            Used,
            #[serde(rename = "expired")]
            Expired,
            #[serde(rename = "all")]
            All,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSort {
            #[serde(rename = "+createdAt")]
            PlusCreatedAt,
            #[serde(rename = "-createdAt")]
            MinusCreatedAt,
            #[serde(rename = "+usedAt")]
            PlusUsedAt,
            #[serde(rename = "-usedAt")]
            MinusUsedAt,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/invite/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:invite-codes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub offset: ::std::option::Option<i64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub r#type: ::std::option::Option<RequestType>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub sort: ::std::option::Option<RequestSort>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/invite/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::InviteCode>;
        #[doc = "`admin/invite/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:invite-codes*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<i64>,
            r#type: ::std::option::Option<RequestType>,
            sort: ::std::option::Option<RequestSort>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    r#type: ::std::option::Option::None,
                    sort: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.offset = ::std::option::Option::Some(value.into());
                self
            }
            pub fn r#type(mut self, value: impl ::std::convert::Into<RequestType>) -> Self {
                self.r#type = ::std::option::Option::Some(value.into());
                self
            }
            pub fn sort(mut self, value: impl ::std::convert::Into<RequestSort>) -> Self {
                self.sort = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    offset: self.offset,
                    r#type: self.r#type,
                    sort: self.sort,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/promo/` 以下のエンドポイント"]
pub mod promo {
    #[doc = "`admin/promo/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/promo/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
    }
    #[doc = "`admin/promo/create`"]
    pub mod create {
        #[doc = "`admin/promo/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:promo*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
            #[serde(rename = "expiresAt")]
            pub expires_at: i64,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/promo/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/promo/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:promo*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`、`expires_at`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            note_id: ::std::option::Option<::std::string::String>,
            expires_at: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    note_id: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn note_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    note_id: ::std::option::Option::Some(value.into()),
                    expires_at: self.expires_at,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn expires_at(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    note_id: self.note_id,
                    expires_at: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    note_id: self
                        .note_id
                        .expect("required parameter is set by the type state"),
                    expires_at: self
                        .expires_at
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/queue/` 以下のエンドポイント"]
pub mod queue {
    #[doc = "`admin/queue/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/queue/clear`"]
        pub fn clear(&self) -> clear::Call<'a> {
            clear::Call::new(self.client)
        }
        #[doc = "`admin/queue/deliver-delayed`"]
        pub fn deliver_delayed(&self) -> deliver_delayed::Call<'a> {
            deliver_delayed::Call::new(self.client)
        }
        #[doc = "`admin/queue/inbox-delayed`"]
        pub fn inbox_delayed(&self) -> inbox_delayed::Call<'a> {
            inbox_delayed::Call::new(self.client)
        }
        #[doc = "`admin/queue/promote`"]
        pub fn promote(&self) -> promote::Call<'a> {
            promote::Call::new(self.client)
        }
        #[doc = "`admin/queue/stats`"]
        pub fn stats(&self) -> stats::Call<'a> {
            stats::Call::new(self.client)
        }
    }
    #[doc = "`admin/queue/clear`"]
    pub mod clear {
        #[doc = "`admin/queue/clear`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:queue*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/queue/clear";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/queue/clear` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:queue*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/queue/deliver-delayed`"]
    pub mod deliver_delayed {
        #[doc = "`admin/queue/deliver-delayed`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:queue*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/queue/deliver-delayed";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<(::std::string::String, f64)>;
        #[doc = "`admin/queue/deliver-delayed` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:queue*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/queue/inbox-delayed`"]
    pub mod inbox_delayed {
        #[doc = "`admin/queue/inbox-delayed`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:queue*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/queue/inbox-delayed";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<(::std::string::String, f64)>;
        #[doc = "`admin/queue/inbox-delayed` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:queue*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/queue/promote`"]
    pub mod promote {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestType {
            #[serde(rename = "deliver")]
            Deliver,
            #[serde(rename = "inbox")]
            Inbox,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/queue/promote`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:queue*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub r#type: RequestType,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/queue/promote";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/queue/promote` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:queue*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `type`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            r#type: ::std::option::Option<RequestType>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    r#type: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn r#type(
                self,
                value: impl ::std::convert::Into<RequestType>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    r#type: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    r#type: self
                        .r#type
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/queue/stats`"]
    pub mod stats {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub deliver: crate::generated::v2024_10_1::types::QueueCount,
            pub inbox: crate::generated::v2024_10_1::types::QueueCount,
            pub db: crate::generated::v2024_10_1::types::QueueCount,
            #[serde(rename = "objectStorage")]
            pub object_storage: crate::generated::v2024_10_1::types::QueueCount,
        }
        #[doc = "`admin/queue/stats`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/queue/stats";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/queue/stats` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/relays/` 以下のエンドポイント"]
pub mod relays {
    #[doc = "`admin/relays/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/relays/add`"]
        pub fn add(&self) -> add::Call<'a> {
            add::Call::new(self.client)
        }
        #[doc = "`admin/relays/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/relays/remove`"]
        pub fn remove(&self) -> remove::Call<'a> {
            remove::Call::new(self.client)
        }
    }
    #[doc = "`admin/relays/add`"]
    pub mod add {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseStatus {
            #[serde(rename = "requesting")]
            Requesting,
            #[serde(rename = "accepted")]
            Accepted,
            #[serde(rename = "rejected")]
            Rejected,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            pub inbox: ::std::string::String,
            pub status: ResponseStatus,
        }
        #[doc = "`admin/relays/add`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:relays*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub inbox: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/relays/add";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`admin/relays/add` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:relays*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `inbox`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            inbox: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    inbox: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn inbox(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    inbox: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    inbox: self
                        .inbox
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/relays/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseItemStatus {
            #[serde(rename = "requesting")]
            Requesting,
            #[serde(rename = "accepted")]
            Accepted,
            #[serde(rename = "rejected")]
            Rejected,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub inbox: ::std::string::String,
            pub status: ResponseItemStatus,
        }
        #[doc = "`admin/relays/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:relays*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/relays/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/relays/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:relays*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/relays/remove`"]
    pub mod remove {
        #[doc = "`admin/relays/remove`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:relays*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub inbox: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/relays/remove";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/relays/remove` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:relays*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `inbox`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            inbox: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    inbox: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn inbox(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    inbox: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    inbox: self
                        .inbox
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/reset-password`"]
pub mod reset_password {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub password: ::std::string::String,
    }
    #[doc = "`admin/reset-password`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:reset-password*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/reset-password";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`admin/reset-password` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:reset-password*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/resolve-abuse-user-report`"]
pub mod resolve_abuse_user_report {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestResolvedAs {
        #[serde(rename = "accept")]
        Accept,
        #[serde(rename = "reject")]
        Reject,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`admin/resolve-abuse-user-report`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "reportId")]
        pub report_id: ::std::string::String,
        #[serde(
            rename = "resolvedAs",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub resolved_as: ::std::option::Option<::std::option::Option<RequestResolvedAs>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/resolve-abuse-user-report";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/resolve-abuse-user-report` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `report_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        report_id: ::std::option::Option<::std::string::String>,
        resolved_as: ::std::option::Option<::std::option::Option<RequestResolvedAs>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                report_id: ::std::option::Option::None,
                resolved_as: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn resolved_as(mut self, value: impl ::std::convert::Into<RequestResolvedAs>) -> Self {
            self.resolved_as =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`resolvedAs` を `null` として送る。"]
        pub fn resolved_as_null(mut self) -> Self {
            self.resolved_as = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn report_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                report_id: ::std::option::Option::Some(value.into()),
                resolved_as: self.resolved_as,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                report_id: self
                    .report_id
                    .expect("required parameter is set by the type state"),
                resolved_as: self.resolved_as,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/forward-abuse-user-report`"]
pub mod forward_abuse_user_report {
    #[doc = "`admin/forward-abuse-user-report`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "reportId")]
        pub report_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/forward-abuse-user-report";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/forward-abuse-user-report` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `report_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        report_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                report_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn report_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                report_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                report_id: self
                    .report_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/update-abuse-user-report`"]
pub mod update_abuse_user_report {
    #[doc = "`admin/update-abuse-user-report`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "reportId")]
        pub report_id: ::std::string::String,
        #[serde(
            rename = "moderationNote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub moderation_note: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/update-abuse-user-report";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/update-abuse-user-report` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `report_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        report_id: ::std::option::Option<::std::string::String>,
        moderation_note: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                report_id: ::std::option::Option::None,
                moderation_note: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn moderation_note(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.moderation_note = ::std::option::Option::Some(value.into());
            self
        }
        pub fn report_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                report_id: ::std::option::Option::Some(value.into()),
                moderation_note: self.moderation_note,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                report_id: self
                    .report_id
                    .expect("required parameter is set by the type state"),
                moderation_note: self.moderation_note,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/send-email`"]
pub mod send_email {
    #[doc = "`admin/send-email`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:send-email*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub to: ::std::string::String,
        pub subject: ::std::string::String,
        pub text: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/send-email";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/send-email` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:send-email*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `to`、`subject`、`text`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2024_10_1::Client,
        to: ::std::option::Option<::std::string::String>,
        subject: ::std::option::Option<::std::string::String>,
        text: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                to: ::std::option::Option::None,
                subject: ::std::option::Option::None,
                text: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
        pub fn to(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2)> {
            Call {
                client: self.client,
                to: ::std::option::Option::Some(value.into()),
                subject: self.subject,
                text: self.text,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn subject(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2)> {
            Call {
                client: self.client,
                to: self.to,
                subject: ::std::option::Option::Some(value.into()),
                text: self.text,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn text(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, S1, crate::builder::Set)> {
            Call {
                client: self.client,
                to: self.to,
                subject: self.subject,
                text: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture
        for Call<
            'a,
            (
                crate::builder::Set,
                crate::builder::Set,
                crate::builder::Set,
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                to: self
                    .to
                    .expect("required parameter is set by the type state"),
                subject: self
                    .subject
                    .expect("required parameter is set by the type state"),
                text: self
                    .text
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/server-info`"]
pub mod server_info {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseCpu {
        pub model: ::std::string::String,
        pub cores: f64,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseMem {
        pub total: f64,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseFs {
        pub total: f64,
        pub used: f64,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNet {
        pub interface: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub machine: ::std::string::String,
        pub os: ::std::string::String,
        pub node: ::std::string::String,
        pub psql: ::std::string::String,
        pub cpu: ResponseCpu,
        pub mem: ResponseMem,
        pub fs: ResponseFs,
        pub net: ResponseNet,
    }
    #[doc = "`admin/server-info`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:server-info*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/server-info";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`admin/server-info` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:server-info*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/show-moderation-logs`"]
pub mod show_moderation_logs {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::Utc>,
        pub r#type: ::std::string::String,
        pub info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        pub user: crate::generated::v2024_10_1::types::UserDetailedNotMe,
    }
    #[doc = "`admin/show-moderation-logs`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-moderation-log*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/show-moderation-logs";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`admin/show-moderation-logs` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-moderation-log*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                r#type: ::std::option::Option::None,
                user_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn r#type(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.r#type = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`type` を `null` として送る。"]
        pub fn type_null(mut self) -> Self {
            self.r#type = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn user_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userId` を `null` として送る。"]
        pub fn user_id_null(mut self) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                r#type: self.r#type,
                user_id: self.user_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/show-user`"]
pub mod show_user {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseMutedWordsItem {
        String(::std::string::String),
        Array(::std::vec::Vec<::std::string::String>),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigNoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigNoteList {
        pub r#type: ResponseNotificationRecieveConfigNoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigNoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigNoteObject {
        pub r#type: ResponseNotificationRecieveConfigNoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigNote {
        List(ResponseNotificationRecieveConfigNoteList),
        Object(ResponseNotificationRecieveConfigNoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigFollowListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigFollowList {
        pub r#type: ResponseNotificationRecieveConfigFollowListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigFollowObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigFollowObject {
        pub r#type: ResponseNotificationRecieveConfigFollowObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigFollow {
        List(ResponseNotificationRecieveConfigFollowList),
        Object(ResponseNotificationRecieveConfigFollowObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigMentionListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigMentionList {
        pub r#type: ResponseNotificationRecieveConfigMentionListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigMentionObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigMentionObject {
        pub r#type: ResponseNotificationRecieveConfigMentionObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigMention {
        List(ResponseNotificationRecieveConfigMentionList),
        Object(ResponseNotificationRecieveConfigMentionObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReplyListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReplyList {
        pub r#type: ResponseNotificationRecieveConfigReplyListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReplyObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReplyObject {
        pub r#type: ResponseNotificationRecieveConfigReplyObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigReply {
        List(ResponseNotificationRecieveConfigReplyList),
        Object(ResponseNotificationRecieveConfigReplyObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigRenoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigRenoteList {
        pub r#type: ResponseNotificationRecieveConfigRenoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigRenoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigRenoteObject {
        pub r#type: ResponseNotificationRecieveConfigRenoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigRenote {
        List(ResponseNotificationRecieveConfigRenoteList),
        Object(ResponseNotificationRecieveConfigRenoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigQuoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigQuoteList {
        pub r#type: ResponseNotificationRecieveConfigQuoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigQuoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigQuoteObject {
        pub r#type: ResponseNotificationRecieveConfigQuoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigQuote {
        List(ResponseNotificationRecieveConfigQuoteList),
        Object(ResponseNotificationRecieveConfigQuoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReactionListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReactionList {
        pub r#type: ResponseNotificationRecieveConfigReactionListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReactionObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReactionObject {
        pub r#type: ResponseNotificationRecieveConfigReactionObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigReaction {
        List(ResponseNotificationRecieveConfigReactionList),
        Object(ResponseNotificationRecieveConfigReactionObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigPollEndedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigPollEndedList {
        pub r#type: ResponseNotificationRecieveConfigPollEndedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigPollEndedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigPollEndedObject {
        pub r#type: ResponseNotificationRecieveConfigPollEndedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigPollEnded {
        List(ResponseNotificationRecieveConfigPollEndedList),
        Object(ResponseNotificationRecieveConfigPollEndedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReceiveFollowRequestListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReceiveFollowRequestList {
        pub r#type: ResponseNotificationRecieveConfigReceiveFollowRequestListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigReceiveFollowRequestObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigReceiveFollowRequestObject {
        pub r#type: ResponseNotificationRecieveConfigReceiveFollowRequestObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigReceiveFollowRequest {
        List(ResponseNotificationRecieveConfigReceiveFollowRequestList),
        Object(ResponseNotificationRecieveConfigReceiveFollowRequestObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigFollowRequestAcceptedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigFollowRequestAcceptedList {
        pub r#type: ResponseNotificationRecieveConfigFollowRequestAcceptedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigFollowRequestAcceptedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigFollowRequestAcceptedObject {
        pub r#type: ResponseNotificationRecieveConfigFollowRequestAcceptedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigFollowRequestAccepted {
        List(ResponseNotificationRecieveConfigFollowRequestAcceptedList),
        Object(ResponseNotificationRecieveConfigFollowRequestAcceptedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigRoleAssignedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigRoleAssignedList {
        pub r#type: ResponseNotificationRecieveConfigRoleAssignedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigRoleAssignedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigRoleAssignedObject {
        pub r#type: ResponseNotificationRecieveConfigRoleAssignedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigRoleAssigned {
        List(ResponseNotificationRecieveConfigRoleAssignedList),
        Object(ResponseNotificationRecieveConfigRoleAssignedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigAchievementEarnedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigAchievementEarnedList {
        pub r#type: ResponseNotificationRecieveConfigAchievementEarnedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigAchievementEarnedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigAchievementEarnedObject {
        pub r#type: ResponseNotificationRecieveConfigAchievementEarnedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigAchievementEarned {
        List(ResponseNotificationRecieveConfigAchievementEarnedList),
        Object(ResponseNotificationRecieveConfigAchievementEarnedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigAppListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigAppList {
        pub r#type: ResponseNotificationRecieveConfigAppListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigAppObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigAppObject {
        pub r#type: ResponseNotificationRecieveConfigAppObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigApp {
        List(ResponseNotificationRecieveConfigAppList),
        Object(ResponseNotificationRecieveConfigAppObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigTestListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigTestList {
        pub r#type: ResponseNotificationRecieveConfigTestListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNotificationRecieveConfigTestObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfigTestObject {
        pub r#type: ResponseNotificationRecieveConfigTestObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum ResponseNotificationRecieveConfigTest {
        List(ResponseNotificationRecieveConfigTestList),
        Object(ResponseNotificationRecieveConfigTestObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotificationRecieveConfig {
        #[serde(default)]
        pub note: ::std::option::Option<ResponseNotificationRecieveConfigNote>,
        #[serde(default)]
        pub follow: ::std::option::Option<ResponseNotificationRecieveConfigFollow>,
        #[serde(default)]
        pub mention: ::std::option::Option<ResponseNotificationRecieveConfigMention>,
        #[serde(default)]
        pub reply: ::std::option::Option<ResponseNotificationRecieveConfigReply>,
        #[serde(default)]
        pub renote: ::std::option::Option<ResponseNotificationRecieveConfigRenote>,
        #[serde(default)]
        pub quote: ::std::option::Option<ResponseNotificationRecieveConfigQuote>,
        #[serde(default)]
        pub reaction: ::std::option::Option<ResponseNotificationRecieveConfigReaction>,
        #[serde(rename = "pollEnded", default)]
        pub poll_ended: ::std::option::Option<ResponseNotificationRecieveConfigPollEnded>,
        #[serde(rename = "receiveFollowRequest", default)]
        pub receive_follow_request:
            ::std::option::Option<ResponseNotificationRecieveConfigReceiveFollowRequest>,
        #[serde(rename = "followRequestAccepted", default)]
        pub follow_request_accepted:
            ::std::option::Option<ResponseNotificationRecieveConfigFollowRequestAccepted>,
        #[serde(rename = "roleAssigned", default)]
        pub role_assigned: ::std::option::Option<ResponseNotificationRecieveConfigRoleAssigned>,
        #[serde(rename = "achievementEarned", default)]
        pub achievement_earned:
            ::std::option::Option<ResponseNotificationRecieveConfigAchievementEarned>,
        #[serde(default)]
        pub app: ::std::option::Option<ResponseNotificationRecieveConfigApp>,
        #[serde(default)]
        pub test: ::std::option::Option<ResponseNotificationRecieveConfigTest>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRoleAssignsItem {
        #[serde(rename = "createdAt")]
        pub created_at: ::std::string::String,
        #[serde(rename = "expiresAt")]
        pub expires_at: ::std::option::Option<::std::string::String>,
        #[serde(rename = "roleId")]
        pub role_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(rename = "emailVerified")]
        pub email_verified: bool,
        #[serde(rename = "followedMessage")]
        pub followed_message: ::std::option::Option<::std::string::String>,
        #[serde(rename = "autoAcceptFollowed")]
        pub auto_accept_followed: bool,
        #[serde(rename = "noCrawle")]
        pub no_crawle: bool,
        #[serde(rename = "preventAiLearning")]
        pub prevent_ai_learning: bool,
        #[serde(rename = "alwaysMarkNsfw")]
        pub always_mark_nsfw: bool,
        #[serde(rename = "autoSensitive")]
        pub auto_sensitive: bool,
        #[serde(rename = "carefulBot")]
        pub careful_bot: bool,
        #[serde(rename = "injectFeaturedNote")]
        pub inject_featured_note: bool,
        #[serde(rename = "receiveAnnouncementEmail")]
        pub receive_announcement_email: bool,
        #[serde(rename = "mutedWords")]
        pub muted_words: ::std::vec::Vec<ResponseMutedWordsItem>,
        #[serde(rename = "mutedInstances")]
        pub muted_instances: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "notificationRecieveConfig")]
        pub notification_recieve_config: ResponseNotificationRecieveConfig,
        #[serde(rename = "isModerator")]
        pub is_moderator: bool,
        #[serde(rename = "isSilenced")]
        pub is_silenced: bool,
        #[serde(rename = "isSuspended")]
        pub is_suspended: bool,
        #[serde(rename = "isHibernated")]
        pub is_hibernated: bool,
        #[serde(rename = "lastActiveDate")]
        pub last_active_date: ::std::option::Option<::std::string::String>,
        #[serde(rename = "moderationNote")]
        pub moderation_note: ::std::string::String,
        pub signins: ::std::vec::Vec<crate::generated::v2024_10_1::types::Signin>,
        pub policies: crate::generated::v2024_10_1::types::RolePolicies,
        pub roles: ::std::vec::Vec<crate::generated::v2024_10_1::types::Role>,
        #[serde(rename = "roleAssigns")]
        pub role_assigns: ::std::vec::Vec<ResponseRoleAssignsItem>,
    }
    #[doc = "`admin/show-user`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-user*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/show-user";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`admin/show-user` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-user*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/show-users`"]
pub mod show_users {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+follower")]
        PlusFollower,
        #[serde(rename = "-follower")]
        MinusFollower,
        #[serde(rename = "+createdAt")]
        PlusCreatedAt,
        #[serde(rename = "-createdAt")]
        MinusCreatedAt,
        #[serde(rename = "+updatedAt")]
        PlusUpdatedAt,
        #[serde(rename = "-updatedAt")]
        MinusUpdatedAt,
        #[serde(rename = "+lastActiveDate")]
        PlusLastActiveDate,
        #[serde(rename = "-lastActiveDate")]
        MinusLastActiveDate,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestState {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "alive")]
        Alive,
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "moderator")]
        Moderator,
        #[serde(rename = "adminOrModerator")]
        AdminOrModerator,
        #[serde(rename = "suspended")]
        Suspended,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestOrigin {
        #[serde(rename = "combined")]
        Combined,
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`admin/show-users`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-user*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort: ::std::option::Option<RequestSort>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: ::std::option::Option<RequestState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: ::std::option::Option<RequestOrigin>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub username: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[doc = "The local host is represented with `null`."]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/show-users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::UserDetailed>;
    #[doc = "`admin/show-users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:show-user*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        sort: ::std::option::Option<RequestSort>,
        state: ::std::option::Option<RequestState>,
        origin: ::std::option::Option<RequestOrigin>,
        username: ::std::option::Option<::std::option::Option<::std::string::String>>,
        hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                sort: ::std::option::Option::None,
                state: ::std::option::Option::None,
                origin: ::std::option::Option::None,
                username: ::std::option::Option::None,
                hostname: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.offset = ::std::option::Option::Some(value.into());
            self
        }
        pub fn sort(mut self, value: impl ::std::convert::Into<RequestSort>) -> Self {
            self.sort = ::std::option::Option::Some(value.into());
            self
        }
        pub fn state(mut self, value: impl ::std::convert::Into<RequestState>) -> Self {
            self.state = ::std::option::Option::Some(value.into());
            self
        }
        pub fn origin(mut self, value: impl ::std::convert::Into<RequestOrigin>) -> Self {
            self.origin = ::std::option::Option::Some(value.into());
            self
        }
        pub fn username(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.username = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`username` を `null` として送る。"]
        pub fn username_null(mut self) -> Self {
            self.username = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        #[doc = "The local host is represented with `null`."]
        pub fn hostname(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.hostname = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`hostname` を `null` として送る。"]
        pub fn hostname_null(mut self) -> Self {
            self.hostname = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                offset: self.offset,
                sort: self.sort,
                state: self.state,
                origin: self.origin,
                username: self.username,
                hostname: self.hostname,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/suspend-user`"]
pub mod suspend_user {
    #[doc = "`admin/suspend-user`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:suspend-user*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/suspend-user";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/suspend-user` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:suspend-user*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/unsuspend-user`"]
pub mod unsuspend_user {
    #[doc = "`admin/unsuspend-user`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unsuspend-user*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/unsuspend-user";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/unsuspend-user` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:unsuspend-user*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/update-meta`"]
pub mod update_meta {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSensitiveMediaDetection {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSensitiveMediaDetectionSensitivity {
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "high")]
        High,
        #[serde(rename = "veryLow")]
        VeryLow,
        #[serde(rename = "veryHigh")]
        VeryHigh,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestFederation {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "specified")]
        Specified,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`admin/update-meta`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:meta*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(
            rename = "disableRegistration",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub disable_registration: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "pinnedUsers",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub pinned_users:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "hiddenTags",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub hidden_tags:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "blockedHosts",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub blocked_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "sensitiveWords",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub sensitive_words:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "prohibitedWords",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub prohibited_words:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "prohibitedWordsForNameOfUser",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub prohibited_words_for_name_of_user:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "themeColor",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub theme_color: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "mascotImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub mascot_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "bannerUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub banner_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "serverErrorImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub server_error_image_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "infoImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub info_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "notFoundImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub not_found_image_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "app192IconUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub app192_icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "app512IconUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub app512_icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "backgroundImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub background_image_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "logoImageUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub logo_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "shortName",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub short_name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "defaultLightTheme",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub default_light_theme:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "defaultDarkTheme",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub default_dark_theme: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "cacheRemoteFiles",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cache_remote_files: ::std::option::Option<bool>,
        #[serde(
            rename = "cacheRemoteSensitiveFiles",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cache_remote_sensitive_files: ::std::option::Option<bool>,
        #[serde(
            rename = "emailRequiredForSignup",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub email_required_for_signup: ::std::option::Option<bool>,
        #[serde(
            rename = "enableHcaptcha",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_hcaptcha: ::std::option::Option<bool>,
        #[serde(
            rename = "hcaptchaSiteKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub hcaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "hcaptchaSecretKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub hcaptcha_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableMcaptcha",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_mcaptcha: ::std::option::Option<bool>,
        #[serde(
            rename = "mcaptchaSiteKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub mcaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "mcaptchaInstanceUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub mcaptcha_instance_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "mcaptchaSecretKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub mcaptcha_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableRecaptcha",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_recaptcha: ::std::option::Option<bool>,
        #[serde(
            rename = "recaptchaSiteKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub recaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "recaptchaSecretKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub recaptcha_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableTurnstile",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_turnstile: ::std::option::Option<bool>,
        #[serde(
            rename = "turnstileSiteKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub turnstile_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "turnstileSecretKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub turnstile_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableTestcaptcha",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_testcaptcha: ::std::option::Option<bool>,
        #[serde(
            rename = "sensitiveMediaDetection",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sensitive_media_detection: ::std::option::Option<RequestSensitiveMediaDetection>,
        #[serde(
            rename = "sensitiveMediaDetectionSensitivity",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sensitive_media_detection_sensitivity:
            ::std::option::Option<RequestSensitiveMediaDetectionSensitivity>,
        #[serde(
            rename = "setSensitiveFlagAutomatically",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub set_sensitive_flag_automatically: ::std::option::Option<bool>,
        #[serde(
            rename = "enableSensitiveMediaDetectionForVideos",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_sensitive_media_detection_for_videos: ::std::option::Option<bool>,
        #[serde(
            rename = "proxyAccountId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub proxy_account_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "maintainerName",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub maintainer_name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "maintainerEmail",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub maintainer_email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub langs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "deeplAuthKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub deepl_auth_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "deeplIsPro",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deepl_is_pro: ::std::option::Option<bool>,
        #[serde(
            rename = "enableEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_email: ::std::option::Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "smtpSecure",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub smtp_secure: ::std::option::Option<bool>,
        #[serde(
            rename = "smtpHost",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub smtp_host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "smtpPort",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub smtp_port: ::std::option::Option<::std::option::Option<i64>>,
        #[serde(
            rename = "smtpUser",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub smtp_user: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "smtpPass",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub smtp_pass: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableServiceWorker",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_service_worker: ::std::option::Option<bool>,
        #[serde(
            rename = "swPublicKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub sw_public_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "swPrivateKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub sw_private_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "tosUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub tos_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "repositoryUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub repository_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "feedbackUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub feedback_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "impressumUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub impressum_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "privacyPolicyUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub privacy_policy_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "inquiryUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub inquiry_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "useObjectStorage",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_object_storage: ::std::option::Option<bool>,
        #[serde(
            rename = "objectStorageBaseUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_base_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStorageBucket",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_bucket:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStoragePrefix",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_prefix:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStorageEndpoint",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_endpoint:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStorageRegion",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_region:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStoragePort",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_port: ::std::option::Option<::std::option::Option<i64>>,
        #[serde(
            rename = "objectStorageAccessKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_access_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStorageSecretKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub object_storage_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "objectStorageUseSSL",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub object_storage_use_ssl: ::std::option::Option<bool>,
        #[serde(
            rename = "objectStorageUseProxy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub object_storage_use_proxy: ::std::option::Option<bool>,
        #[serde(
            rename = "objectStorageSetPublicRead",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub object_storage_set_public_read: ::std::option::Option<bool>,
        #[serde(
            rename = "objectStorageS3ForcePathStyle",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub object_storage_s3_force_path_style: ::std::option::Option<bool>,
        #[serde(
            rename = "enableIpLogging",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_ip_logging: ::std::option::Option<bool>,
        #[serde(
            rename = "enableActiveEmailValidation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_active_email_validation: ::std::option::Option<bool>,
        #[serde(
            rename = "enableVerifymailApi",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_verifymail_api: ::std::option::Option<bool>,
        #[serde(
            rename = "verifymailAuthKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub verifymail_auth_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableTruemailApi",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_truemail_api: ::std::option::Option<bool>,
        #[serde(
            rename = "truemailInstance",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub truemail_instance: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "truemailAuthKey",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub truemail_auth_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "enableChartsForRemoteUser",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_charts_for_remote_user: ::std::option::Option<bool>,
        #[serde(
            rename = "enableChartsForFederatedInstances",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_charts_for_federated_instances: ::std::option::Option<bool>,
        #[serde(
            rename = "enableStatsForFederatedInstances",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_stats_for_federated_instances: ::std::option::Option<bool>,
        #[serde(
            rename = "enableServerMachineStats",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_server_machine_stats: ::std::option::Option<bool>,
        #[serde(
            rename = "enableIdenticonGeneration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_identicon_generation: ::std::option::Option<bool>,
        #[serde(
            rename = "serverRules",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub server_rules: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "bannedEmailDomains",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub banned_email_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "preservedUsernames",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preserved_usernames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "manifestJsonOverride",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub manifest_json_override: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "enableFanoutTimeline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_fanout_timeline: ::std::option::Option<bool>,
        #[serde(
            rename = "enableFanoutTimelineDbFallback",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_fanout_timeline_db_fallback: ::std::option::Option<bool>,
        #[serde(
            rename = "perLocalUserUserTimelineCacheMax",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_local_user_user_timeline_cache_max: ::std::option::Option<i64>,
        #[serde(
            rename = "perRemoteUserUserTimelineCacheMax",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_remote_user_user_timeline_cache_max: ::std::option::Option<i64>,
        #[serde(
            rename = "perUserHomeTimelineCacheMax",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_user_home_timeline_cache_max: ::std::option::Option<i64>,
        #[serde(
            rename = "perUserListTimelineCacheMax",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_user_list_timeline_cache_max: ::std::option::Option<i64>,
        #[serde(
            rename = "enableReactionsBuffering",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_reactions_buffering: ::std::option::Option<bool>,
        #[serde(
            rename = "notesPerOneAd",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub notes_per_one_ad: ::std::option::Option<i64>,
        #[serde(
            rename = "silencedHosts",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub silenced_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "mediaSilencedHosts",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub media_silenced_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        #[doc = "\\[Deprecated\\] Use \"urlPreviewSummaryProxyUrl\" instead."]
        #[serde(
            rename = "summalyProxy",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub summaly_proxy: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "urlPreviewEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub url_preview_enabled: ::std::option::Option<bool>,
        #[serde(
            rename = "urlPreviewTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub url_preview_timeout: ::std::option::Option<i64>,
        #[serde(
            rename = "urlPreviewMaximumContentLength",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub url_preview_maximum_content_length: ::std::option::Option<i64>,
        #[serde(
            rename = "urlPreviewRequireContentLength",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub url_preview_require_content_length: ::std::option::Option<bool>,
        #[serde(
            rename = "urlPreviewUserAgent",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub url_preview_user_agent:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "urlPreviewSummaryProxyUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub url_preview_summary_proxy_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub federation: ::std::option::Option<RequestFederation>,
        #[serde(
            rename = "federationHosts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub federation_hosts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/update-meta";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/update-meta` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:meta*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        disable_registration: ::std::option::Option<::std::option::Option<bool>>,
        pinned_users:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        hidden_tags:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        blocked_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        sensitive_words:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        prohibited_words:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        prohibited_words_for_name_of_user:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        theme_color: ::std::option::Option<::std::option::Option<::std::string::String>>,
        mascot_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        banner_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        server_error_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        info_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        not_found_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        app192_icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        app512_icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        background_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        logo_image_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        short_name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        default_light_theme: ::std::option::Option<::std::option::Option<::std::string::String>>,
        default_dark_theme: ::std::option::Option<::std::option::Option<::std::string::String>>,
        cache_remote_files: ::std::option::Option<bool>,
        cache_remote_sensitive_files: ::std::option::Option<bool>,
        email_required_for_signup: ::std::option::Option<bool>,
        enable_hcaptcha: ::std::option::Option<bool>,
        hcaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        hcaptcha_secret_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_mcaptcha: ::std::option::Option<bool>,
        mcaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        mcaptcha_instance_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        mcaptcha_secret_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_recaptcha: ::std::option::Option<bool>,
        recaptcha_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        recaptcha_secret_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_turnstile: ::std::option::Option<bool>,
        turnstile_site_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        turnstile_secret_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_testcaptcha: ::std::option::Option<bool>,
        sensitive_media_detection: ::std::option::Option<RequestSensitiveMediaDetection>,
        sensitive_media_detection_sensitivity:
            ::std::option::Option<RequestSensitiveMediaDetectionSensitivity>,
        set_sensitive_flag_automatically: ::std::option::Option<bool>,
        enable_sensitive_media_detection_for_videos: ::std::option::Option<bool>,
        proxy_account_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        maintainer_name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        maintainer_email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        langs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        deepl_auth_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        deepl_is_pro: ::std::option::Option<bool>,
        enable_email: ::std::option::Option<bool>,
        email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        smtp_secure: ::std::option::Option<bool>,
        smtp_host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        smtp_port: ::std::option::Option<::std::option::Option<i64>>,
        smtp_user: ::std::option::Option<::std::option::Option<::std::string::String>>,
        smtp_pass: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_service_worker: ::std::option::Option<bool>,
        sw_public_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        sw_private_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        tos_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        repository_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        feedback_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        impressum_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        privacy_policy_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        inquiry_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        use_object_storage: ::std::option::Option<bool>,
        object_storage_base_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_bucket: ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_prefix: ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_endpoint:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_region: ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_port: ::std::option::Option<::std::option::Option<i64>>,
        object_storage_access_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_secret_key:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        object_storage_use_ssl: ::std::option::Option<bool>,
        object_storage_use_proxy: ::std::option::Option<bool>,
        object_storage_set_public_read: ::std::option::Option<bool>,
        object_storage_s3_force_path_style: ::std::option::Option<bool>,
        enable_ip_logging: ::std::option::Option<bool>,
        enable_active_email_validation: ::std::option::Option<bool>,
        enable_verifymail_api: ::std::option::Option<bool>,
        verifymail_auth_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_truemail_api: ::std::option::Option<bool>,
        truemail_instance: ::std::option::Option<::std::option::Option<::std::string::String>>,
        truemail_auth_key: ::std::option::Option<::std::option::Option<::std::string::String>>,
        enable_charts_for_remote_user: ::std::option::Option<bool>,
        enable_charts_for_federated_instances: ::std::option::Option<bool>,
        enable_stats_for_federated_instances: ::std::option::Option<bool>,
        enable_server_machine_stats: ::std::option::Option<bool>,
        enable_identicon_generation: ::std::option::Option<bool>,
        server_rules: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        banned_email_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        preserved_usernames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        manifest_json_override: ::std::option::Option<::std::string::String>,
        enable_fanout_timeline: ::std::option::Option<bool>,
        enable_fanout_timeline_db_fallback: ::std::option::Option<bool>,
        per_local_user_user_timeline_cache_max: ::std::option::Option<i64>,
        per_remote_user_user_timeline_cache_max: ::std::option::Option<i64>,
        per_user_home_timeline_cache_max: ::std::option::Option<i64>,
        per_user_list_timeline_cache_max: ::std::option::Option<i64>,
        enable_reactions_buffering: ::std::option::Option<bool>,
        notes_per_one_ad: ::std::option::Option<i64>,
        silenced_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        media_silenced_hosts:
            ::std::option::Option<::std::option::Option<::std::vec::Vec<::std::string::String>>>,
        summaly_proxy: ::std::option::Option<::std::option::Option<::std::string::String>>,
        url_preview_enabled: ::std::option::Option<bool>,
        url_preview_timeout: ::std::option::Option<i64>,
        url_preview_maximum_content_length: ::std::option::Option<i64>,
        url_preview_require_content_length: ::std::option::Option<bool>,
        url_preview_user_agent: ::std::option::Option<::std::option::Option<::std::string::String>>,
        url_preview_summary_proxy_url:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        federation: ::std::option::Option<RequestFederation>,
        federation_hosts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                disable_registration: ::std::option::Option::None,
                pinned_users: ::std::option::Option::None,
                hidden_tags: ::std::option::Option::None,
                blocked_hosts: ::std::option::Option::None,
                sensitive_words: ::std::option::Option::None,
                prohibited_words: ::std::option::Option::None,
                prohibited_words_for_name_of_user: ::std::option::Option::None,
                theme_color: ::std::option::Option::None,
                mascot_image_url: ::std::option::Option::None,
                banner_url: ::std::option::Option::None,
                server_error_image_url: ::std::option::Option::None,
                info_image_url: ::std::option::Option::None,
                not_found_image_url: ::std::option::Option::None,
                icon_url: ::std::option::Option::None,
                app192_icon_url: ::std::option::Option::None,
                app512_icon_url: ::std::option::Option::None,
                background_image_url: ::std::option::Option::None,
                logo_image_url: ::std::option::Option::None,
                name: ::std::option::Option::None,
                short_name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                default_light_theme: ::std::option::Option::None,
                default_dark_theme: ::std::option::Option::None,
                cache_remote_files: ::std::option::Option::None,
                cache_remote_sensitive_files: ::std::option::Option::None,
                email_required_for_signup: ::std::option::Option::None,
                enable_hcaptcha: ::std::option::Option::None,
                hcaptcha_site_key: ::std::option::Option::None,
                hcaptcha_secret_key: ::std::option::Option::None,
                enable_mcaptcha: ::std::option::Option::None,
                mcaptcha_site_key: ::std::option::Option::None,
                mcaptcha_instance_url: ::std::option::Option::None,
                mcaptcha_secret_key: ::std::option::Option::None,
                enable_recaptcha: ::std::option::Option::None,
                recaptcha_site_key: ::std::option::Option::None,
                recaptcha_secret_key: ::std::option::Option::None,
                enable_turnstile: ::std::option::Option::None,
                turnstile_site_key: ::std::option::Option::None,
                turnstile_secret_key: ::std::option::Option::None,
                enable_testcaptcha: ::std::option::Option::None,
                sensitive_media_detection: ::std::option::Option::None,
                sensitive_media_detection_sensitivity: ::std::option::Option::None,
                set_sensitive_flag_automatically: ::std::option::Option::None,
                enable_sensitive_media_detection_for_videos: ::std::option::Option::None,
                proxy_account_id: ::std::option::Option::None,
                maintainer_name: ::std::option::Option::None,
                maintainer_email: ::std::option::Option::None,
                langs: ::std::option::Option::None,
                deepl_auth_key: ::std::option::Option::None,
                deepl_is_pro: ::std::option::Option::None,
                enable_email: ::std::option::Option::None,
                email: ::std::option::Option::None,
                smtp_secure: ::std::option::Option::None,
                smtp_host: ::std::option::Option::None,
                smtp_port: ::std::option::Option::None,
                smtp_user: ::std::option::Option::None,
                smtp_pass: ::std::option::Option::None,
                enable_service_worker: ::std::option::Option::None,
                sw_public_key: ::std::option::Option::None,
                sw_private_key: ::std::option::Option::None,
                tos_url: ::std::option::Option::None,
                repository_url: ::std::option::Option::None,
                feedback_url: ::std::option::Option::None,
                impressum_url: ::std::option::Option::None,
                privacy_policy_url: ::std::option::Option::None,
                inquiry_url: ::std::option::Option::None,
                use_object_storage: ::std::option::Option::None,
                object_storage_base_url: ::std::option::Option::None,
                object_storage_bucket: ::std::option::Option::None,
                object_storage_prefix: ::std::option::Option::None,
                object_storage_endpoint: ::std::option::Option::None,
                object_storage_region: ::std::option::Option::None,
                object_storage_port: ::std::option::Option::None,
                object_storage_access_key: ::std::option::Option::None,
                object_storage_secret_key: ::std::option::Option::None,
                object_storage_use_ssl: ::std::option::Option::None,
                object_storage_use_proxy: ::std::option::Option::None,
                object_storage_set_public_read: ::std::option::Option::None,
                object_storage_s3_force_path_style: ::std::option::Option::None,
                enable_ip_logging: ::std::option::Option::None,
                enable_active_email_validation: ::std::option::Option::None,
                enable_verifymail_api: ::std::option::Option::None,
                verifymail_auth_key: ::std::option::Option::None,
                enable_truemail_api: ::std::option::Option::None,
                truemail_instance: ::std::option::Option::None,
                truemail_auth_key: ::std::option::Option::None,
                enable_charts_for_remote_user: ::std::option::Option::None,
                enable_charts_for_federated_instances: ::std::option::Option::None,
                enable_stats_for_federated_instances: ::std::option::Option::None,
                enable_server_machine_stats: ::std::option::Option::None,
                enable_identicon_generation: ::std::option::Option::None,
                server_rules: ::std::option::Option::None,
                banned_email_domains: ::std::option::Option::None,
                preserved_usernames: ::std::option::Option::None,
                manifest_json_override: ::std::option::Option::None,
                enable_fanout_timeline: ::std::option::Option::None,
                enable_fanout_timeline_db_fallback: ::std::option::Option::None,
                per_local_user_user_timeline_cache_max: ::std::option::Option::None,
                per_remote_user_user_timeline_cache_max: ::std::option::Option::None,
                per_user_home_timeline_cache_max: ::std::option::Option::None,
                per_user_list_timeline_cache_max: ::std::option::Option::None,
                enable_reactions_buffering: ::std::option::Option::None,
                notes_per_one_ad: ::std::option::Option::None,
                silenced_hosts: ::std::option::Option::None,
                media_silenced_hosts: ::std::option::Option::None,
                summaly_proxy: ::std::option::Option::None,
                url_preview_enabled: ::std::option::Option::None,
                url_preview_timeout: ::std::option::Option::None,
                url_preview_maximum_content_length: ::std::option::Option::None,
                url_preview_require_content_length: ::std::option::Option::None,
                url_preview_user_agent: ::std::option::Option::None,
                url_preview_summary_proxy_url: ::std::option::Option::None,
                federation: ::std::option::Option::None,
                federation_hosts: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn disable_registration(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.disable_registration =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`disableRegistration` を `null` として送る。"]
        pub fn disable_registration_null(mut self) -> Self {
            self.disable_registration = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn pinned_users(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.pinned_users =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`pinnedUsers` を `null` として送る。"]
        pub fn pinned_users_null(mut self) -> Self {
            self.pinned_users = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn hidden_tags(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.hidden_tags =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`hiddenTags` を `null` として送る。"]
        pub fn hidden_tags_null(mut self) -> Self {
            self.hidden_tags = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn blocked_hosts(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.blocked_hosts =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`blockedHosts` を `null` として送る。"]
        pub fn blocked_hosts_null(mut self) -> Self {
            self.blocked_hosts = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn sensitive_words(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.sensitive_words =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`sensitiveWords` を `null` として送る。"]
        pub fn sensitive_words_null(mut self) -> Self {
            self.sensitive_words = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn prohibited_words(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.prohibited_words =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`prohibitedWords` を `null` として送る。"]
        pub fn prohibited_words_null(mut self) -> Self {
            self.prohibited_words = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn prohibited_words_for_name_of_user(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.prohibited_words_for_name_of_user =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`prohibitedWordsForNameOfUser` を `null` として送る。"]
        pub fn prohibited_words_for_name_of_user_null(mut self) -> Self {
            self.prohibited_words_for_name_of_user =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn theme_color(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.theme_color =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`themeColor` を `null` として送る。"]
        pub fn theme_color_null(mut self) -> Self {
            self.theme_color = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn mascot_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.mascot_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`mascotImageUrl` を `null` として送る。"]
        pub fn mascot_image_url_null(mut self) -> Self {
            self.mascot_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn banner_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.banner_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerUrl` を `null` として送る。"]
        pub fn banner_url_null(mut self) -> Self {
            self.banner_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn server_error_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.server_error_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`serverErrorImageUrl` を `null` として送る。"]
        pub fn server_error_image_url_null(mut self) -> Self {
            self.server_error_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn info_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.info_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`infoImageUrl` を `null` として送る。"]
        pub fn info_image_url_null(mut self) -> Self {
            self.info_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn not_found_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.not_found_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`notFoundImageUrl` を `null` として送る。"]
        pub fn not_found_image_url_null(mut self) -> Self {
            self.not_found_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn icon_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.icon_url = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`iconUrl` を `null` として送る。"]
        pub fn icon_url_null(mut self) -> Self {
            self.icon_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn app192_icon_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.app192_icon_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`app192IconUrl` を `null` として送る。"]
        pub fn app192_icon_url_null(mut self) -> Self {
            self.app192_icon_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn app512_icon_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.app512_icon_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`app512IconUrl` を `null` として送る。"]
        pub fn app512_icon_url_null(mut self) -> Self {
            self.app512_icon_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn background_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.background_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`backgroundImageUrl` を `null` として送る。"]
        pub fn background_image_url_null(mut self) -> Self {
            self.background_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn logo_image_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.logo_image_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`logoImageUrl` を `null` として送る。"]
        pub fn logo_image_url_null(mut self) -> Self {
            self.logo_image_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn name(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.name = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`name` を `null` として送る。"]
        pub fn name_null(mut self) -> Self {
            self.name = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn short_name(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.short_name =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`shortName` を `null` として送る。"]
        pub fn short_name_null(mut self) -> Self {
            self.short_name = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn description(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.description =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`description` を `null` として送る。"]
        pub fn description_null(mut self) -> Self {
            self.description = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn default_light_theme(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.default_light_theme =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`defaultLightTheme` を `null` として送る。"]
        pub fn default_light_theme_null(mut self) -> Self {
            self.default_light_theme = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn default_dark_theme(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.default_dark_theme =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`defaultDarkTheme` を `null` として送る。"]
        pub fn default_dark_theme_null(mut self) -> Self {
            self.default_dark_theme = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn cache_remote_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.cache_remote_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn cache_remote_sensitive_files(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.cache_remote_sensitive_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn email_required_for_signup(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.email_required_for_signup = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_hcaptcha(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_hcaptcha = ::std::option::Option::Some(value.into());
            self
        }
        pub fn hcaptcha_site_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.hcaptcha_site_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`hcaptchaSiteKey` を `null` として送る。"]
        pub fn hcaptcha_site_key_null(mut self) -> Self {
            self.hcaptcha_site_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn hcaptcha_secret_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.hcaptcha_secret_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`hcaptchaSecretKey` を `null` として送る。"]
        pub fn hcaptcha_secret_key_null(mut self) -> Self {
            self.hcaptcha_secret_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_mcaptcha(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_mcaptcha = ::std::option::Option::Some(value.into());
            self
        }
        pub fn mcaptcha_site_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.mcaptcha_site_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`mcaptchaSiteKey` を `null` として送る。"]
        pub fn mcaptcha_site_key_null(mut self) -> Self {
            self.mcaptcha_site_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn mcaptcha_instance_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.mcaptcha_instance_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`mcaptchaInstanceUrl` を `null` として送る。"]
        pub fn mcaptcha_instance_url_null(mut self) -> Self {
            self.mcaptcha_instance_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn mcaptcha_secret_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.mcaptcha_secret_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`mcaptchaSecretKey` を `null` として送る。"]
        pub fn mcaptcha_secret_key_null(mut self) -> Self {
            self.mcaptcha_secret_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_recaptcha(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_recaptcha = ::std::option::Option::Some(value.into());
            self
        }
        pub fn recaptcha_site_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.recaptcha_site_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`recaptchaSiteKey` を `null` として送る。"]
        pub fn recaptcha_site_key_null(mut self) -> Self {
            self.recaptcha_site_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn recaptcha_secret_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.recaptcha_secret_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`recaptchaSecretKey` を `null` として送る。"]
        pub fn recaptcha_secret_key_null(mut self) -> Self {
            self.recaptcha_secret_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_turnstile(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_turnstile = ::std::option::Option::Some(value.into());
            self
        }
        pub fn turnstile_site_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.turnstile_site_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`turnstileSiteKey` を `null` として送る。"]
        pub fn turnstile_site_key_null(mut self) -> Self {
            self.turnstile_site_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn turnstile_secret_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.turnstile_secret_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`turnstileSecretKey` を `null` として送る。"]
        pub fn turnstile_secret_key_null(mut self) -> Self {
            self.turnstile_secret_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_testcaptcha(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_testcaptcha = ::std::option::Option::Some(value.into());
            self
        }
        pub fn sensitive_media_detection(
            mut self,
            value: impl ::std::convert::Into<RequestSensitiveMediaDetection>,
        ) -> Self {
            self.sensitive_media_detection = ::std::option::Option::Some(value.into());
            self
        }
        pub fn sensitive_media_detection_sensitivity(
            mut self,
            value: impl ::std::convert::Into<RequestSensitiveMediaDetectionSensitivity>,
        ) -> Self {
            self.sensitive_media_detection_sensitivity = ::std::option::Option::Some(value.into());
            self
        }
        pub fn set_sensitive_flag_automatically(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.set_sensitive_flag_automatically = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_sensitive_media_detection_for_videos(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_sensitive_media_detection_for_videos =
                ::std::option::Option::Some(value.into());
            self
        }
        pub fn proxy_account_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.proxy_account_id =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`proxyAccountId` を `null` として送る。"]
        pub fn proxy_account_id_null(mut self) -> Self {
            self.proxy_account_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn maintainer_name(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maintainer_name =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`maintainerName` を `null` として送る。"]
        pub fn maintainer_name_null(mut self) -> Self {
            self.maintainer_name = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn maintainer_email(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maintainer_email =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`maintainerEmail` を `null` として送る。"]
        pub fn maintainer_email_null(mut self) -> Self {
            self.maintainer_email = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn langs(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.langs = ::std::option::Option::Some(value.into());
            self
        }
        pub fn deepl_auth_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.deepl_auth_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`deeplAuthKey` を `null` として送る。"]
        pub fn deepl_auth_key_null(mut self) -> Self {
            self.deepl_auth_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn deepl_is_pro(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.deepl_is_pro = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_email(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_email = ::std::option::Option::Some(value.into());
            self
        }
        pub fn email(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.email = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`email` を `null` として送る。"]
        pub fn email_null(mut self) -> Self {
            self.email = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn smtp_secure(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.smtp_secure = ::std::option::Option::Some(value.into());
            self
        }
        pub fn smtp_host(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.smtp_host = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`smtpHost` を `null` として送る。"]
        pub fn smtp_host_null(mut self) -> Self {
            self.smtp_host = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn smtp_port(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.smtp_port = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`smtpPort` を `null` として送る。"]
        pub fn smtp_port_null(mut self) -> Self {
            self.smtp_port = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn smtp_user(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.smtp_user = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`smtpUser` を `null` として送る。"]
        pub fn smtp_user_null(mut self) -> Self {
            self.smtp_user = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn smtp_pass(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.smtp_pass = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`smtpPass` を `null` として送る。"]
        pub fn smtp_pass_null(mut self) -> Self {
            self.smtp_pass = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_service_worker(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_service_worker = ::std::option::Option::Some(value.into());
            self
        }
        pub fn sw_public_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.sw_public_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`swPublicKey` を `null` として送る。"]
        pub fn sw_public_key_null(mut self) -> Self {
            self.sw_public_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn sw_private_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.sw_private_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`swPrivateKey` を `null` として送る。"]
        pub fn sw_private_key_null(mut self) -> Self {
            self.sw_private_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn tos_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.tos_url = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`tosUrl` を `null` として送る。"]
        pub fn tos_url_null(mut self) -> Self {
            self.tos_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn repository_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.repository_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`repositoryUrl` を `null` として送る。"]
        pub fn repository_url_null(mut self) -> Self {
            self.repository_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn feedback_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.feedback_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`feedbackUrl` を `null` として送る。"]
        pub fn feedback_url_null(mut self) -> Self {
            self.feedback_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn impressum_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.impressum_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`impressumUrl` を `null` として送る。"]
        pub fn impressum_url_null(mut self) -> Self {
            self.impressum_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn privacy_policy_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.privacy_policy_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`privacyPolicyUrl` を `null` として送る。"]
        pub fn privacy_policy_url_null(mut self) -> Self {
            self.privacy_policy_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn inquiry_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.inquiry_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`inquiryUrl` を `null` として送る。"]
        pub fn inquiry_url_null(mut self) -> Self {
            self.inquiry_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn use_object_storage(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.use_object_storage = ::std::option::Option::Some(value.into());
            self
        }
        pub fn object_storage_base_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_base_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageBaseUrl` を `null` として送る。"]
        pub fn object_storage_base_url_null(mut self) -> Self {
            self.object_storage_base_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_bucket(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_bucket =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageBucket` を `null` として送る。"]
        pub fn object_storage_bucket_null(mut self) -> Self {
            self.object_storage_bucket = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_prefix(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_prefix =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStoragePrefix` を `null` として送る。"]
        pub fn object_storage_prefix_null(mut self) -> Self {
            self.object_storage_prefix = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_endpoint(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_endpoint =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageEndpoint` を `null` として送る。"]
        pub fn object_storage_endpoint_null(mut self) -> Self {
            self.object_storage_endpoint = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_region(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_region =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageRegion` を `null` として送る。"]
        pub fn object_storage_region_null(mut self) -> Self {
            self.object_storage_region = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_port(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.object_storage_port =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStoragePort` を `null` として送る。"]
        pub fn object_storage_port_null(mut self) -> Self {
            self.object_storage_port = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_access_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_access_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageAccessKey` を `null` として送る。"]
        pub fn object_storage_access_key_null(mut self) -> Self {
            self.object_storage_access_key =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_secret_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.object_storage_secret_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`objectStorageSecretKey` を `null` として送る。"]
        pub fn object_storage_secret_key_null(mut self) -> Self {
            self.object_storage_secret_key =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn object_storage_use_ssl(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.object_storage_use_ssl = ::std::option::Option::Some(value.into());
            self
        }
        pub fn object_storage_use_proxy(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.object_storage_use_proxy = ::std::option::Option::Some(value.into());
            self
        }
        pub fn object_storage_set_public_read(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.object_storage_set_public_read = ::std::option::Option::Some(value.into());
            self
        }
        pub fn object_storage_s3_force_path_style(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.object_storage_s3_force_path_style = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_ip_logging(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_ip_logging = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_active_email_validation(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_active_email_validation = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_verifymail_api(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_verifymail_api = ::std::option::Option::Some(value.into());
            self
        }
        pub fn verifymail_auth_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.verifymail_auth_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`verifymailAuthKey` を `null` として送る。"]
        pub fn verifymail_auth_key_null(mut self) -> Self {
            self.verifymail_auth_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_truemail_api(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_truemail_api = ::std::option::Option::Some(value.into());
            self
        }
        pub fn truemail_instance(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.truemail_instance =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`truemailInstance` を `null` として送る。"]
        pub fn truemail_instance_null(mut self) -> Self {
            self.truemail_instance = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn truemail_auth_key(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.truemail_auth_key =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`truemailAuthKey` を `null` として送る。"]
        pub fn truemail_auth_key_null(mut self) -> Self {
            self.truemail_auth_key = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn enable_charts_for_remote_user(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_charts_for_remote_user = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_charts_for_federated_instances(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_charts_for_federated_instances = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_stats_for_federated_instances(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_stats_for_federated_instances = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_server_machine_stats(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_server_machine_stats = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_identicon_generation(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_identicon_generation = ::std::option::Option::Some(value.into());
            self
        }
        pub fn server_rules(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.server_rules = ::std::option::Option::Some(value.into());
            self
        }
        pub fn banned_email_domains(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.banned_email_domains = ::std::option::Option::Some(value.into());
            self
        }
        pub fn preserved_usernames(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.preserved_usernames = ::std::option::Option::Some(value.into());
            self
        }
        pub fn manifest_json_override(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.manifest_json_override = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_fanout_timeline(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.enable_fanout_timeline = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_fanout_timeline_db_fallback(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_fanout_timeline_db_fallback = ::std::option::Option::Some(value.into());
            self
        }
        pub fn per_local_user_user_timeline_cache_max(
            mut self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.per_local_user_user_timeline_cache_max = ::std::option::Option::Some(value.into());
            self
        }
        pub fn per_remote_user_user_timeline_cache_max(
            mut self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.per_remote_user_user_timeline_cache_max =
                ::std::option::Option::Some(value.into());
            self
        }
        pub fn per_user_home_timeline_cache_max(
            mut self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.per_user_home_timeline_cache_max = ::std::option::Option::Some(value.into());
            self
        }
        pub fn per_user_list_timeline_cache_max(
            mut self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.per_user_list_timeline_cache_max = ::std::option::Option::Some(value.into());
            self
        }
        pub fn enable_reactions_buffering(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.enable_reactions_buffering = ::std::option::Option::Some(value.into());
            self
        }
        pub fn notes_per_one_ad(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.notes_per_one_ad = ::std::option::Option::Some(value.into());
            self
        }
        pub fn silenced_hosts(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.silenced_hosts =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`silencedHosts` を `null` として送る。"]
        pub fn silenced_hosts_null(mut self) -> Self {
            self.silenced_hosts = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn media_silenced_hosts(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.media_silenced_hosts =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`mediaSilencedHosts` を `null` として送る。"]
        pub fn media_silenced_hosts_null(mut self) -> Self {
            self.media_silenced_hosts = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        #[doc = "\\[Deprecated\\] Use \"urlPreviewSummaryProxyUrl\" instead."]
        pub fn summaly_proxy(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.summaly_proxy =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`summalyProxy` を `null` として送る。"]
        pub fn summaly_proxy_null(mut self) -> Self {
            self.summaly_proxy = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn url_preview_enabled(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.url_preview_enabled = ::std::option::Option::Some(value.into());
            self
        }
        pub fn url_preview_timeout(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.url_preview_timeout = ::std::option::Option::Some(value.into());
            self
        }
        pub fn url_preview_maximum_content_length(
            mut self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.url_preview_maximum_content_length = ::std::option::Option::Some(value.into());
            self
        }
        pub fn url_preview_require_content_length(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.url_preview_require_content_length = ::std::option::Option::Some(value.into());
            self
        }
        pub fn url_preview_user_agent(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.url_preview_user_agent =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`urlPreviewUserAgent` を `null` として送る。"]
        pub fn url_preview_user_agent_null(mut self) -> Self {
            self.url_preview_user_agent = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn url_preview_summary_proxy_url(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.url_preview_summary_proxy_url =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`urlPreviewSummaryProxyUrl` を `null` として送る。"]
        pub fn url_preview_summary_proxy_url_null(mut self) -> Self {
            self.url_preview_summary_proxy_url =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn federation(mut self, value: impl ::std::convert::Into<RequestFederation>) -> Self {
            self.federation = ::std::option::Option::Some(value.into());
            self
        }
        pub fn federation_hosts(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.federation_hosts = ::std::option::Option::Some(value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                disable_registration: self.disable_registration,
                pinned_users: self.pinned_users,
                hidden_tags: self.hidden_tags,
                blocked_hosts: self.blocked_hosts,
                sensitive_words: self.sensitive_words,
                prohibited_words: self.prohibited_words,
                prohibited_words_for_name_of_user: self.prohibited_words_for_name_of_user,
                theme_color: self.theme_color,
                mascot_image_url: self.mascot_image_url,
                banner_url: self.banner_url,
                server_error_image_url: self.server_error_image_url,
                info_image_url: self.info_image_url,
                not_found_image_url: self.not_found_image_url,
                icon_url: self.icon_url,
                app192_icon_url: self.app192_icon_url,
                app512_icon_url: self.app512_icon_url,
                background_image_url: self.background_image_url,
                logo_image_url: self.logo_image_url,
                name: self.name,
                short_name: self.short_name,
                description: self.description,
                default_light_theme: self.default_light_theme,
                default_dark_theme: self.default_dark_theme,
                cache_remote_files: self.cache_remote_files,
                cache_remote_sensitive_files: self.cache_remote_sensitive_files,
                email_required_for_signup: self.email_required_for_signup,
                enable_hcaptcha: self.enable_hcaptcha,
                hcaptcha_site_key: self.hcaptcha_site_key,
                hcaptcha_secret_key: self.hcaptcha_secret_key,
                enable_mcaptcha: self.enable_mcaptcha,
                mcaptcha_site_key: self.mcaptcha_site_key,
                mcaptcha_instance_url: self.mcaptcha_instance_url,
                mcaptcha_secret_key: self.mcaptcha_secret_key,
                enable_recaptcha: self.enable_recaptcha,
                recaptcha_site_key: self.recaptcha_site_key,
                recaptcha_secret_key: self.recaptcha_secret_key,
                enable_turnstile: self.enable_turnstile,
                turnstile_site_key: self.turnstile_site_key,
                turnstile_secret_key: self.turnstile_secret_key,
                enable_testcaptcha: self.enable_testcaptcha,
                sensitive_media_detection: self.sensitive_media_detection,
                sensitive_media_detection_sensitivity: self.sensitive_media_detection_sensitivity,
                set_sensitive_flag_automatically: self.set_sensitive_flag_automatically,
                enable_sensitive_media_detection_for_videos: self
                    .enable_sensitive_media_detection_for_videos,
                proxy_account_id: self.proxy_account_id,
                maintainer_name: self.maintainer_name,
                maintainer_email: self.maintainer_email,
                langs: self.langs,
                deepl_auth_key: self.deepl_auth_key,
                deepl_is_pro: self.deepl_is_pro,
                enable_email: self.enable_email,
                email: self.email,
                smtp_secure: self.smtp_secure,
                smtp_host: self.smtp_host,
                smtp_port: self.smtp_port,
                smtp_user: self.smtp_user,
                smtp_pass: self.smtp_pass,
                enable_service_worker: self.enable_service_worker,
                sw_public_key: self.sw_public_key,
                sw_private_key: self.sw_private_key,
                tos_url: self.tos_url,
                repository_url: self.repository_url,
                feedback_url: self.feedback_url,
                impressum_url: self.impressum_url,
                privacy_policy_url: self.privacy_policy_url,
                inquiry_url: self.inquiry_url,
                use_object_storage: self.use_object_storage,
                object_storage_base_url: self.object_storage_base_url,
                object_storage_bucket: self.object_storage_bucket,
                object_storage_prefix: self.object_storage_prefix,
                object_storage_endpoint: self.object_storage_endpoint,
                object_storage_region: self.object_storage_region,
                object_storage_port: self.object_storage_port,
                object_storage_access_key: self.object_storage_access_key,
                object_storage_secret_key: self.object_storage_secret_key,
                object_storage_use_ssl: self.object_storage_use_ssl,
                object_storage_use_proxy: self.object_storage_use_proxy,
                object_storage_set_public_read: self.object_storage_set_public_read,
                object_storage_s3_force_path_style: self.object_storage_s3_force_path_style,
                enable_ip_logging: self.enable_ip_logging,
                enable_active_email_validation: self.enable_active_email_validation,
                enable_verifymail_api: self.enable_verifymail_api,
                verifymail_auth_key: self.verifymail_auth_key,
                enable_truemail_api: self.enable_truemail_api,
                truemail_instance: self.truemail_instance,
                truemail_auth_key: self.truemail_auth_key,
                enable_charts_for_remote_user: self.enable_charts_for_remote_user,
                enable_charts_for_federated_instances: self.enable_charts_for_federated_instances,
                enable_stats_for_federated_instances: self.enable_stats_for_federated_instances,
                enable_server_machine_stats: self.enable_server_machine_stats,
                enable_identicon_generation: self.enable_identicon_generation,
                server_rules: self.server_rules,
                banned_email_domains: self.banned_email_domains,
                preserved_usernames: self.preserved_usernames,
                manifest_json_override: self.manifest_json_override,
                enable_fanout_timeline: self.enable_fanout_timeline,
                enable_fanout_timeline_db_fallback: self.enable_fanout_timeline_db_fallback,
                per_local_user_user_timeline_cache_max: self.per_local_user_user_timeline_cache_max,
                per_remote_user_user_timeline_cache_max: self
                    .per_remote_user_user_timeline_cache_max,
                per_user_home_timeline_cache_max: self.per_user_home_timeline_cache_max,
                per_user_list_timeline_cache_max: self.per_user_list_timeline_cache_max,
                enable_reactions_buffering: self.enable_reactions_buffering,
                notes_per_one_ad: self.notes_per_one_ad,
                silenced_hosts: self.silenced_hosts,
                media_silenced_hosts: self.media_silenced_hosts,
                summaly_proxy: self.summaly_proxy,
                url_preview_enabled: self.url_preview_enabled,
                url_preview_timeout: self.url_preview_timeout,
                url_preview_maximum_content_length: self.url_preview_maximum_content_length,
                url_preview_require_content_length: self.url_preview_require_content_length,
                url_preview_user_agent: self.url_preview_user_agent,
                url_preview_summary_proxy_url: self.url_preview_summary_proxy_url,
                federation: self.federation,
                federation_hosts: self.federation_hosts,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/delete-account`"]
pub mod delete_account {
    #[doc = "`admin/delete-account`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:delete-account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/delete-account";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/delete-account` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:delete-account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/update-user-note`"]
pub mod update_user_note {
    #[doc = "`admin/update-user-note`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:user-note*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        pub text: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "admin/update-user-note";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`admin/update-user-note` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:user-note*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`、`text`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        text: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                text: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                text: self.text,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn text(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                user_id: self.user_id,
                text: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
                text: self
                    .text
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`admin/roles/` 以下のエンドポイント"]
pub mod roles {
    #[doc = "`admin/roles/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/roles/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/roles/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/roles/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/roles/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`admin/roles/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
        #[doc = "`admin/roles/assign`"]
        pub fn assign(&self) -> assign::Call<'a> {
            assign::Call::new(self.client)
        }
        #[doc = "`admin/roles/unassign`"]
        pub fn unassign(&self) -> unassign::Call<'a> {
            unassign::Call::new(self.client)
        }
        #[doc = "`admin/roles/update-default-policies`"]
        pub fn update_default_policies(&self) -> update_default_policies::Call<'a> {
            update_default_policies::Call::new(self.client)
        }
        #[doc = "`admin/roles/users`"]
        pub fn users(&self) -> users::Call<'a> {
            users::Call::new(self.client)
        }
    }
    #[doc = "`admin/roles/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestTarget {
            #[serde(rename = "manual")]
            Manual,
            #[serde(rename = "conditional")]
            Conditional,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/roles/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            pub description: ::std::string::String,
            pub color: ::std::option::Option<::std::string::String>,
            #[serde(rename = "iconUrl")]
            pub icon_url: ::std::option::Option<::std::string::String>,
            pub target: RequestTarget,
            #[serde(rename = "condFormula")]
            pub cond_formula: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            #[serde(rename = "isPublic")]
            pub is_public: bool,
            #[serde(rename = "isModerator")]
            pub is_moderator: bool,
            #[serde(rename = "isAdministrator")]
            pub is_administrator: bool,
            #[serde(
                rename = "isExplorable",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_explorable: ::std::option::Option<bool>,
            #[serde(rename = "asBadge")]
            pub as_badge: bool,
            #[serde(rename = "canEditMembersByModerator")]
            pub can_edit_members_by_moderator: bool,
            #[serde(rename = "displayOrder")]
            pub display_order: f64,
            pub policies: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::Role;
        #[doc = "`admin/roles/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`description`、`color`、`icon_url`、`target`、`cond_formula`、`is_public`、`is_moderator`、`is_administrator`、`as_badge`、`can_edit_members_by_moderator`、`display_order`、`policies`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            color: ::std::option::Option<::std::option::Option<::std::string::String>>,
            icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            target: ::std::option::Option<RequestTarget>,
            cond_formula: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            is_public: ::std::option::Option<bool>,
            is_moderator: ::std::option::Option<bool>,
            is_administrator: ::std::option::Option<bool>,
            is_explorable: ::std::option::Option<bool>,
            as_badge: ::std::option::Option<bool>,
            can_edit_members_by_moderator: ::std::option::Option<bool>,
            display_order: ::std::option::Option<f64>,
            policies: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    color: ::std::option::Option::None,
                    icon_url: ::std::option::Option::None,
                    target: ::std::option::Option::None,
                    cond_formula: ::std::option::Option::None,
                    is_public: ::std::option::Option::None,
                    is_moderator: ::std::option::Option::None,
                    is_administrator: ::std::option::Option::None,
                    is_explorable: ::std::option::Option::None,
                    as_badge: ::std::option::Option::None,
                    can_edit_members_by_moderator: ::std::option::Option::None,
                    display_order: ::std::option::Option::None,
                    policies: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12>
            Call<'a, (S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12)>
        {
            pub fn is_explorable(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_explorable = ::std::option::Option::Some(value.into());
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<
                'a,
                (
                    crate::builder::Set,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<
                'a,
                (
                    S0,
                    crate::builder::Set,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: ::std::option::Option::Some(value.into()),
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn color(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    crate::builder::Set,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: ::std::option::Option::Some(::std::option::Option::Some(value.into())),
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            #[doc = "`color` を `null` として送る。"]
            pub fn color_null(
                self,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    crate::builder::Set,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: ::std::option::Option::Some(::std::option::Option::None),
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn icon_url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    crate::builder::Set,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: ::std::option::Option::Some(::std::option::Option::Some(
                        value.into(),
                    )),
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            #[doc = "`iconUrl` を `null` として送る。"]
            pub fn icon_url_null(
                self,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    crate::builder::Set,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: ::std::option::Option::Some(::std::option::Option::None),
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn target(
                self,
                value: impl ::std::convert::Into<RequestTarget>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    crate::builder::Set,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: ::std::option::Option::Some(value.into()),
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn cond_formula(
                self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    crate::builder::Set,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: ::std::option::Option::Some(value.into()),
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn is_public(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    crate::builder::Set,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: ::std::option::Option::Some(value.into()),
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn is_moderator(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    crate::builder::Set,
                    S8,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: ::std::option::Option::Some(value.into()),
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn is_administrator(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    crate::builder::Set,
                    S9,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: ::std::option::Option::Some(value.into()),
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn as_badge(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    crate::builder::Set,
                    S10,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: ::std::option::Option::Some(value.into()),
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn can_edit_members_by_moderator(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    crate::builder::Set,
                    S11,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: ::std::option::Option::Some(value.into()),
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn display_order(
                self,
                value: impl ::std::convert::Into<f64>,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    crate::builder::Set,
                    S12,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: ::std::option::Option::Some(value.into()),
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn policies(
                self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Call<
                'a,
                (
                    S0,
                    S1,
                    S2,
                    S3,
                    S4,
                    S5,
                    S6,
                    S7,
                    S8,
                    S9,
                    S10,
                    S11,
                    crate::builder::Set,
                ),
            > {
                Call {
                    client: self.client,
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    description: self
                        .description
                        .expect("required parameter is set by the type state"),
                    color: self
                        .color
                        .expect("required parameter is set by the type state"),
                    icon_url: self
                        .icon_url
                        .expect("required parameter is set by the type state"),
                    target: self
                        .target
                        .expect("required parameter is set by the type state"),
                    cond_formula: self
                        .cond_formula
                        .expect("required parameter is set by the type state"),
                    is_public: self
                        .is_public
                        .expect("required parameter is set by the type state"),
                    is_moderator: self
                        .is_moderator
                        .expect("required parameter is set by the type state"),
                    is_administrator: self
                        .is_administrator
                        .expect("required parameter is set by the type state"),
                    is_explorable: self.is_explorable,
                    as_badge: self
                        .as_badge
                        .expect("required parameter is set by the type state"),
                    can_edit_members_by_moderator: self
                        .can_edit_members_by_moderator
                        .expect("required parameter is set by the type state"),
                    display_order: self
                        .display_order
                        .expect("required parameter is set by the type state"),
                    policies: self
                        .policies
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/delete`"]
    pub mod delete {
        #[doc = "`admin/roles/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/roles/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/list`"]
    pub mod list {
        #[doc = "`admin/roles/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Role>;
        #[doc = "`admin/roles/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:roles*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {}
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {};
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/show`"]
    pub mod show {
        #[doc = "`admin/roles/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::Role;
        #[doc = "`admin/roles/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/update`"]
    pub mod update {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestTarget {
            #[serde(rename = "manual")]
            Manual,
            #[serde(rename = "conditional")]
            Conditional,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/roles/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: ::std::option::Option<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub color: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "iconUrl",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub target: ::std::option::Option<RequestTarget>,
            #[serde(
                rename = "condFormula",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub cond_formula: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
            pub is_public: ::std::option::Option<bool>,
            #[serde(
                rename = "isModerator",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_moderator: ::std::option::Option<bool>,
            #[serde(
                rename = "isAdministrator",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_administrator: ::std::option::Option<bool>,
            #[serde(
                rename = "isExplorable",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_explorable: ::std::option::Option<bool>,
            #[serde(rename = "asBadge", default, skip_serializing_if = "Option::is_none")]
            pub as_badge: ::std::option::Option<bool>,
            #[serde(
                rename = "canEditMembersByModerator",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub can_edit_members_by_moderator: ::std::option::Option<bool>,
            #[serde(
                rename = "displayOrder",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub display_order: ::std::option::Option<f64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub policies: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/roles/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            color: ::std::option::Option<::std::option::Option<::std::string::String>>,
            icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
            target: ::std::option::Option<RequestTarget>,
            cond_formula: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            is_public: ::std::option::Option<bool>,
            is_moderator: ::std::option::Option<bool>,
            is_administrator: ::std::option::Option<bool>,
            is_explorable: ::std::option::Option<bool>,
            as_badge: ::std::option::Option<bool>,
            can_edit_members_by_moderator: ::std::option::Option<bool>,
            display_order: ::std::option::Option<f64>,
            policies: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    color: ::std::option::Option::None,
                    icon_url: ::std::option::Option::None,
                    target: ::std::option::Option::None,
                    cond_formula: ::std::option::Option::None,
                    is_public: ::std::option::Option::None,
                    is_moderator: ::std::option::Option::None,
                    is_administrator: ::std::option::Option::None,
                    is_explorable: ::std::option::Option::None,
                    as_badge: ::std::option::Option::None,
                    can_edit_members_by_moderator: ::std::option::Option::None,
                    display_order: ::std::option::Option::None,
                    policies: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn name(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.name = ::std::option::Option::Some(value.into());
                self
            }
            pub fn description(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.description = ::std::option::Option::Some(value.into());
                self
            }
            pub fn color(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.color = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`color` を `null` として送る。"]
            pub fn color_null(mut self) -> Self {
                self.color = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn icon_url(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.icon_url =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`iconUrl` を `null` として送る。"]
            pub fn icon_url_null(mut self) -> Self {
                self.icon_url = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn target(mut self, value: impl ::std::convert::Into<RequestTarget>) -> Self {
                self.target = ::std::option::Option::Some(value.into());
                self
            }
            pub fn cond_formula(
                mut self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Self {
                self.cond_formula = ::std::option::Option::Some(value.into());
                self
            }
            pub fn is_public(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_public = ::std::option::Option::Some(value.into());
                self
            }
            pub fn is_moderator(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_moderator = ::std::option::Option::Some(value.into());
                self
            }
            pub fn is_administrator(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_administrator = ::std::option::Option::Some(value.into());
                self
            }
            pub fn is_explorable(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_explorable = ::std::option::Option::Some(value.into());
                self
            }
            pub fn as_badge(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.as_badge = ::std::option::Option::Some(value.into());
                self
            }
            pub fn can_edit_members_by_moderator(
                mut self,
                value: impl ::std::convert::Into<bool>,
            ) -> Self {
                self.can_edit_members_by_moderator = ::std::option::Option::Some(value.into());
                self
            }
            pub fn display_order(mut self, value: impl ::std::convert::Into<f64>) -> Self {
                self.display_order = ::std::option::Option::Some(value.into());
                self
            }
            pub fn policies(
                mut self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Self {
                self.policies = ::std::option::Option::Some(value.into());
                self
            }
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    description: self.description,
                    color: self.color,
                    icon_url: self.icon_url,
                    target: self.target,
                    cond_formula: self.cond_formula,
                    is_public: self.is_public,
                    is_moderator: self.is_moderator,
                    is_administrator: self.is_administrator,
                    is_explorable: self.is_explorable,
                    as_badge: self.as_badge,
                    can_edit_members_by_moderator: self.can_edit_members_by_moderator,
                    display_order: self.display_order,
                    policies: self.policies,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/assign`"]
    pub mod assign {
        #[doc = "`admin/roles/assign`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
            #[serde(
                rename = "expiresAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expires_at: ::std::option::Option<::std::option::Option<i64>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/assign";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/roles/assign` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::string::String>,
            expires_at: ::std::option::Option<::std::option::Option<i64>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn expires_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.expires_at =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`expiresAt` を `null` として送る。"]
            pub fn expires_at_null(mut self) -> Self {
                self.expires_at = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    user_id: self.user_id,
                    expires_at: self.expires_at,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    role_id: self.role_id,
                    user_id: ::std::option::Option::Some(value.into()),
                    expires_at: self.expires_at,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                    expires_at: self.expires_at,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/unassign`"]
    pub mod unassign {
        #[doc = "`admin/roles/unassign`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/unassign";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/roles/unassign` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    role_id: self.role_id,
                    user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/update-default-policies`"]
    pub mod update_default_policies {
        #[doc = "`admin/roles/update-default-policies`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub policies: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/update-default-policies";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/roles/update-default-policies` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `policies`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            policies: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    policies: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn policies(
                self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    policies: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    policies: self
                        .policies
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/roles/users`"]
    pub mod users {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            pub user: crate::generated::v2024_10_1::types::UserDetailed,
            #[serde(rename = "expiresAt")]
            pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
        }
        #[doc = "`admin/roles/users`"]
        #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:admin:roles*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roleId")]
            pub role_id: ::std::string::String,
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/roles/users";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`admin/roles/users` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:admin:roles*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `role_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            role_id: ::std::option::Option<::std::string::String>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            limit: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    role_id: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.until_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn role_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    role_id: ::std::option::Option::Some(value.into()),
                    since_id: self.since_id,
                    until_id: self.until_id,
                    limit: self.limit,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    role_id: self
                        .role_id
                        .expect("required parameter is set by the type state"),
                    since_id: self.since_id,
                    until_id: self.until_id,
                    limit: self.limit,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`admin/system-webhook/` 以下のエンドポイント"]
pub mod system_webhook {
    #[doc = "`admin/system-webhook/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`admin/system-webhook/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`admin/system-webhook/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`admin/system-webhook/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`admin/system-webhook/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`admin/system-webhook/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
        #[doc = "`admin/system-webhook/test`"]
        pub fn test(&self) -> test::Call<'a> {
            test::Call::new(self.client)
        }
    }
    #[doc = "`admin/system-webhook/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOnItem {
            #[serde(rename = "abuseReport")]
            AbuseReport,
            #[serde(rename = "abuseReportResolved")]
            AbuseReportResolved,
            #[serde(rename = "userCreated")]
            UserCreated,
            #[serde(rename = "inactiveModeratorsWarning")]
            InactiveModeratorsWarning,
            #[serde(rename = "inactiveModeratorsInvitationOnlyChanged")]
            InactiveModeratorsInvitationOnlyChanged,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/system-webhook/create`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "isActive")]
            pub is_active: bool,
            pub name: ::std::string::String,
            pub on: ::std::vec::Vec<RequestOnItem>,
            pub url: ::std::string::String,
            pub secret: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::SystemWebhook;
        #[doc = "`admin/system-webhook/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `is_active`、`name`、`on`、`url`、`secret`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            is_active: ::std::option::Option<bool>,
            name: ::std::option::Option<::std::string::String>,
            on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
            url: ::std::option::Option<::std::string::String>,
            secret: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    is_active: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    on: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    secret: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2, S3, S4> Call<'a, (S0, S1, S2, S3, S4)> {
            pub fn is_active(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4)> {
                Call {
                    client: self.client,
                    is_active: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    on: self.on,
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4)> {
                Call {
                    client: self.client,
                    is_active: self.is_active,
                    name: ::std::option::Option::Some(value.into()),
                    on: self.on,
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn on(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
            ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4)> {
                Call {
                    client: self.client,
                    is_active: self.is_active,
                    name: self.name,
                    on: ::std::option::Option::Some(value.into()),
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4)> {
                Call {
                    client: self.client,
                    is_active: self.is_active,
                    name: self.name,
                    on: self.on,
                    url: ::std::option::Option::Some(value.into()),
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn secret(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set)> {
                Call {
                    client: self.client,
                    is_active: self.is_active,
                    name: self.name,
                    on: self.on,
                    url: self.url,
                    secret: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    is_active: self
                        .is_active
                        .expect("required parameter is set by the type state"),
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    on: self
                        .on
                        .expect("required parameter is set by the type state"),
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    secret: self
                        .secret
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/system-webhook/delete`"]
    pub mod delete {
        #[doc = "`admin/system-webhook/delete`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/system-webhook/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/system-webhook/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOnItem {
            #[serde(rename = "abuseReport")]
            AbuseReport,
            #[serde(rename = "abuseReportResolved")]
            AbuseReportResolved,
            #[serde(rename = "userCreated")]
            UserCreated,
            #[serde(rename = "inactiveModeratorsWarning")]
            InactiveModeratorsWarning,
            #[serde(rename = "inactiveModeratorsInvitationOnlyChanged")]
            InactiveModeratorsInvitationOnlyChanged,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/system-webhook/list`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
            pub is_active: ::std::option::Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::SystemWebhook>;
        #[doc = "`admin/system-webhook/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2024_10_1::Client,
            is_active: ::std::option::Option<bool>,
            on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    is_active: ::std::option::Option::None,
                    on: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn is_active(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_active = ::std::option::Option::Some(value.into());
                self
            }
            pub fn on(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
            ) -> Self {
                self.on = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    is_active: self.is_active,
                    on: self.on,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/system-webhook/show`"]
    pub mod show {
        #[doc = "`admin/system-webhook/show`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::SystemWebhook;
        #[doc = "`admin/system-webhook/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/system-webhook/update`"]
    pub mod update {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOnItem {
            #[serde(rename = "abuseReport")]
            AbuseReport,
            #[serde(rename = "abuseReportResolved")]
            AbuseReportResolved,
            #[serde(rename = "userCreated")]
            UserCreated,
            #[serde(rename = "inactiveModeratorsWarning")]
            InactiveModeratorsWarning,
            #[serde(rename = "inactiveModeratorsInvitationOnlyChanged")]
            InactiveModeratorsInvitationOnlyChanged,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`admin/system-webhook/update`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub id: ::std::string::String,
            #[serde(rename = "isActive")]
            pub is_active: bool,
            pub name: ::std::string::String,
            pub on: ::std::vec::Vec<RequestOnItem>,
            pub url: ::std::string::String,
            pub secret: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2024_10_1::types::SystemWebhook;
        #[doc = "`admin/system-webhook/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `id`、`is_active`、`name`、`on`、`url`、`secret`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2024_10_1::Client,
            id: ::std::option::Option<::std::string::String>,
            is_active: ::std::option::Option<bool>,
            name: ::std::option::Option<::std::string::String>,
            on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
            url: ::std::option::Option<::std::string::String>,
            secret: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    id: ::std::option::Option::None,
                    is_active: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    on: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    secret: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2, S3, S4, S5> Call<'a, (S0, S1, S2, S3, S4, S5)> {
            pub fn id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4, S5)> {
                Call {
                    client: self.client,
                    id: ::std::option::Option::Some(value.into()),
                    is_active: self.is_active,
                    name: self.name,
                    on: self.on,
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn is_active(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4, S5)> {
                Call {
                    client: self.client,
                    id: self.id,
                    is_active: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    on: self.on,
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4, S5)> {
                Call {
                    client: self.client,
                    id: self.id,
                    is_active: self.is_active,
                    name: ::std::option::Option::Some(value.into()),
                    on: self.on,
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn on(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
            ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4, S5)> {
                Call {
                    client: self.client,
                    id: self.id,
                    is_active: self.is_active,
                    name: self.name,
                    on: ::std::option::Option::Some(value.into()),
                    url: self.url,
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set, S5)> {
                Call {
                    client: self.client,
                    id: self.id,
                    is_active: self.is_active,
                    name: self.name,
                    on: self.on,
                    url: ::std::option::Option::Some(value.into()),
                    secret: self.secret,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn secret(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, S1, S2, S3, S4, crate::builder::Set)> {
                Call {
                    client: self.client,
                    id: self.id,
                    is_active: self.is_active,
                    name: self.name,
                    on: self.on,
                    url: self.url,
                    secret: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture
            for Call<
                'a,
                (
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                    crate::builder::Set,
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    id: self
                        .id
                        .expect("required parameter is set by the type state"),
                    is_active: self
                        .is_active
                        .expect("required parameter is set by the type state"),
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    on: self
                        .on
                        .expect("required parameter is set by the type state"),
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    secret: self
                        .secret
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`admin/system-webhook/test`"]
    pub mod test {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestType {
            #[serde(rename = "abuseReport")]
            AbuseReport,
            #[serde(rename = "abuseReportResolved")]
            AbuseReportResolved,
            #[serde(rename = "userCreated")]
            UserCreated,
            #[serde(rename = "inactiveModeratorsWarning")]
            InactiveModeratorsWarning,
            #[serde(rename = "inactiveModeratorsInvitationOnlyChanged")]
            InactiveModeratorsInvitationOnlyChanged,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct RequestOverride {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub secret: ::std::option::Option<::std::string::String>,
        }
        #[doc = "`admin/system-webhook/test`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:system-webhook*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "webhookId")]
            pub webhook_id: ::std::string::String,
            pub r#type: RequestType,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub r#override: ::std::option::Option<RequestOverride>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "admin/system-webhook/test";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`admin/system-webhook/test` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:admin:system-webhook*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `webhook_id`、`type`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2024_10_1::Client,
            webhook_id: ::std::option::Option<::std::string::String>,
            r#type: ::std::option::Option<RequestType>,
            r#override: ::std::option::Option<RequestOverride>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
                Call {
                    client,
                    webhook_id: ::std::option::Option::None,
                    r#type: ::std::option::Option::None,
                    r#override: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn r#override(mut self, value: impl ::std::convert::Into<RequestOverride>) -> Self {
                self.r#override = ::std::option::Option::Some(value.into());
                self
            }
            pub fn webhook_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    webhook_id: ::std::option::Option::Some(value.into()),
                    r#type: self.r#type,
                    r#override: self.r#override,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn r#type(
                self,
                value: impl ::std::convert::Into<RequestType>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    webhook_id: self.webhook_id,
                    r#type: ::std::option::Option::Some(value.into()),
                    r#override: self.r#override,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    webhook_id: self
                        .webhook_id
                        .expect("required parameter is set by the type state"),
                    r#type: self
                        .r#type
                        .expect("required parameter is set by the type state"),
                    r#override: self.r#override,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
