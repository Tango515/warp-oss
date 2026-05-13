use std::borrow::Cow;
use std::sync::atomic::{AtomicU8, Ordering};

use settings::Setting as _;
use warpui::SingletonEntity;

use crate::settings::LocaleSettings;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Locale {
    #[default]
    EnUs,
    ZhCn,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MessageId {
    SettingsAbout,
    SettingsAccount,
    SettingsAgents,
    SettingsAppearance,
    SettingsBillingAndUsage,
    SettingsCloudEnvironments,
    SettingsCloudPlatform,
    SettingsCode,
    SettingsCodeEditorAndCodeReview,
    SettingsCodeIndexing,
    SettingsFeatures,
    SettingsKeybindings,
    SettingsLanguage,
    SettingsLocalAIProvider,
    SettingsKnowledge,
    SettingsMcpServers,
    SettingsOzCloudApiKeys,
    SettingsPrivacy,
    SettingsProfiles,
    SettingsReferrals,
    SettingsSharedBlocks,
    SettingsTeams,
    SettingsThirdPartyCliAgents,
    SettingsWarpAgent,
    SettingsWarpDrive,
    SettingsWarpify,
    AboutCopyright,
    LanguagePageDescription,
    LanguagePageOptionEnglish,
    LanguagePageOptionSimplifiedChinese,
    LanguagePageRestartNote,
    LanguagePageSelectorLabel,
    MenuAi,
    MenuBlocks,
    MenuDrive,
    MenuEdit,
    MenuFile,
    MenuHelp,
    MenuTab,
    MenuView,
    MenuWindow,

    // App menu
    AppPreferences,
    AppPrivacyPolicy,
    AppDebug,
    AppSetDefaultTerminal,
    AppLogout,

    // File menu
    FileNewWindow,
    FileNewTerminalTab,
    FileNewAgentTab,
    FileNewFile,
    FileReopenClosedSession,
    FileLaunchConfigurations,
    FileOpenRecent,
    FileSaveNew,
    FileClose,
    FileCloseWindow,

    // Edit menu
    EditUseWarpPrompt,
    EditCopyOnSelect,
    EditSynchronizeInputs,

    // View menu
    ViewToggleMouseReporting,
    ViewToggleScrollReporting,
    ViewToggleFocusReporting,
    ViewCompactMode,

    // Help menu
    HelpSendFeedback,
    HelpWarpDocumentation,
    HelpGitHubIssues,
    HelpWarpSlackCommunity,

    // Dock menu
    DockNewWindow,

    // Debug menu (subset)
    DebugEnableShellDebugMode,
    DebugDisableShellDebugMode,
    DebugEnableInBandGenerators,
    DebugDisableInBandGenerators,
    DebugEnablePtyRecording,
    DebugDisablePtyRecording,
    DebugShowBootstrapBlock,
    DebugHideBootstrapBlock,
    DebugShowInBandCommandBlocks,
    DebugHideInBandCommandBlocks,
    DebugShowSshBlocks,
    DebugHideSshBlocks,
    DebugExportDefaultSettingsCsv,
    DebugToggleNetworkStatus,
    DebugCreateAnonymousUser,

    // Edit menu actions
    EditUndo,
    EditRedo,
    EditCut,
    EditCopy,
    EditPaste,
    EditSelectAll,
    EditClearEditor,
    EditFind,
    EditGoToLine,
    EditFocusInput,
    EditAddNextOccurrence,
    EditAddCursorAbove,
    EditAddCursorBelow,

    // View menu actions
    ViewCommandPalette,
    ViewNavigationPalette,
    ViewLaunchConfigPalette,
    ViewFilesPalette,
    ViewToggleConversationList,
    ViewToggleProjectExplorer,
    ViewToggleGlobalSearch,
    ViewHistory,
    ViewCommandSearch,
    ViewWorkflows,
    ViewIncreaseFontSize,
    ViewDecreaseFontSize,
    ViewResetFontSize,
    ViewIncreaseZoom,
    ViewDecreaseZoom,
    ViewResetZoom,
    ViewToggleWarpDrive,

    // Tab menu actions
    TabRename,
    TabSplitRight,
    TabSplitLeft,
    TabSplitDown,
    TabSplitUp,
    TabMoveLeft,
    TabMoveRight,
    TabCycleNext,
    TabCyclePrev,
    TabActivateNextPane,
    TabActivatePrevPane,
    TabMaximizePane,
    TabClose,
    TabCloseOthers,
    TabCloseRight,

    // Blocks menu actions
    BlocksClear,
    BlocksSelectAbove,
    BlocksSelectBelow,
    BlocksSelectAll,
    BlocksScrollToTop,
    BlocksScrollToBottom,
    BlocksCreatePermalink,
    BlocksToggleBookmark,
    BlocksFindWithin,
    BlocksCopy,
    BlocksCopyCommand,
    BlocksCopyOutput,
    BlocksViewShared,

    // AI menu actions
    AiNewAgentMode,
    AiAttachSelection,
    AiSearch,
    AiOpenFactCollection,
    AiOpenMcpServers,

    // Drive menu actions
    DriveNewPersonalWorkflow,
    DriveNewPersonalNotebook,
    DriveNewPersonalAIPrompt,
    DriveNewPersonalEnvVars,
    DriveNewTeamWorkflow,
    DriveNewTeamNotebook,
    DriveNewTeamAIPrompt,
    DriveNewTeamEnvVars,
    DriveSearch,
    DriveOpenTeamSettings,
    DriveSharePaneContents,
    DriveShareSession,

    // App menu actions
    AppShowAboutWarp,
    AppToggleResourceCenter,
    AppReferAFriend,
    AppShowSettings,
    AppToggleKeybindings,
    AppConfigureKeybindings,
    AppShowAppearance,
    AppViewChangelog,

    // Sync input actions
    SyncAllTerminalInputs,
    SyncTerminalInputsCurrentTab,
    DisableSyncInputs,

    // File menu extras
    FileOpenRepository,
    AddWindow,

    // Settings content - Account page
    SettingsSignUp,
    SettingsFree,
    SettingsComparePlans,
    SettingsUpToDate,
    SettingsVersion,
    SettingsLogOut,
    SettingsReferralCta,
    SettingsContactSupport,
    SettingsManageBilling,
    SettingsCheckForUpdates,
    SettingsRelaunchWarp,
    SettingsUpdateAvailable,
    SettingsUpdating,
    SettingsInstalledUpdate,
    SettingsUpdateManually,
    SettingsSettingsSync,

    // Settings content - Appearance page categories
    AppearanceCategoryThemes,
    AppearanceCategoryIcon,
    AppearanceCategoryWindow,
    AppearanceCategoryInput,
    AppearanceCategoryPanes,
    AppearanceCategoryBlocks,
    AppearanceCategoryText,
    AppearanceCategoryCursor,
    AppearanceCategoryTabs,
    AppearanceCategoryFullscreenApps,

    // Settings content - Appearance page labels
    AppearanceCompactMode,
    AppearanceSyncWithOs,
    AppearanceCursorBlink,
    AppearanceJumpToBottom,
    AppearanceBlockDividers,
    AppearanceDimInactivePanes,
    AppearanceTabIndicators,
    AppearanceFocusFollowsMouse,
    AppearanceZenMode,
    AppearanceVerticalTabLayout,
    AppearanceLigatureRendering,
    AppearanceStartInputTop,
    AppearancePinInputTop,
    AppearancePinInputBottom,
    AppearanceToggleInputMode,
    AppearanceAlwaysShowTabBar,
    AppearanceHideTabBarFullscreen,
    AppearanceShowTabBarOnHover,
    AppearanceHeaderToolbarLayout,
    AppearanceCreateCustomTheme,
    AppearanceWindowOpacity,
    AppearanceWindowBlur,
    AppearanceInputType,
    AppearanceShowCodeReviewButton,
    AppearanceHideCodeReviewButton,

    // Settings content - Appearance dropdown options
    AppearanceInputModeWarp,
    AppearanceInputModeReverse,
    AppearanceInputModeClassic,
    AppearanceThinStrokesNever,
    AppearanceThinStrokesLowDpi,
    AppearanceThinStrokesHighDpi,
    AppearanceThinStrokesAlways,
    AppearanceContrastAlways,
    AppearanceContrastNamedColors,
    AppearanceNever,
    AppearanceAlways,

    // Settings content - Features page categories
    FeaturesCategoryGeneral,
    FeaturesCategorySession,
    FeaturesCategoryKeys,
    FeaturesCategoryTextEditing,
    FeaturesCategoryTerminalInput,
    FeaturesCategoryTerminal,
    FeaturesCategoryNotifications,
    FeaturesCategoryWorkflows,
    FeaturesCategorySystem,

    // Settings content - Features page labels
    FeaturesCopyOnSelect,
    FeaturesLinuxSelectionClipboard,
    FeaturesAutocompleteQuotes,
    FeaturesRestoreWindows,
    FeaturesScrollReporting,
    FeaturesCompletionsWhileTyping,
    FeaturesCommandCorrections,
    FeaturesErrorUnderlining,
    FeaturesSyntaxHighlighting,
    FeaturesAudibleTerminalBell,
    FeaturesAutosuggestions,
    FeaturesAutosuggestionKeybindingHint,
    FeaturesSshWrapper,
    FeaturesLinkTooltip,
    FeaturesVimUnnamedRegister,
    FeaturesVimStatusBar,
    FeaturesWaylandWindowManagement,
    FeaturesConfigureGlobalHotkey,
    FeaturesMakeDefaultTerminal,
    FeaturesLeftOptionMeta,
    FeaturesRightOptionMeta,
    FeaturesLeftAltMeta,
    FeaturesRightAltMeta,
    FeaturesPerformanceWarning,

    // Settings content - Code page
    CodeInitializationSettings,
    CodeCodebaseIndexing,
    CodeCodebaseIndexDescription,
    CodeWarpIndexingIgnoreDescription,
    CodeAutoIndexFeatureName,
    CodeAutoIndexDescription,
    CodeIndexingDisabledAdmin,
    CodeIndexingWorkspaceEnabledAdmin,
    CodeIndexingDisabledGlobalAi,
    CodeCodebaseIndexLimitReached,
    CodeCodebaseIndexingCategory,
    CodeEditorAndReviewCategory,
    CodeIndexNewFolder,
    CodeRestartServer,
    CodeViewLogs,
    CodeSyncing,
    CodeSynced,
    CodeCodebaseTooLarge,
    CodeInstalled,
    CodeInstalling,
    CodeChecking,
    CodeAvailableForDownload,
    CodeAvailable,
    CodeBusy,
    CodeFailed,
    CodeStopped,
    CodeNotRunning,
    CodeInitializedFolders,
    CodeNoFoldersInitialized,
    CodeOpenProjectRules,
    CodeIndexingLabel,
    CodeNoIndexCreated,
    CodeAutoOpenCodeReview,
    CodeAutoOpenCodeReviewDescription,
    CodeShowCodeReviewButton,
    CodeShowCodeReviewButtonDescription,
    CodeShowDiffStats,
    CodeShowDiffStatsDescription,
    CodeProjectExplorer,
    CodeProjectExplorerDescription,
    CodeGlobalFileSearch,
    CodeGlobalFileSearchDescription,

    // Settings content - Privacy page
    PrivacySecretRedaction,
    PrivacySecretRedactionDescription,
    PrivacyCustomSecretRedaction,
    PrivacyCustomSecretDescription,
    PrivacyTelemetryTitle,
    PrivacyTelemetryDescription,
    PrivacyTelemetryFreeTierNote,
    PrivacyDataManagementTitle,
    PrivacyDataManagementDescription,
    PrivacyDataManagementLinkText,
    PrivacyPrivacyPolicyTitle,
    PrivacyPrivacyPolicyLinkText,

    // Settings content - Billing page
    BillingOverview,
    BillingUsageHistory,
    BillingViewOverageDetails,
    BillingEnableOverages,
    BillingOveragesEnabled,
    BillingOveragesNotEnabled,
    BillingAtoZ,
    BillingZtoA,
    BillingUsageAscending,
    BillingUsageDescending,
    BillingEnterpriseUsageCallout,

    // Settings content - Teams page
    TeamsTeamName,
    TeamsLeaveTeam,
    TeamsDeleteTeam,
    TeamsCreate,
    TeamsCreateDescription,
    TeamsDomainsPlaceholder,
    TeamsEmailsPlaceholder,
    TeamsSet,
    TeamsInvite,
    TeamsInviteLinkInstructions,
    TeamsInviteByEmailExpiry,
    TeamsOffline,

    // Settings content - Referrals page
    ReferralsHeader,
    ReferralsAnonymousHeader,
    ReferralsFailedLoad,
    ReferralsCopyLink,
    ReferralsSend,
    ReferralsSending,
    ReferralsLoading,
    ReferralsLinkCopied,
    ReferralsEmailSuccess,
    ReferralsEmailFailure,
    ReferralsRewardIntro,
    ReferralsCurrentReferral,
    ReferralsCurrentReferrals,
    ReferralsCertainRestrictions,

    // Settings content - Keybindings page
    KeybindingsSearchPlaceholder,
    KeybindingsConflictWarning,
    KeybindingsDefault,
    KeybindingsCancel,
    KeybindingsClear,
    KeybindingsSave,

    // Settings content - Other pages
    WarpDriveSignUpRequired,
    WarpifySubshells,
    WarpifySsh,
    McpServersTitle,
    EnvironmentsLastEdited,
    EnvironmentsLastUsed,
    EnvironmentsViewRuns,
    PlatformNewApiKey,
    ShowBlocksCopyLink,
    ShowBlocksDeleting,
    SettingsOpenSettingsFile,
    SettingsDefault,
}

static CURRENT_LOCALE: AtomicU8 = AtomicU8::new(Locale::ZhCn as u8);

impl Locale {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::ZhCn,
            _ => Self::EnUs,
        }
    }
}

