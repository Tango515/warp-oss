use settings::{macros::define_settings_group, SupportedPlatforms, SyncToCloud};

define_settings_group!(LocaleSettings, settings: [
    locale: LocaleSetting {
        type: String,
        default: "zh-CN".to_string(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: false,
        toml_path: "app.locale",
        description: "The display language for Warp UI.",
    },
]);