pub fn current_locale() -> Locale {
    Locale::from_u8(CURRENT_LOCALE.load(Ordering::Relaxed))
}

pub fn set_current_locale(locale: Locale) {
    CURRENT_LOCALE.store(locale as u8, Ordering::Relaxed);
}

impl Locale {
    pub fn to_string_id(&self) -> &'static str {
        match self {
            Locale::EnUs => "en-US",
            Locale::ZhCn => "zh-CN",
        }
    }

    pub fn from_string_id(id: &str) -> Self {
        match id {
            "zh-CN" => Self::ZhCn,
            _ => Self::EnUs,
        }
    }
}

/// Initialize the locale from persisted settings.
pub fn init_locale_from_settings(ctx: &warpui::AppContext) {
    let locale_str = LocaleSettings::handle(ctx)
        .as_ref(ctx)
        .locale
        .value()
        .clone();
    let locale = Locale::from_string_id(&locale_str);
    set_current_locale(locale);
}

/// Persist the locale to settings.
pub fn persist_locale(ctx: &mut warpui::AppContext, locale: Locale) {
    LocaleSettings::handle(ctx).update(ctx, |settings, ctx| {
        let _ = settings
            .locale
            .set_value(locale.to_string_id().to_string(), ctx);
    });
}

pub fn tr(locale: Locale, message_id: MessageId) -> Cow<'static, str> {
    Cow::Borrowed(match locale {
        Locale::EnUs => en_us(message_id),
        Locale::ZhCn => zh_cn(message_id),
    })
}

fn en_us(message_id: MessageId) -> &'static str {
    match message_id {
        MessageId::SettingsAbout => "About",
        MessageId::SettingsAccount => "Account",
        MessageId::SettingsAgents => "Agents",
        MessageId::SettingsAppearance => "Appearance",
        MessageId::SettingsBillingAndUsage => "Billing and usage",
        MessageId::SettingsCloudEnvironments => "Environments",
        MessageId::SettingsCloudPlatform => "Cloud platform",
        MessageId::SettingsCode => "Code",
        MessageId::SettingsCodeEditorAndCodeReview => "Editor and Code Review",
        MessageId::SettingsCodeIndexing => "Indexing and projects",
        MessageId::SettingsFeatures => "Features",
        MessageId::SettingsKeybindings => "Keyboard shortcuts",
        MessageId::SettingsLanguage => "Language",
        MessageId::SettingsLocalAIProvider => "Local AI Provider",
        MessageId::SettingsKnowledge => "Knowledge",
        MessageId::SettingsMcpServers => "MCP Servers",
        MessageId::SettingsOzCloudApiKeys => "Oz Cloud API Keys",
        MessageId::SettingsPrivacy => "Privacy",
        MessageId::SettingsProfiles => "Profiles",
        MessageId::SettingsReferrals => "Referrals",
        MessageId::SettingsSharedBlocks => "Shared blocks",
        MessageId::SettingsTeams => "Teams",
        MessageId::SettingsThirdPartyCliAgents => "Third party CLI agents",
        MessageId::SettingsWarpAgent => "Warp Agent",
        MessageId::SettingsWarpDrive => "Warp Drive",
        MessageId::SettingsWarpify => "Warpify",
        MessageId::AboutCopyright => "Copyright 2026 Warp",
        MessageId::LanguagePageDescription => "Choose the display language for Warp.",
        MessageId::LanguagePageOptionEnglish => "English",
        MessageId::LanguagePageOptionSimplifiedChinese => "Simplified Chinese",
        MessageId::LanguagePageRestartNote => "Some text may update after reopening the window.",
        MessageId::LanguagePageSelectorLabel => "App language",
        MessageId::MenuAi => "AI",
        MessageId::MenuBlocks => "Blocks",
        MessageId::MenuDrive => "Drive",
        MessageId::MenuEdit => "Edit",
        MessageId::MenuFile => "File",
        MessageId::MenuHelp => "Help",
        MessageId::MenuTab => "Tab",
        MessageId::MenuView => "View",
        MessageId::MenuWindow => "Window",

        // App menu
        MessageId::AppPreferences => "Preferences",
        MessageId::AppPrivacyPolicy => "Privacy Policy...",
        MessageId::AppDebug => "Debug",
        MessageId::AppSetDefaultTerminal => "Set Warp as Default Terminal",
        MessageId::AppLogout => "Log out",

        // File menu
        MessageId::FileNewWindow => "New Window",
        MessageId::FileNewTerminalTab => "New Terminal Tab",
        MessageId::FileNewAgentTab => "New Agent Tab",
        MessageId::FileNewFile => "New File",
        MessageId::FileReopenClosedSession => "Reopen closed session",
        MessageId::FileLaunchConfigurations => "Launch Configurations",
        MessageId::FileOpenRecent => "Open Recent",
        MessageId::FileSaveNew => "Save New...",
        MessageId::FileClose => "Close Current Session",
        MessageId::FileCloseWindow => "Close Window",

        // Edit menu
        MessageId::EditUseWarpPrompt => "Use Warp's Prompt",
        MessageId::EditCopyOnSelect => "Copy on Select within the Terminal",
        MessageId::EditSynchronizeInputs => "Synchronize Inputs",

        // View menu
        MessageId::ViewToggleMouseReporting => "Toggle Mouse Reporting",
        MessageId::ViewToggleScrollReporting => "Toggle Scroll Reporting",
        MessageId::ViewToggleFocusReporting => "Toggle Focus Reporting",
        MessageId::ViewCompactMode => "Compact Mode",

        // Help menu
        MessageId::HelpSendFeedback => "Send Feedback...",
        MessageId::HelpWarpDocumentation => "Warp Documentation...",
        MessageId::HelpGitHubIssues => "GitHub Issues...",
        MessageId::HelpWarpSlackCommunity => "Warp Slack Community...",

        // Dock menu
        MessageId::DockNewWindow => "New Window",

        // Debug menu
        MessageId::DebugEnableShellDebugMode => "Enable Shell Debug Mode (-x) for New Sessions",
        MessageId::DebugDisableShellDebugMode => "Disable Shell Debug Mode (-x) for New Sessions",
        MessageId::DebugEnableInBandGenerators => "Enable In-band Generators for New Sessions",
        MessageId::DebugDisableInBandGenerators => "Disable in-band generators for new sessions",
        MessageId::DebugEnablePtyRecording => "Enable PTY Recording Mode (warp.pty.recording)",
        MessageId::DebugDisablePtyRecording => "Disable PTY Recording Mode (warp.pty.recording)",
        MessageId::DebugShowBootstrapBlock => "Show Initialization Block",
        MessageId::DebugHideBootstrapBlock => "Hide Initialization Block",
        MessageId::DebugShowInBandCommandBlocks => "Show In-band Command Blocks",
        MessageId::DebugHideInBandCommandBlocks => "Hide In-band Command Blocks",
        MessageId::DebugShowSshBlocks => "Show Warpified SSH Blocks",
        MessageId::DebugHideSshBlocks => "Hide Warpified SSH Blocks",
        MessageId::DebugExportDefaultSettingsCsv => "Export Default Settings as CSV to home dir",
        MessageId::DebugToggleNetworkStatus => "Manually Toggle Network Status",
        MessageId::DebugCreateAnonymousUser => "Create anonymous user",

        // Edit menu actions
        MessageId::EditUndo => "Undo",
        MessageId::EditRedo => "Redo",
        MessageId::EditCut => "Cut",
        MessageId::EditCopy => "Copy",
        MessageId::EditPaste => "Paste",
        MessageId::EditSelectAll => "Select All",
        MessageId::EditClearEditor => "Clear Editor",
        MessageId::EditFind => "Find in Terminal",
        MessageId::EditGoToLine => "Go to Line",
        MessageId::EditFocusInput => "Focus Input",
        MessageId::EditAddNextOccurrence => "Add Next Occurrence",
        MessageId::EditAddCursorAbove => "Add Cursor Above",
        MessageId::EditAddCursorBelow => "Add Cursor Below",

        // View menu actions
        MessageId::ViewCommandPalette => "Command Palette",
        MessageId::ViewNavigationPalette => "Navigation Palette",
        MessageId::ViewLaunchConfigPalette => "Launch Configuration Palette",
        MessageId::ViewFilesPalette => "Files Palette",
        MessageId::ViewToggleConversationList => "Toggle Conversation List",
        MessageId::ViewToggleProjectExplorer => "Toggle Project Explorer",
        MessageId::ViewToggleGlobalSearch => "Toggle Global Search",
        MessageId::ViewHistory => "History",
        MessageId::ViewCommandSearch => "Command Search",
        MessageId::ViewWorkflows => "Workflows",
        MessageId::ViewIncreaseFontSize => "Increase Font Size",
        MessageId::ViewDecreaseFontSize => "Decrease Font Size",
        MessageId::ViewResetFontSize => "Reset Font Size",
        MessageId::ViewIncreaseZoom => "Increase Zoom",
        MessageId::ViewDecreaseZoom => "Decrease Zoom",
        MessageId::ViewResetZoom => "Reset Zoom",
        MessageId::ViewToggleWarpDrive => "Toggle Warp Drive",

        // Tab menu actions
        MessageId::TabRename => "Rename Tab",
        MessageId::TabSplitRight => "Split Right",
        MessageId::TabSplitLeft => "Split Left",
        MessageId::TabSplitDown => "Split Down",
        MessageId::TabSplitUp => "Split Up",
        MessageId::TabMoveLeft => "Move Tab Left",
        MessageId::TabMoveRight => "Move Tab Right",
        MessageId::TabCycleNext => "Next Tab",
        MessageId::TabCyclePrev => "Previous Tab",
        MessageId::TabActivateNextPane => "Activate Next Pane",
        MessageId::TabActivatePrevPane => "Activate Previous Pane",
        MessageId::TabMaximizePane => "Maximize Pane",
        MessageId::TabClose => "Close Tab",
        MessageId::TabCloseOthers => "Close Other Tabs",
        MessageId::TabCloseRight => "Close Tabs to the Right",

        // Blocks menu actions
        MessageId::BlocksClear => "Clear Blocks",
        MessageId::BlocksSelectAbove => "Select Block Above",
        MessageId::BlocksSelectBelow => "Select Block Below",
        MessageId::BlocksSelectAll => "Select All Blocks",
        MessageId::BlocksScrollToTop => "Scroll to Top of Selected Blocks",
        MessageId::BlocksScrollToBottom => "Scroll to Bottom of Selected Blocks",
        MessageId::BlocksCreatePermalink => "Create Block Permalink",
        MessageId::BlocksToggleBookmark => "Toggle Bookmark",
        MessageId::BlocksFindWithin => "Find Within Block",
        MessageId::BlocksCopy => "Copy Block",
        MessageId::BlocksCopyCommand => "Copy Block Command",
        MessageId::BlocksCopyOutput => "Copy Block Output",
        MessageId::BlocksViewShared => "View Shared Blocks",

        // AI menu actions
        MessageId::AiNewAgentMode => "New Agent Mode Pane",
        MessageId::AiAttachSelection => "Attach Selection as Context",
        MessageId::AiSearch => "AI Search",
        MessageId::AiOpenFactCollection => "Open Fact Collection",
        MessageId::AiOpenMcpServers => "Open MCP Servers",

        // Drive menu actions
        MessageId::DriveNewPersonalWorkflow => "New Personal Workflow",
        MessageId::DriveNewPersonalNotebook => "New Personal Notebook",
        MessageId::DriveNewPersonalAIPrompt => "New Personal AI Prompt",
        MessageId::DriveNewPersonalEnvVars => "New Personal Env Variables",
        MessageId::DriveNewTeamWorkflow => "New Team Workflow",
        MessageId::DriveNewTeamNotebook => "New Team Notebook",
        MessageId::DriveNewTeamAIPrompt => "New Team AI Prompt",
        MessageId::DriveNewTeamEnvVars => "New Team Env Variables",
        MessageId::DriveSearch => "Search Drive",
        MessageId::DriveOpenTeamSettings => "Open Team Settings",
        MessageId::DriveSharePaneContents => "Share Pane Contents",
        MessageId::DriveShareSession => "Share Current Session",

        // App menu actions
        MessageId::AppShowAboutWarp => "About Warp",
        MessageId::AppToggleResourceCenter => "Resource Center",
        MessageId::AppReferAFriend => "Refer a Friend",
        MessageId::AppShowSettings => "Settings",
        MessageId::AppToggleKeybindings => "Keybindings",
        MessageId::AppConfigureKeybindings => "Configure Keybindings",
        MessageId::AppShowAppearance => "Appearance",
        MessageId::AppViewChangelog => "View Changelog",

        // Sync input actions
        MessageId::SyncAllTerminalInputs => "Sync All Terminal Inputs in All Tabs",
        MessageId::SyncTerminalInputsCurrentTab => "Sync Terminal Inputs in Current Tab",
        MessageId::DisableSyncInputs => "Disable Sync Inputs",

        // File menu extras
        MessageId::FileOpenRepository => "Open Repository",
        MessageId::AddWindow => "Add Window",

        // Settings content - Account page
        MessageId::SettingsSignUp => "Sign up",
        MessageId::SettingsFree => "Free",
        MessageId::SettingsComparePlans => "Compare plans",
        MessageId::SettingsUpToDate => "Up to date",
        MessageId::SettingsVersion => "Version",
        MessageId::SettingsLogOut => "Log out",
        MessageId::SettingsReferralCta => "Earn rewards by sharing Warp with friends & colleagues",
        MessageId::SettingsContactSupport => "Contact support",
        MessageId::SettingsManageBilling => "Manage billing",
        MessageId::SettingsCheckForUpdates => "Check for updates",
        MessageId::SettingsRelaunchWarp => "Relaunch Warp",
        MessageId::SettingsUpdateAvailable => "Update available",
        MessageId::SettingsUpdating => "Updating...",
        MessageId::SettingsInstalledUpdate => "Installed update",
        MessageId::SettingsUpdateManually => "Update Warp manually",
        MessageId::SettingsSettingsSync => "Settings sync",

        // Settings content - Appearance page categories
        MessageId::AppearanceCategoryThemes => "Themes",
        MessageId::AppearanceCategoryIcon => "Icon",
        MessageId::AppearanceCategoryWindow => "Window",
        MessageId::AppearanceCategoryInput => "Input",
        MessageId::AppearanceCategoryPanes => "Panes",
        MessageId::AppearanceCategoryBlocks => "Blocks",
        MessageId::AppearanceCategoryText => "Text",
        MessageId::AppearanceCategoryCursor => "Cursor",
        MessageId::AppearanceCategoryTabs => "Tabs",
        MessageId::AppearanceCategoryFullscreenApps => "Full-screen Apps",

        // Settings content - Appearance page labels
        MessageId::AppearanceCompactMode => "compact mode",
        MessageId::AppearanceSyncWithOs => "themes: sync with OS",
        MessageId::AppearanceCursorBlink => "cursor blink",
        MessageId::AppearanceJumpToBottom => "jump to bottom of block button",
        MessageId::AppearanceBlockDividers => "block dividers",
        MessageId::AppearanceDimInactivePanes => "dim inactive panes",
        MessageId::AppearanceTabIndicators => "tab indicators",
        MessageId::AppearanceFocusFollowsMouse => "focus follows mouse",
        MessageId::AppearanceZenMode => "zen mode",
        MessageId::AppearanceVerticalTabLayout => "vertical tab layout",
        MessageId::AppearanceLigatureRendering => "ligature rendering",
        MessageId::AppearanceStartInputTop => "Start Input at the Top",
        MessageId::AppearancePinInputTop => "Pin Input to the Top",
        MessageId::AppearancePinInputBottom => "Pin Input to the Bottom",
        MessageId::AppearanceToggleInputMode => "Toggle Input Mode (Warp/Classic)",
        MessageId::AppearanceAlwaysShowTabBar => "Always show tab bar",
        MessageId::AppearanceHideTabBarFullscreen => "Hide tab bar if fullscreen",
        MessageId::AppearanceShowTabBarOnHover => "Only show tab bar on hover",
        MessageId::AppearanceHeaderToolbarLayout => "Header toolbar layout",
        MessageId::AppearanceCreateCustomTheme => "Create your own custom theme",
        MessageId::AppearanceWindowOpacity => "Window Opacity",
        MessageId::AppearanceWindowBlur => "Use Window Blur",
        MessageId::AppearanceInputType => "Input type",
        MessageId::AppearanceShowCodeReviewButton => "Show code review button in tab bar",
        MessageId::AppearanceHideCodeReviewButton => "Hide code review button in tab bar",

        // Settings content - Appearance dropdown options
        MessageId::AppearanceInputModeWarp => "Pin to the bottom (Warp mode)",
        MessageId::AppearanceInputModeReverse => "Pin to the top (Reverse mode)",
        MessageId::AppearanceInputModeClassic => "Start at the top (Classic mode)",
        MessageId::AppearanceThinStrokesNever => "Never",
        MessageId::AppearanceThinStrokesLowDpi => "On low-DPI displays",
        MessageId::AppearanceThinStrokesHighDpi => "On high-DPI displays",
        MessageId::AppearanceThinStrokesAlways => "Always",
        MessageId::AppearanceContrastAlways => "Always",
        MessageId::AppearanceContrastNamedColors => "Only for named colors",
        MessageId::AppearanceNever => "Never",
        MessageId::AppearanceAlways => "Always",

        // Settings content - Features page categories
        MessageId::FeaturesCategoryGeneral => "General",
        MessageId::FeaturesCategorySession => "Session",
        MessageId::FeaturesCategoryKeys => "Keys",
        MessageId::FeaturesCategoryTextEditing => "Text Editing",
        MessageId::FeaturesCategoryTerminalInput => "Terminal Input",
        MessageId::FeaturesCategoryTerminal => "Terminal",
        MessageId::FeaturesCategoryNotifications => "Notifications",
        MessageId::FeaturesCategoryWorkflows => "Workflows",
        MessageId::FeaturesCategorySystem => "System",

        // Settings content - Features page labels
        MessageId::FeaturesCopyOnSelect => "Copy on select within the terminal",
        MessageId::FeaturesLinuxSelectionClipboard => "Linux selection clipboard",
        MessageId::FeaturesAutocompleteQuotes => "Autocomplete quotes, parentheses, and brackets",
        MessageId::FeaturesRestoreWindows => "Restore windows, tabs, and panes on startup",
        MessageId::FeaturesScrollReporting => "Scroll reporting",
        MessageId::FeaturesCompletionsWhileTyping => "Completions while typing",
        MessageId::FeaturesCommandCorrections => "Command corrections",
        MessageId::FeaturesErrorUnderlining => "Error underlining",
        MessageId::FeaturesSyntaxHighlighting => "Syntax highlighting",
        MessageId::FeaturesAudibleTerminalBell => "Audible terminal bell",
        MessageId::FeaturesAutosuggestions => "Autosuggestions",
        MessageId::FeaturesAutosuggestionKeybindingHint => "Autosuggestion keybinding hint",
        MessageId::FeaturesSshWrapper => "Warp SSH wrapper",
        MessageId::FeaturesLinkTooltip => "Show tooltip on click on links",
        MessageId::FeaturesVimUnnamedRegister => "Vim unnamed register as system clipboard",
        MessageId::FeaturesVimStatusBar => "Vim status bar",
        MessageId::FeaturesWaylandWindowManagement => "Wayland for window management",
        MessageId::FeaturesConfigureGlobalHotkey => "Configure Global Hotkey",
        MessageId::FeaturesMakeDefaultTerminal => "Make Warp the default terminal",
        MessageId::FeaturesLeftOptionMeta => "Left Option key is Meta",
        MessageId::FeaturesRightOptionMeta => "Right Option key is Meta",
        MessageId::FeaturesLeftAltMeta => "Left Alt key is Meta",
        MessageId::FeaturesRightAltMeta => "Right Alt key is Meta",
        MessageId::FeaturesPerformanceWarning => "Setting the limit above 100k lines may impact performance.",

        // Settings content - Code page
        MessageId::CodeInitializationSettings => "Initialization Settings",
        MessageId::CodeCodebaseIndexing => "Codebase indexing",
        MessageId::CodeCodebaseIndexDescription => "Warp can automatically index code repositories as you navigate them, helping agents quickly understand context and provide solutions. Code is never stored on the server.",
        MessageId::CodeWarpIndexingIgnoreDescription => "To exclude specific files or directories from indexing, add them to the .warpindexingignore file in your repository directory.",
        MessageId::CodeAutoIndexFeatureName => "Index new folders by default",
        MessageId::CodeAutoIndexDescription => "When set to true, Warp will automatically index code repositories as you navigate them.",
        MessageId::CodeIndexingDisabledAdmin => "Team admins have disabled codebase indexing.",
        MessageId::CodeIndexingWorkspaceEnabledAdmin => "Team admins have enabled codebase indexing.",
        MessageId::CodeIndexingDisabledGlobalAi => "AI Features must be enabled to use codebase indexing.",
        MessageId::CodeCodebaseIndexLimitReached => "You have reached the maximum number of codebase indices for your plan.",
        MessageId::CodeCodebaseIndexingCategory => "Codebase Indexing",
        MessageId::CodeEditorAndReviewCategory => "Code Editor and Review",
        MessageId::CodeIndexNewFolder => "Index new folder",
        MessageId::CodeRestartServer => "Restart server",
        MessageId::CodeViewLogs => "View logs",
        MessageId::CodeSyncing => "Syncing...",
        MessageId::CodeSynced => "Synced",
        MessageId::CodeCodebaseTooLarge => "Codebase too large",
        MessageId::CodeInstalled => "Installed",
        MessageId::CodeInstalling => "Installing...",
        MessageId::CodeChecking => "Checking...",
        MessageId::CodeAvailableForDownload => "Available for download",
        MessageId::CodeAvailable => "Available",
        MessageId::CodeBusy => "Busy",
        MessageId::CodeFailed => "Failed",
        MessageId::CodeStopped => "Stopped",
        MessageId::CodeNotRunning => "Not running",
        MessageId::CodeInitializedFolders => "Initialized / indexed folders",
        MessageId::CodeNoFoldersInitialized => "No folders have been initialized yet.",
        MessageId::CodeOpenProjectRules => "Open project rules",
        MessageId::CodeIndexingLabel => "INDEXING",
        MessageId::CodeNoIndexCreated => "No index created",
        MessageId::CodeAutoOpenCodeReview => "Auto open code review panel",
        MessageId::CodeAutoOpenCodeReviewDescription => "When this setting is on, the code review panel will open on the first accepted diff of a conversation",
        MessageId::CodeShowCodeReviewButton => "Show code review button",
        MessageId::CodeShowCodeReviewButtonDescription => "Show a button in the top right of the window to toggle the code review panel.",
        MessageId::CodeShowDiffStats => "Show diff stats on code review button",
        MessageId::CodeShowDiffStatsDescription => "Show lines added and removed counts on the code review button.",
        MessageId::CodeProjectExplorer => "Project explorer",
        MessageId::CodeProjectExplorerDescription => "Adds an IDE-style project explorer / file tree to the left side tools panel.",
        MessageId::CodeGlobalFileSearch => "Global file search",
        MessageId::CodeGlobalFileSearchDescription => "Adds global file search to the left side tools panel.",

        // Settings content - Privacy page
        MessageId::PrivacySecretRedaction => "Secret redaction",
        MessageId::PrivacySecretRedactionDescription => "When this setting is enabled, Warp will scan blocks, the contents of Warp Drive objects, and Oz prompts for potential sensitive information and prevent saving or sending this data to any servers.",
        MessageId::PrivacyCustomSecretRedaction => "Custom secret redaction",
        MessageId::PrivacyCustomSecretDescription => "Use regex to define additional secrets or data you'd like to redact. This will take effect when the next command runs.",
        MessageId::PrivacyTelemetryTitle => "Help improve Warp",
        MessageId::PrivacyTelemetryDescription => "App analytics help us make the product better for you. We may collect certain console interactions to improve Warp's AI capabilities.",
        MessageId::PrivacyTelemetryFreeTierNote => "On the free tier, analytics must be enabled to use AI features.",
        MessageId::PrivacyDataManagementTitle => "Manage your data",
        MessageId::PrivacyDataManagementDescription => "At any time, you may choose to delete your Warp account permanently. You will no longer be able to use Warp.",
        MessageId::PrivacyDataManagementLinkText => "Visit the data management page",
        MessageId::PrivacyPrivacyPolicyTitle => "Privacy policy",
        MessageId::PrivacyPrivacyPolicyLinkText => "Read Warp's privacy policy",

        // Settings content - Billing page
        MessageId::BillingOverview => "Overview",
        MessageId::BillingUsageHistory => "Usage History",
        MessageId::BillingViewOverageDetails => "View overage details",
        MessageId::BillingEnableOverages => "Enable overages",
        MessageId::BillingOveragesEnabled => "Overages enabled",
        MessageId::BillingOveragesNotEnabled => "Overages not enabled",
        MessageId::BillingAtoZ => "A to Z",
        MessageId::BillingZtoA => "Z to A",
        MessageId::BillingUsageAscending => "Usage ascending",
        MessageId::BillingUsageDescending => "Usage descending",
        MessageId::BillingEnterpriseUsageCallout => "Contact your team admin for usage details.",

        // Settings content - Teams page
        MessageId::TeamsTeamName => "Team name",
        MessageId::TeamsLeaveTeam => "Leave team",
        MessageId::TeamsDeleteTeam => "Delete team",
        MessageId::TeamsCreate => "Create",
        MessageId::TeamsCreateDescription => "Create a new team to collaborate with your colleagues.",
        MessageId::TeamsDomainsPlaceholder => "Add domains",
        MessageId::TeamsEmailsPlaceholder => "Add email addresses",
        MessageId::TeamsSet => "Set",
        MessageId::TeamsInvite => "Invite",
        MessageId::TeamsInviteLinkInstructions => "Share this link with others to invite them to your team.",
        MessageId::TeamsInviteByEmailExpiry => "Invitations expire after 7 days.",
        MessageId::TeamsOffline => "You appear to be offline.",

        // Settings content - Referrals page
        MessageId::ReferralsHeader => "Invite a friend to Warp",
        MessageId::ReferralsAnonymousHeader => "Create an account to start inviting friends",
        MessageId::ReferralsFailedLoad => "Failed to load referral data.",
        MessageId::ReferralsCopyLink => "Copy link",
        MessageId::ReferralsSend => "Send",
        MessageId::ReferralsSending => "Sending...",
        MessageId::ReferralsLoading => "Loading...",
        MessageId::ReferralsLinkCopied => "Link copied!",
        MessageId::ReferralsEmailSuccess => "Invitation sent!",
        MessageId::ReferralsEmailFailure => "Failed to send invitation.",
        MessageId::ReferralsRewardIntro => "You'll both earn rewards when your friend joins Warp.",
        MessageId::ReferralsCurrentReferral => "You have 1 active referral.",
        MessageId::ReferralsCurrentReferrals => "You have {count} active referrals.",
        MessageId::ReferralsCertainRestrictions => "Certain restrictions apply.",

        // Settings content - Keybindings page
        MessageId::KeybindingsSearchPlaceholder => "Search by name or by keys",
        MessageId::KeybindingsConflictWarning => "This shortcut conflicts with other keybinds",
        MessageId::KeybindingsDefault => "Default",
        MessageId::KeybindingsCancel => "Cancel",
        MessageId::KeybindingsClear => "Clear",
        MessageId::KeybindingsSave => "Save",

        // Settings content - Other pages
        MessageId::WarpDriveSignUpRequired => "To use Warp Drive, please create an account.",
        MessageId::WarpifySubshells => "Subshells",
        MessageId::WarpifySsh => "SSH",
        MessageId::McpServersTitle => "MCP Servers",
        MessageId::EnvironmentsLastEdited => "Last edited:",
        MessageId::EnvironmentsLastUsed => "Last used:",
        MessageId::EnvironmentsViewRuns => "View my runs",
        MessageId::PlatformNewApiKey => "New API key",
        MessageId::ShowBlocksCopyLink => "Copy link",
        MessageId::ShowBlocksDeleting => "Deleting...",
        MessageId::SettingsOpenSettingsFile => "Open settings file",
        MessageId::SettingsDefault => "Default",
    }
}

fn zh_cn(message_id: MessageId) -> &'static str {
    match message_id {
        MessageId::SettingsAbout => "关于",
        MessageId::SettingsAccount => "账户",
        MessageId::SettingsAgents => "智能体",
        MessageId::SettingsAppearance => "外观",
        MessageId::SettingsBillingAndUsage => "账单和用量",
        MessageId::SettingsCloudEnvironments => "环境",
        MessageId::SettingsCloudPlatform => "云平台",
        MessageId::SettingsCode => "代码",
        MessageId::SettingsCodeEditorAndCodeReview => "编辑器和代码审查",
        MessageId::SettingsCodeIndexing => "索引和项目",
        MessageId::SettingsFeatures => "功能",
        MessageId::SettingsKeybindings => "键盘快捷键",
        MessageId::SettingsLanguage => "语言",
        MessageId::SettingsLocalAIProvider => "本地 AI Provider",
        MessageId::SettingsKnowledge => "知识",
        MessageId::SettingsMcpServers => "MCP 服务器",
        MessageId::SettingsOzCloudApiKeys => "Oz Cloud API 密钥",
        MessageId::SettingsPrivacy => "隐私",
        MessageId::SettingsProfiles => "配置文件",
        MessageId::SettingsReferrals => "邀请奖励",
        MessageId::SettingsSharedBlocks => "共享块",
        MessageId::SettingsTeams => "团队",
        MessageId::SettingsThirdPartyCliAgents => "第三方 CLI 智能体",
        MessageId::SettingsWarpAgent => "Warp 智能体",
        MessageId::SettingsWarpDrive => "Warp Drive",
        MessageId::SettingsWarpify => "Warpify",
        MessageId::AboutCopyright => "版权所有 2026 Warp",
        MessageId::LanguagePageDescription => "选择 Warp 的显示语言。",
        MessageId::LanguagePageOptionEnglish => "English",
        MessageId::LanguagePageOptionSimplifiedChinese => "简体中文",
        MessageId::LanguagePageRestartNote => "部分文字可能需要重新打开窗口后更新。",
        MessageId::LanguagePageSelectorLabel => "应用语言",
        MessageId::MenuAi => "AI",
        MessageId::MenuBlocks => "块",
        MessageId::MenuDrive => "云盘",
        MessageId::MenuEdit => "编辑",
        MessageId::MenuFile => "文件",
        MessageId::MenuHelp => "帮助",
        MessageId::MenuTab => "标签页",
        MessageId::MenuView => "视图",
        MessageId::MenuWindow => "窗口",

        // App menu
        MessageId::AppPreferences => "偏好设置",
        MessageId::AppPrivacyPolicy => "隐私政策...",
        MessageId::AppDebug => "调试",
        MessageId::AppSetDefaultTerminal => "将 Warp 设为默认终端",
        MessageId::AppLogout => "退出登录",

        // File menu
        MessageId::FileNewWindow => "新建窗口",
        MessageId::FileNewTerminalTab => "新建终端标签页",
        MessageId::FileNewAgentTab => "新建智能体标签页",
        MessageId::FileNewFile => "新建文件",
        MessageId::FileReopenClosedSession => "重新打开已关闭的会话",
        MessageId::FileLaunchConfigurations => "启动配置",
        MessageId::FileOpenRecent => "最近打开",
        MessageId::FileSaveNew => "另存为...",
        MessageId::FileClose => "关闭当前会话",
        MessageId::FileCloseWindow => "关闭窗口",

        // Edit menu
        MessageId::EditUseWarpPrompt => "使用 Warp 提示符",
        MessageId::EditCopyOnSelect => "在终端中选中时复制",
        MessageId::EditSynchronizeInputs => "同步输入",

        // View menu
        MessageId::ViewToggleMouseReporting => "切换鼠标上报",
        MessageId::ViewToggleScrollReporting => "切换滚动上报",
        MessageId::ViewToggleFocusReporting => "切换焦点上报",
        MessageId::ViewCompactMode => "紧凑模式",

        // Help menu
        MessageId::HelpSendFeedback => "发送反馈...",
        MessageId::HelpWarpDocumentation => "Warp 文档...",
        MessageId::HelpGitHubIssues => "GitHub Issues...",
        MessageId::HelpWarpSlackCommunity => "Warp Slack 社区...",

        // Dock menu
        MessageId::DockNewWindow => "新建窗口",

        // Debug menu
        MessageId::DebugEnableShellDebugMode => "为新会话启用 Shell 调试模式 (-x)",
        MessageId::DebugDisableShellDebugMode => "为新会话禁用 Shell 调试模式 (-x)",
        MessageId::DebugEnableInBandGenerators => "为新会话启用带内生成器",
        MessageId::DebugDisableInBandGenerators => "为新会话禁用带内生成器",
        MessageId::DebugEnablePtyRecording => "启用 PTY 录制模式 (warp.pty.recording)",
        MessageId::DebugDisablePtyRecording => "禁用 PTY 录制模式 (warp.pty.recording)",
        MessageId::DebugShowBootstrapBlock => "显示初始化块",
        MessageId::DebugHideBootstrapBlock => "隐藏初始化块",
        MessageId::DebugShowInBandCommandBlocks => "显示带内命令块",
        MessageId::DebugHideInBandCommandBlocks => "隐藏带内命令块",
        MessageId::DebugShowSshBlocks => "显示 Warp 化的 SSH 块",
        MessageId::DebugHideSshBlocks => "隐藏 Warp 化的 SSH 块",
        MessageId::DebugExportDefaultSettingsCsv => "将默认设置导出为 CSV 到主目录",
        MessageId::DebugToggleNetworkStatus => "手动切换网络状态",
        MessageId::DebugCreateAnonymousUser => "创建匿名用户",

        // Edit menu actions
        MessageId::EditUndo => "撤销",
        MessageId::EditRedo => "重做",
        MessageId::EditCut => "剪切",
        MessageId::EditCopy => "复制",
        MessageId::EditPaste => "粘贴",
        MessageId::EditSelectAll => "全选",
        MessageId::EditClearEditor => "清除编辑器",
        MessageId::EditFind => "在终端中查找",
        MessageId::EditGoToLine => "跳转到行",
        MessageId::EditFocusInput => "聚焦输入",
        MessageId::EditAddNextOccurrence => "添加下一个匹配项",
        MessageId::EditAddCursorAbove => "在上方添加光标",
        MessageId::EditAddCursorBelow => "在下方添加光标",

        // View menu actions
        MessageId::ViewCommandPalette => "命令面板",
        MessageId::ViewNavigationPalette => "导航面板",
        MessageId::ViewLaunchConfigPalette => "启动配置面板",
        MessageId::ViewFilesPalette => "文件面板",
        MessageId::ViewToggleConversationList => "切换对话列表",
        MessageId::ViewToggleProjectExplorer => "切换项目浏览器",
        MessageId::ViewToggleGlobalSearch => "切换全局搜索",
        MessageId::ViewHistory => "历史记录",
        MessageId::ViewCommandSearch => "命令搜索",
        MessageId::ViewWorkflows => "工作流",
        MessageId::ViewIncreaseFontSize => "增大字体",
        MessageId::ViewDecreaseFontSize => "减小字体",
        MessageId::ViewResetFontSize => "重置字体大小",
        MessageId::ViewIncreaseZoom => "放大",
        MessageId::ViewDecreaseZoom => "缩小",
        MessageId::ViewResetZoom => "重置缩放",
        MessageId::ViewToggleWarpDrive => "切换 Warp Drive",

        // Tab menu actions
        MessageId::TabRename => "重命名标签页",
        MessageId::TabSplitRight => "向右分屏",
        MessageId::TabSplitLeft => "向左分屏",
        MessageId::TabSplitDown => "向下分屏",
        MessageId::TabSplitUp => "向上分屏",
        MessageId::TabMoveLeft => "向左移动标签页",
        MessageId::TabMoveRight => "向右移动标签页",
        MessageId::TabCycleNext => "下一个标签页",
        MessageId::TabCyclePrev => "上一个标签页",
        MessageId::TabActivateNextPane => "激活下一个窗格",
        MessageId::TabActivatePrevPane => "激活上一个窗格",
        MessageId::TabMaximizePane => "最大化窗格",
        MessageId::TabClose => "关闭标签页",
        MessageId::TabCloseOthers => "关闭其他标签页",
        MessageId::TabCloseRight => "关闭右侧标签页",

        // Blocks menu actions
        MessageId::BlocksClear => "清除块",
        MessageId::BlocksSelectAbove => "选择上方的块",
        MessageId::BlocksSelectBelow => "选择下方的块",
        MessageId::BlocksSelectAll => "选择所有块",
        MessageId::BlocksScrollToTop => "滚动到选中块顶部",
        MessageId::BlocksScrollToBottom => "滚动到选中块底部",
        MessageId::BlocksCreatePermalink => "创建块永久链接",
        MessageId::BlocksToggleBookmark => "切换书签",
        MessageId::BlocksFindWithin => "在块内查找",
        MessageId::BlocksCopy => "复制块",
        MessageId::BlocksCopyCommand => "复制块命令",
        MessageId::BlocksCopyOutput => "复制块输出",
        MessageId::BlocksViewShared => "查看共享块",

        // AI menu actions
        MessageId::AiNewAgentMode => "新建智能体模式窗格",
        MessageId::AiAttachSelection => "附加选中内容作为上下文",
        MessageId::AiSearch => "AI 搜索",
        MessageId::AiOpenFactCollection => "打开知识库",
        MessageId::AiOpenMcpServers => "打开 MCP 服务器",

        // Drive menu actions
        MessageId::DriveNewPersonalWorkflow => "新建个人工作流",
        MessageId::DriveNewPersonalNotebook => "新建个人笔记本",
        MessageId::DriveNewPersonalAIPrompt => "新建个人 AI 提示词",
        MessageId::DriveNewPersonalEnvVars => "新建个人环境变量",
        MessageId::DriveNewTeamWorkflow => "新建团队工作流",
        MessageId::DriveNewTeamNotebook => "新建团队笔记本",
        MessageId::DriveNewTeamAIPrompt => "新建团队 AI 提示词",
        MessageId::DriveNewTeamEnvVars => "新建团队环境变量",
        MessageId::DriveSearch => "搜索云盘",
        MessageId::DriveOpenTeamSettings => "打开团队设置",
        MessageId::DriveSharePaneContents => "分享窗格内容",
        MessageId::DriveShareSession => "分享当前会话",

        // App menu actions
        MessageId::AppShowAboutWarp => "关于 Warp",
        MessageId::AppToggleResourceCenter => "资源中心",
        MessageId::AppReferAFriend => "邀请好友",
        MessageId::AppShowSettings => "设置",
        MessageId::AppToggleKeybindings => "快捷键",
        MessageId::AppConfigureKeybindings => "配置快捷键",
        MessageId::AppShowAppearance => "外观",
        MessageId::AppViewChangelog => "查看更新日志",

        // Sync input actions
        MessageId::SyncAllTerminalInputs => "同步所有标签页的终端输入",
        MessageId::SyncTerminalInputsCurrentTab => "同步当前标签页的终端输入",
        MessageId::DisableSyncInputs => "禁用同步输入",

        // File menu extras
        MessageId::FileOpenRepository => "打开仓库",
        MessageId::AddWindow => "新建窗口",

        // Settings content - Account page
        MessageId::SettingsSignUp => "注册",
        MessageId::SettingsFree => "免费版",
        MessageId::SettingsComparePlans => "对比套餐",
        MessageId::SettingsUpToDate => "已是最新版本",
        MessageId::SettingsVersion => "版本",
        MessageId::SettingsLogOut => "退出登录",
        MessageId::SettingsReferralCta => "邀请朋友和同事使用 Warp 即可获得奖励",
        MessageId::SettingsContactSupport => "联系客服",
        MessageId::SettingsManageBilling => "管理账单",
        MessageId::SettingsCheckForUpdates => "检查更新",
        MessageId::SettingsRelaunchWarp => "重新启动 Warp",
        MessageId::SettingsUpdateAvailable => "有可用更新",
        MessageId::SettingsUpdating => "正在更新...",
        MessageId::SettingsInstalledUpdate => "已安装更新",
        MessageId::SettingsUpdateManually => "手动更新 Warp",
        MessageId::SettingsSettingsSync => "设置同步",

        // Settings content - Appearance page categories
        MessageId::AppearanceCategoryThemes => "主题",
        MessageId::AppearanceCategoryIcon => "图标",
        MessageId::AppearanceCategoryWindow => "窗口",
        MessageId::AppearanceCategoryInput => "输入",
        MessageId::AppearanceCategoryPanes => "窗格",
        MessageId::AppearanceCategoryBlocks => "块",
        MessageId::AppearanceCategoryText => "文本",
        MessageId::AppearanceCategoryCursor => "光标",
        MessageId::AppearanceCategoryTabs => "标签页",
        MessageId::AppearanceCategoryFullscreenApps => "全屏应用",

        // Settings content - Appearance page labels
        MessageId::AppearanceCompactMode => "紧凑模式",
        MessageId::AppearanceSyncWithOs => "主题：与系统同步",
        MessageId::AppearanceCursorBlink => "光标闪烁",
        MessageId::AppearanceJumpToBottom => "跳转到块底部按钮",
        MessageId::AppearanceBlockDividers => "块分隔线",
        MessageId::AppearanceDimInactivePanes => "暗化非活动窗格",
        MessageId::AppearanceTabIndicators => "标签页指示器",
        MessageId::AppearanceFocusFollowsMouse => "焦点跟随鼠标",
        MessageId::AppearanceZenMode => "禅模式",
        MessageId::AppearanceVerticalTabLayout => "垂直标签页布局",
        MessageId::AppearanceLigatureRendering => "连字渲染",
        MessageId::AppearanceStartInputTop => "在顶部开始输入",
        MessageId::AppearancePinInputTop => "固定输入到顶部",
        MessageId::AppearancePinInputBottom => "固定输入到底部",
        MessageId::AppearanceToggleInputMode => "切换输入模式（Warp/经典）",
        MessageId::AppearanceAlwaysShowTabBar => "始终显示标签栏",
        MessageId::AppearanceHideTabBarFullscreen => "全屏时隐藏标签栏",
        MessageId::AppearanceShowTabBarOnHover => "悬停时显示标签栏",
        MessageId::AppearanceHeaderToolbarLayout => "顶部工具栏布局",
        MessageId::AppearanceCreateCustomTheme => "创建自定义主题",
        MessageId::AppearanceWindowOpacity => "窗口不透明度",
        MessageId::AppearanceWindowBlur => "使用窗口模糊",
        MessageId::AppearanceInputType => "输入类型",
        MessageId::AppearanceShowCodeReviewButton => "在标签栏显示代码审查按钮",
        MessageId::AppearanceHideCodeReviewButton => "在标签栏隐藏代码审查按钮",

        // Settings content - Appearance dropdown options
        MessageId::AppearanceInputModeWarp => "固定到底部（Warp 模式）",
        MessageId::AppearanceInputModeReverse => "固定到顶部（反向模式）",
        MessageId::AppearanceInputModeClassic => "在顶部开始（经典模式）",
        MessageId::AppearanceThinStrokesNever => "从不",
        MessageId::AppearanceThinStrokesLowDpi => "在低 DPI 显示器上",
        MessageId::AppearanceThinStrokesHighDpi => "在高 DPI 显示器上",
        MessageId::AppearanceThinStrokesAlways => "始终",
        MessageId::AppearanceContrastAlways => "始终",
        MessageId::AppearanceContrastNamedColors => "仅命名颜色",
        MessageId::AppearanceNever => "从不",
        MessageId::AppearanceAlways => "始终",

        // Settings content - Features page categories
        MessageId::FeaturesCategoryGeneral => "通用",
        MessageId::FeaturesCategorySession => "会话",
        MessageId::FeaturesCategoryKeys => "按键",
        MessageId::FeaturesCategoryTextEditing => "文本编辑",
        MessageId::FeaturesCategoryTerminalInput => "终端输入",
        MessageId::FeaturesCategoryTerminal => "终端",
        MessageId::FeaturesCategoryNotifications => "通知",
        MessageId::FeaturesCategoryWorkflows => "工作流",
        MessageId::FeaturesCategorySystem => "系统",

        // Settings content - Features page labels
        MessageId::FeaturesCopyOnSelect => "在终端中选中时复制",
        MessageId::FeaturesLinuxSelectionClipboard => "Linux 选择剪贴板",
        MessageId::FeaturesAutocompleteQuotes => "自动补全引号、圆括号和方括号",
        MessageId::FeaturesRestoreWindows => "启动时恢复窗口、标签页和窗格",
        MessageId::FeaturesScrollReporting => "滚动上报",
        MessageId::FeaturesCompletionsWhileTyping => "输入时自动补全",
        MessageId::FeaturesCommandCorrections => "命令纠错",
        MessageId::FeaturesErrorUnderlining => "错误下划线",
        MessageId::FeaturesSyntaxHighlighting => "语法高亮",
        MessageId::FeaturesAudibleTerminalBell => "终端响铃",
        MessageId::FeaturesAutosuggestions => "自动建议",
        MessageId::FeaturesAutosuggestionKeybindingHint => "自动建议快捷键提示",
        MessageId::FeaturesSshWrapper => "Warp SSH 包装器",
        MessageId::FeaturesLinkTooltip => "点击链接时显示提示",
        MessageId::FeaturesVimUnnamedRegister => "Vim 未命名寄存器作为系统剪贴板",
        MessageId::FeaturesVimStatusBar => "Vim 状态栏",
        MessageId::FeaturesWaylandWindowManagement => "使用 Wayland 进行窗口管理",
        MessageId::FeaturesConfigureGlobalHotkey => "配置全局热键",
        MessageId::FeaturesMakeDefaultTerminal => "将 Warp 设为默认终端",
        MessageId::FeaturesLeftOptionMeta => "左侧 Option 键为 Meta",
        MessageId::FeaturesRightOptionMeta => "右侧 Option 键为 Meta",
        MessageId::FeaturesLeftAltMeta => "左侧 Alt 键为 Meta",
        MessageId::FeaturesRightAltMeta => "右侧 Alt 键为 Meta",
        MessageId::FeaturesPerformanceWarning => "将限制设置为超过 10 万行可能会影响性能。",

        // Settings content - Code page
        MessageId::CodeInitializationSettings => "初始化设置",
        MessageId::CodeCodebaseIndexing => "代码库索引",
        MessageId::CodeCodebaseIndexDescription => "Warp 可以在您浏览代码仓库时自动建立索引，帮助智能体快速理解上下文并提供解决方案。代码永远不会存储在服务器上。",
        MessageId::CodeWarpIndexingIgnoreDescription => "要从索引中排除特定文件或目录，请将它们添加到仓库目录中的 .warpindexingignore 文件中。",
        MessageId::CodeAutoIndexFeatureName => "默认索引新文件夹",
        MessageId::CodeAutoIndexDescription => "启用后，Warp 将在您浏览代码仓库时自动建立索引。",
        MessageId::CodeIndexingDisabledAdmin => "团队管理员已禁用代码库索引。",
        MessageId::CodeIndexingWorkspaceEnabledAdmin => "团队管理员已启用代码库索引。",
        MessageId::CodeIndexingDisabledGlobalAi => "必须启用 AI 功能才能使用代码库索引。",
        MessageId::CodeCodebaseIndexLimitReached => "您已达到当前套餐的最大代码库索引数量。",
        MessageId::CodeCodebaseIndexingCategory => "代码库索引",
        MessageId::CodeEditorAndReviewCategory => "代码编辑器和审查",
        MessageId::CodeIndexNewFolder => "索引新文件夹",
        MessageId::CodeRestartServer => "重启服务器",
        MessageId::CodeViewLogs => "查看日志",
        MessageId::CodeSyncing => "同步中...",
        MessageId::CodeSynced => "已同步",
        MessageId::CodeCodebaseTooLarge => "代码库过大",
        MessageId::CodeInstalled => "已安装",
        MessageId::CodeInstalling => "安装中...",
        MessageId::CodeChecking => "检查中...",
        MessageId::CodeAvailableForDownload => "可下载",
        MessageId::CodeAvailable => "可用",
        MessageId::CodeBusy => "忙碌",
        MessageId::CodeFailed => "失败",
        MessageId::CodeStopped => "已停止",
        MessageId::CodeNotRunning => "未运行",
        MessageId::CodeInitializedFolders => "已初始化/索引的文件夹",
        MessageId::CodeNoFoldersInitialized => "尚未初始化任何文件夹。",
        MessageId::CodeOpenProjectRules => "打开项目规则",
        MessageId::CodeIndexingLabel => "索引",
        MessageId::CodeNoIndexCreated => "未创建索引",
        MessageId::CodeAutoOpenCodeReview => "自动打开代码审查面板",
        MessageId::CodeAutoOpenCodeReviewDescription => "启用此设置后，代码审查面板将在对话中接受第一个 diff 时打开",
        MessageId::CodeShowCodeReviewButton => "显示代码审查按钮",
        MessageId::CodeShowCodeReviewButtonDescription => "在窗口右上角显示一个按钮来切换代码审查面板。",
        MessageId::CodeShowDiffStats => "在代码审查按钮上显示差异统计",
        MessageId::CodeShowDiffStatsDescription => "在代码审查按钮上显示添加和删除的行数。",
        MessageId::CodeProjectExplorer => "项目浏览器",
        MessageId::CodeProjectExplorerDescription => "在左侧工具面板中添加 IDE 风格的项目浏览器/文件树。",
        MessageId::CodeGlobalFileSearch => "全局文件搜索",
        MessageId::CodeGlobalFileSearchDescription => "在左侧工具面板中添加全局文件搜索。",

        // Settings content - Privacy page
        MessageId::PrivacySecretRedaction => "密钥脱敏",
        MessageId::PrivacySecretRedactionDescription => "启用此设置后，Warp 将扫描块、Warp Drive 对象内容和 Oz 提示词中的潜在敏感信息，防止将这些数据保存或发送到任何服务器。",
        MessageId::PrivacyCustomSecretRedaction => "自定义密钥脱敏",
        MessageId::PrivacyCustomSecretDescription => "使用正则表达式定义要脱敏的额外密钥或数据。此设置将在下一条命令运行时生效。",
        MessageId::PrivacyTelemetryTitle => "帮助改进 Warp",
        MessageId::PrivacyTelemetryDescription => "应用分析帮助我们为您改进产品。我们可能会收集某些控制台交互以改进 Warp 的 AI 功能。",
        MessageId::PrivacyTelemetryFreeTierNote => "在免费套餐中，必须启用分析功能才能使用 AI 功能。",
        MessageId::PrivacyDataManagementTitle => "管理您的数据",
        MessageId::PrivacyDataManagementDescription => "您可以随时选择永久删除您的 Warp 账户。删除后将无法再使用 Warp。",
        MessageId::PrivacyDataManagementLinkText => "访问数据管理页面",
        MessageId::PrivacyPrivacyPolicyTitle => "隐私政策",
        MessageId::PrivacyPrivacyPolicyLinkText => "阅读 Warp 的隐私政策",

        // Settings content - Billing page
        MessageId::BillingOverview => "概览",
        MessageId::BillingUsageHistory => "用量历史",
        MessageId::BillingViewOverageDetails => "查看超额详情",
        MessageId::BillingEnableOverages => "启用超额",
        MessageId::BillingOveragesEnabled => "超额已启用",
        MessageId::BillingOveragesNotEnabled => "超额未启用",
        MessageId::BillingAtoZ => "A 到 Z",
        MessageId::BillingZtoA => "Z 到 A",
        MessageId::BillingUsageAscending => "用量升序",
        MessageId::BillingUsageDescending => "用量降序",
        MessageId::BillingEnterpriseUsageCallout => "请联系您的团队管理员获取用量详情。",

        // Settings content - Teams page
        MessageId::TeamsTeamName => "团队名称",
        MessageId::TeamsLeaveTeam => "离开团队",
        MessageId::TeamsDeleteTeam => "删除团队",
        MessageId::TeamsCreate => "创建",
        MessageId::TeamsCreateDescription => "创建一个新团队，与您的同事协作。",
        MessageId::TeamsDomainsPlaceholder => "添加域名",
        MessageId::TeamsEmailsPlaceholder => "添加电子邮件地址",
        MessageId::TeamsSet => "设置",
        MessageId::TeamsInvite => "邀请",
        MessageId::TeamsInviteLinkInstructions => "将此链接分享给其他人以邀请他们加入您的团队。",
        MessageId::TeamsInviteByEmailExpiry => "邀请在 7 天后过期。",
        MessageId::TeamsOffline => "您似乎处于离线状态。",

        // Settings content - Referrals page
        MessageId::ReferralsHeader => "邀请朋友使用 Warp",
        MessageId::ReferralsAnonymousHeader => "创建账户以开始邀请朋友",
        MessageId::ReferralsFailedLoad => "加载推荐数据失败。",
        MessageId::ReferralsCopyLink => "复制链接",
        MessageId::ReferralsSend => "发送",
        MessageId::ReferralsSending => "发送中...",
        MessageId::ReferralsLoading => "加载中...",
        MessageId::ReferralsLinkCopied => "链接已复制！",
        MessageId::ReferralsEmailSuccess => "邀请已发送！",
        MessageId::ReferralsEmailFailure => "发送邀请失败。",
        MessageId::ReferralsRewardIntro => "当您的朋友加入 Warp 后，你们都将获得奖励。",
        MessageId::ReferralsCurrentReferral => "您有 1 个活跃推荐。",
        MessageId::ReferralsCurrentReferrals => "您有 {count} 个活跃推荐。",
        MessageId::ReferralsCertainRestrictions => "某些限制适用。",

        // Settings content - Keybindings page
        MessageId::KeybindingsSearchPlaceholder => "按名称或按键搜索",
        MessageId::KeybindingsConflictWarning => "此快捷键与其他快捷键冲突",
        MessageId::KeybindingsDefault => "默认",
        MessageId::KeybindingsCancel => "取消",
        MessageId::KeybindingsClear => "清除",
        MessageId::KeybindingsSave => "保存",

        // Settings content - Other pages
        MessageId::WarpDriveSignUpRequired => "要使用 Warp Drive，请创建账户。",
        MessageId::WarpifySubshells => "子 Shell",
        MessageId::WarpifySsh => "SSH",
        MessageId::McpServersTitle => "MCP 服务器",
        MessageId::EnvironmentsLastEdited => "上次编辑：",
        MessageId::EnvironmentsLastUsed => "上次使用：",
        MessageId::EnvironmentsViewRuns => "查看我的运行记录",
        MessageId::PlatformNewApiKey => "新建 API 密钥",
        MessageId::ShowBlocksCopyLink => "复制链接",
        MessageId::ShowBlocksDeleting => "删除中...",
        MessageId::SettingsOpenSettingsFile => "打开设置文件",
        MessageId::SettingsDefault => "默认",
    }
}

#[cfg(test)]
mod tests {
    use super::{set_current_locale, tr, Locale, MessageId};

    #[test]
    fn returns_english_messages() {
        assert_eq!(
            tr(Locale::EnUs, MessageId::SettingsBillingAndUsage),
            "Billing and usage"
        );
    }

    #[test]
    fn returns_simplified_chinese_messages() {
        assert_eq!(
            tr(Locale::ZhCn, MessageId::SettingsBillingAndUsage),
            "账单和用量"
        );
    }

    #[test]
    #[serial_test::serial]
    fn current_default_locale_is_simplified_chinese() {
        set_current_locale(Locale::ZhCn);
        assert_eq!(super::current_locale(), Locale::ZhCn);
    }

    #[test]
    #[serial_test::serial]
    fn current_locale_can_be_changed() {
        set_current_locale(Locale::EnUs);
        assert_eq!(super::current_locale(), Locale::EnUs);
        set_current_locale(Locale::ZhCn);
    }
}
