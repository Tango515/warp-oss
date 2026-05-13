use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::ai::persisted_workspace::PersistedWorkspace;
use crate::auth::AuthStateProvider;
use crate::default_terminal::DefaultTerminal;
use crate::features::{runtime_flags_menu_items, FeatureFlag};
use crate::i18n::{current_locale, tr, MessageId};
use crate::root_view::OpenLaunchConfigArg;
use crate::server::telemetry::LaunchConfigUiLocation;
use crate::settings::{
    AISettings, BlockVisibilitySettings, DebugSettings, DefaultSessionMode, SelectionSettings,
};
use crate::terminal::alt_screen_reporting::AltScreenReporting;
use crate::terminal::session_settings::SessionSettings;
use crate::terminal::settings::{SpacingMode, TerminalSettings};
use crate::undo_close::UndoCloseStack;
use crate::user_config::WarpConfig;
use crate::util::bindings::{self, trigger_to_keystroke, CustomAction};
use crate::util::links;
use crate::workspace::sync_inputs::SyncedInputState;
use crate::{auth, report_if_error};
use ai::workspace::WorkspaceMetadata;
use csv::Writer;
use enclose::enclose;
use itertools::Itertools;
use settings::manager::SettingsManager;
use settings::Setting as _;
use warp_core::context_flag::ContextFlag;
use warp_util::path::user_friendly_path;
use warpui::actions::StandardAction;
use warpui::keymap::{Keystroke, Trigger};
use warpui::platform::menu::{
    CustomMenuItem, Menu, MenuBar, MenuItem, MenuItemProperties, MenuItemPropertyChanges,
};
use warpui::windowing::WindowManager;
use warpui::{AppContext, SingletonEntity};

type CheckmarkStatusGetter = dyn 'static + Fn(&mut AppContext) -> bool;

const SETTINGS_CSV_FILE_NAME: &str = "warp_default_settings.csv";
const MAX_RECENT_REPOS_IN_MENU: usize = 10;

/// Creates the root app menu bar
pub fn menu_bar(ctx: &mut AppContext) -> MenuBar {
    MenuBar::new(vec![
        make_new_app_menu(ctx),
        make_new_file_menu(ctx),
        make_new_edit_menu(ctx),
        make_new_view_menu(ctx),
        make_new_tab_menu(ctx),
        make_new_blocks_menu(ctx),
        make_new_ai_menu(ctx),
        make_new_drive_menu(ctx),
        make_new_window_menu(),
        make_new_help_menu(),
    ])
}

// Creates the app dock menu
// Menu here instead of MenuBar, since we only need one vec<MenuItem> in the dock
// To create submenus, we could use MenuItem::Custom(CustomMenuItem::new_with_submenu(...))
pub fn dock_menu() -> Menu {
    Menu::new(
        &menu_title(MessageId::FileNewWindow),
        vec![MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DockNewWindow),
            move |ctx| {
                ctx.dispatch_global_action("root_view:open_new", &());
                ctx.dispatch_global_action("workspace:save_app", &());
            },
            translated_updater(MessageId::DockNewWindow),
            Some(Keystroke::parse("cmd-n").expect("Valid keystroke")),
        ))],
    )
}

fn custom_shortcut(action: CustomAction) -> Option<Keystroke> {
    trigger_to_keystroke(&Trigger::Custom(action.into()))
}

fn default_name(action: CustomAction, ctx: &AppContext) -> String {
    if let Some(message_id) = custom_action_to_message_id(action) {
        let locale = current_locale();
        if locale != crate::i18n::Locale::EnUs {
            return tr(locale, message_id).into_owned();
        }
    }
    ctx.description_for_custom_action(action.into(), bindings::MAC_MENUS_CONTEXT)
        .unwrap_or_else(|| {
            debug_assert!(false, "action should have a name: {action:?}");
            "<NO DESCRIPTION>".into()
        })
}

fn custom_action_to_message_id(action: CustomAction) -> Option<MessageId> {
    match action {
        // App menu
        CustomAction::ShowAboutWarp => Some(MessageId::AppShowAboutWarp),
        CustomAction::ToggleResourceCenter => Some(MessageId::AppToggleResourceCenter),
        CustomAction::ReferAFriend => Some(MessageId::AppReferAFriend),
        CustomAction::ShowSettings => Some(MessageId::AppShowSettings),
        CustomAction::ToggleKeybindingsPage => Some(MessageId::AppToggleKeybindings),
        CustomAction::ConfigureKeybindings => Some(MessageId::AppConfigureKeybindings),
        CustomAction::ShowAppearance => Some(MessageId::AppShowAppearance),
        CustomAction::ViewChangelog => Some(MessageId::AppViewChangelog),
        CustomAction::ShowAccount => Some(MessageId::SettingsAccount),
        // Edit menu
        CustomAction::Undo => Some(MessageId::EditUndo),
        CustomAction::Redo => Some(MessageId::EditRedo),
        CustomAction::Cut => Some(MessageId::EditCut),
        CustomAction::Copy => Some(MessageId::EditCopy),
        CustomAction::Paste => Some(MessageId::EditPaste),
        CustomAction::SelectAll => Some(MessageId::EditSelectAll),
        CustomAction::ClearEditor => Some(MessageId::EditClearEditor),
        CustomAction::Find => Some(MessageId::EditFind),
        CustomAction::GoToLine => Some(MessageId::EditGoToLine),
        CustomAction::FocusInput => Some(MessageId::EditFocusInput),
        CustomAction::AddNextOccurrence => Some(MessageId::EditAddNextOccurrence),
        CustomAction::AddCursorAbove => Some(MessageId::EditAddCursorAbove),
        CustomAction::AddCursorBelow => Some(MessageId::EditAddCursorBelow),
        // View menu
        CustomAction::CommandPalette => Some(MessageId::ViewCommandPalette),
        CustomAction::NavigationPalette => Some(MessageId::ViewNavigationPalette),
        CustomAction::LaunchConfigPalette => Some(MessageId::ViewLaunchConfigPalette),
        CustomAction::FilesPalette => Some(MessageId::ViewFilesPalette),
        CustomAction::ToggleConversationListView => Some(MessageId::ViewToggleConversationList),
        CustomAction::ToggleProjectExplorer => Some(MessageId::ViewToggleProjectExplorer),
        CustomAction::ToggleGlobalSearch => Some(MessageId::ViewToggleGlobalSearch),
        CustomAction::History => Some(MessageId::ViewHistory),
        CustomAction::CommandSearch => Some(MessageId::ViewCommandSearch),
        CustomAction::Workflows => Some(MessageId::ViewWorkflows),
        CustomAction::IncreaseFontSize => Some(MessageId::ViewIncreaseFontSize),
        CustomAction::DecreaseFontSize => Some(MessageId::ViewDecreaseFontSize),
        CustomAction::ResetFontSize => Some(MessageId::ViewResetFontSize),
        CustomAction::IncreaseZoom => Some(MessageId::ViewIncreaseZoom),
        CustomAction::DecreaseZoom => Some(MessageId::ViewDecreaseZoom),
        CustomAction::ResetZoom => Some(MessageId::ViewResetZoom),
        CustomAction::ToggleWarpDrive => Some(MessageId::ViewToggleWarpDrive),
        // Tab menu
        CustomAction::RenameTab => Some(MessageId::TabRename),
        CustomAction::SplitPaneRight => Some(MessageId::TabSplitRight),
        CustomAction::SplitPaneLeft => Some(MessageId::TabSplitLeft),
        CustomAction::SplitPaneDown => Some(MessageId::TabSplitDown),
        CustomAction::SplitPaneUp => Some(MessageId::TabSplitUp),
        CustomAction::MoveTabLeft => Some(MessageId::TabMoveLeft),
        CustomAction::MoveTabRight => Some(MessageId::TabMoveRight),
        CustomAction::CycleNextSession => Some(MessageId::TabCycleNext),
        CustomAction::CyclePrevSession => Some(MessageId::TabCyclePrev),
        CustomAction::ActivateNextPane => Some(MessageId::TabActivateNextPane),
        CustomAction::ActivatePreviousPane => Some(MessageId::TabActivatePrevPane),
        CustomAction::ToggleMaximizePane => Some(MessageId::TabMaximizePane),
        CustomAction::CloseTab => Some(MessageId::TabClose),
        CustomAction::CloseOtherTabs => Some(MessageId::TabCloseOthers),
        CustomAction::CloseTabsRight => Some(MessageId::TabCloseRight),
        CustomAction::ActivateNextTab => Some(MessageId::TabCycleNext),
        CustomAction::ActivatePreviousTab => Some(MessageId::TabCyclePrev),
        // Blocks menu
        CustomAction::ClearBlocks => Some(MessageId::BlocksClear),
        CustomAction::SelectBlockAbove => Some(MessageId::BlocksSelectAbove),
        CustomAction::SelectBlockBelow => Some(MessageId::BlocksSelectBelow),
        CustomAction::SelectAllBlocks => Some(MessageId::BlocksSelectAll),
        CustomAction::ScrollToTopOfSelectedBlocks => Some(MessageId::BlocksScrollToTop),
        CustomAction::ScrollToBottomOfSelectedBlocks => Some(MessageId::BlocksScrollToBottom),
        CustomAction::CreateBlockPermalink => Some(MessageId::BlocksCreatePermalink),
        CustomAction::ToggleBookmarkBlock => Some(MessageId::BlocksToggleBookmark),
        CustomAction::FindWithinBlock => Some(MessageId::BlocksFindWithin),
        CustomAction::CopyBlock => Some(MessageId::BlocksCopy),
        CustomAction::CopyBlockCommand => Some(MessageId::BlocksCopyCommand),
        CustomAction::CopyBlockOutput => Some(MessageId::BlocksCopyOutput),
        CustomAction::ViewSharedBlocks => Some(MessageId::BlocksViewShared),
        // AI menu
        CustomAction::NewAgentModePane => Some(MessageId::AiNewAgentMode),
        CustomAction::AttachSelectionAsAgentModeContext => Some(MessageId::AiAttachSelection),
        CustomAction::AISearch => Some(MessageId::AiSearch),
        CustomAction::OpenAIFactCollection => Some(MessageId::AiOpenFactCollection),
        CustomAction::OpenMCPServerCollection => Some(MessageId::AiOpenMcpServers),
        // Drive menu
        CustomAction::NewPersonalWorkflow => Some(MessageId::DriveNewPersonalWorkflow),
        CustomAction::NewPersonalNotebook => Some(MessageId::DriveNewPersonalNotebook),
        CustomAction::NewPersonalAIPrompt => Some(MessageId::DriveNewPersonalAIPrompt),
        CustomAction::NewPersonalEnvVars => Some(MessageId::DriveNewPersonalEnvVars),
        CustomAction::NewTeamWorkflow => Some(MessageId::DriveNewTeamWorkflow),
        CustomAction::NewTeamNotebook => Some(MessageId::DriveNewTeamNotebook),
        CustomAction::NewTeamAIPrompt => Some(MessageId::DriveNewTeamAIPrompt),
        CustomAction::NewTeamEnvVars => Some(MessageId::DriveNewTeamEnvVars),
        CustomAction::SearchDrive => Some(MessageId::DriveSearch),
        CustomAction::OpenTeamSettings => Some(MessageId::DriveOpenTeamSettings),
        CustomAction::SharePaneContents => Some(MessageId::DriveSharePaneContents),
        CustomAction::ShareCurrentSession => Some(MessageId::DriveShareSession),
        // Sync inputs
        CustomAction::ToggleSyncAllTerminalInputsInAllTabs => {
            Some(MessageId::SyncAllTerminalInputs)
        }
        CustomAction::ToggleSyncTerminalInputsInCurrentTab => {
            Some(MessageId::SyncTerminalInputsCurrentTab)
        }
        CustomAction::DisableSyncTerminalInputs => Some(MessageId::DisableSyncInputs),
        // File menu
        CustomAction::OpenRepository => Some(MessageId::FileOpenRepository),
        CustomAction::NewFile => Some(MessageId::FileNewFile),
        CustomAction::SaveCurrentConfig => Some(MessageId::FileSaveNew),
        CustomAction::ReopenClosedSession => Some(MessageId::FileReopenClosedSession),
        CustomAction::CloseCurrentSession => Some(MessageId::FileClose),
        CustomAction::CloseWindow => Some(MessageId::FileCloseWindow),
        CustomAction::NewTab => Some(MessageId::FileNewTerminalTab),
        CustomAction::NewTerminalTab => Some(MessageId::FileNewTerminalTab),
        CustomAction::NewAgentTab => Some(MessageId::FileNewAgentTab),
        CustomAction::AddWindow => Some(MessageId::AddWindow),
        // No translation available
        _ => None,
    }
}

fn menu_title(message_id: MessageId) -> String {
    tr(current_locale(), message_id).into_owned()
}

/// Global reference to the menu bar builder function, so we can rebuild the
/// menu bar when the locale changes.
static MENU_BAR_BUILDER: OnceLock<fn(&mut AppContext) -> MenuBar> = OnceLock::new();

/// Store the menu bar builder for later rebuilds.
pub fn store_menu_bar_builder(builder: fn(&mut AppContext) -> MenuBar) {
    let _ = MENU_BAR_BUILDER.set(builder);
}

/// Rebuild the entire menu bar with the current locale and set it on NSApp.
#[cfg(target_os = "macos")]
pub fn rebuild_menu_bar(ctx: &mut AppContext) {
    let Some(builder) = MENU_BAR_BUILDER.get() else {
        return;
    };
    let menu_bar = builder(ctx);
    unsafe {
        use cocoa::appkit::NSApp;
        use objc::{msg_send, sel, sel_impl};
        use warpui::platform::mac::menus::make_main_menu;
        let nsmenu = make_main_menu(menu_bar);
        let () = msg_send![NSApp(), setMainMenu: nsmenu];
    }
}

#[cfg(not(target_os = "macos"))]
pub fn rebuild_menu_bar(_ctx: &mut AppContext) {}

/// Returns an updater that dynamically sets the menu item name to the
/// current translation of `message_id`. Use this instead of `no_updates`
/// for menu items whose label should change when the locale changes.
fn translated_updater(
    message_id: MessageId,
) -> impl Fn(&MenuItemProperties, &mut AppContext) -> MenuItemPropertyChanges {
    move |_, _| MenuItemPropertyChanges {
        name: Some(menu_title(message_id)),
        ..Default::default()
    }
}

fn non_updateable_custom_item(action: CustomAction, ctx: &AppContext) -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &default_name(action, ctx),
        custom_action_dispatcher(action),
        no_updates,
        custom_shortcut(action),
    ))
}

/// Return a Custom Menu Item whose CustomAction can be updated and
/// whose checkmark status is determined by
/// the bool result of should_be_checked, a provided closure
fn updateable_custom_item_with_checkmark(
    action: CustomAction,
    ctx: &AppContext,
    should_be_checked: Box<CheckmarkStatusGetter>,
) -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &default_name(action, ctx),
        custom_action_dispatcher(action),
        custom_action_updater(action, should_be_checked),
        custom_shortcut(action),
    ))
}

/// Return a Custom Menu Item whose CustomAction can be updated
/// and is always unchecked
fn updateable_custom_item_without_checkmark(action: CustomAction, ctx: &AppContext) -> MenuItem {
    updateable_custom_item_with_checkmark(action, ctx, Box::new(|_| false))
}

fn make_new_app_menu(ctx: &AppContext) -> Menu {
    let mut menu_items = vec![updateable_custom_item_without_checkmark(
        CustomAction::ShowAboutWarp,
        ctx,
    )];

    if !FeatureFlag::AvatarInTabBar.is_enabled() {
        menu_items.push(updateable_custom_item_without_checkmark(
            CustomAction::ToggleResourceCenter,
            ctx,
        ))
    }

    menu_items.extend([
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ReferAFriend, ctx),
        MenuItem::Separator,
    ]);

    let preferences_menu_items = vec![
        updateable_custom_item_without_checkmark(CustomAction::ShowSettings, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ToggleKeybindingsPage, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ConfigureKeybindings, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ShowAppearance, ctx),
        MenuItem::Separator,
    ];

    menu_items.push(MenuItem::Custom(CustomMenuItem::new_with_submenu(
        &menu_title(MessageId::AppPreferences),
        |_| (),
        translated_updater(MessageId::AppPreferences),
        None,
        preferences_menu_items,
    )));

    if FeatureFlag::Changelog.is_enabled() {
        menu_items.push(updateable_custom_item_without_checkmark(
            CustomAction::ViewChangelog,
            ctx,
        ));
    }

    #[cfg(target_os = "macos")]
    {
        menu_items.push(MenuItem::Services);
    }

    menu_items.push(MenuItem::Separator);
    menu_items.push(link_menu_item(
        MessageId::AppPrivacyPolicy,
        links::PRIVACY_POLICY_URL.into(),
    ));

    let debug_menu_items = debug_menu_items();
    if !debug_menu_items.is_empty() {
        menu_items.push(MenuItem::Custom(CustomMenuItem::new_with_submenu(
            &menu_title(MessageId::AppDebug),
            |_| (),
            translated_updater(MessageId::AppDebug),
            None,
            debug_menu_items,
        )));
    }

    menu_items.push(MenuItem::Separator);
    menu_items.push(MenuItem::Standard(StandardAction::Hide));
    menu_items.push(MenuItem::Standard(StandardAction::HideOtherApps));
    menu_items.push(MenuItem::Standard(StandardAction::ShowAllApps));
    menu_items.push(MenuItem::Separator);
    menu_items.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::AppSetDefaultTerminal),
        move |ctx| {
            DefaultTerminal::handle(ctx).update(ctx, |default_terminal, ctx| {
                default_terminal.make_warp_default(ctx)
            });
        },
        move |_props, ctx| {
            let default_terminal = DefaultTerminal::handle(ctx).as_ref(ctx);
            MenuItemPropertyChanges {
                name: Some(menu_title(MessageId::AppSetDefaultTerminal)),
                disabled: Some(
                    !DefaultTerminal::can_warp_become_default()
                        || default_terminal.is_warp_default(),
                ),
                ..Default::default()
            }
        },
        None,
    )));
    menu_items.push(MenuItem::Separator);
    menu_items.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::AppLogout),
        auth::maybe_log_out,
        move |_, ctx| {
            let is_anonymous = AuthStateProvider::handle(ctx)
                .as_ref(ctx)
                .get()
                .is_anonymous_or_logged_out();
            MenuItemPropertyChanges {
                name: Some(menu_title(MessageId::AppLogout)),
                disabled: Some(is_anonymous),
                ..Default::default()
            }
        },
        None,
    )));
    menu_items.push(MenuItem::Standard(StandardAction::Quit));
    Menu::new("Warp", menu_items)
}

fn make_new_file_menu(ctx: &AppContext) -> Menu {
    let mut file_menu_options = make_new_elements_menu_items(ctx);
    file_menu_options.extend([
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::OpenRepository, ctx),
        MenuItem::Custom(CustomMenuItem::new_with_submenu(
            &menu_title(MessageId::FileOpenRecent),
            |_| (),
            |_props, ctx| {
                let recent_repos = generate_recent_repos_for_menu(ctx);
                MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::FileOpenRecent)),
                    submenu: Some(Some(make_recent_repos_menu_items(ctx))),
                    disabled: Some(recent_repos.is_empty()),
                    ..Default::default()
                }
            },
            None,
            vec![],
        )),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::CloseCurrentSession, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CloseWindow, ctx),
    ]);

    Menu::new(menu_title(MessageId::MenuFile), file_menu_options)
}

fn make_new_edit_menu(ctx: &AppContext) -> Menu {
    let mut edit_menu_items = vec![];

    let group_1 = vec![
        updateable_custom_item_without_checkmark(CustomAction::Undo, ctx),
        updateable_custom_item_without_checkmark(CustomAction::Redo, ctx),
    ];
    let group_2 = vec![
        updateable_custom_item_without_checkmark(CustomAction::Cut, ctx),
        updateable_custom_item_without_checkmark(CustomAction::Copy, ctx),
        updateable_custom_item_without_checkmark(CustomAction::Paste, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SelectAll, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ClearEditor, ctx),
    ];
    let group_3 = vec![
        updateable_custom_item_without_checkmark(CustomAction::AddNextOccurrence, ctx),
        updateable_custom_item_without_checkmark(CustomAction::AddCursorAbove, ctx),
        updateable_custom_item_without_checkmark(CustomAction::AddCursorBelow, ctx),
    ];
    let group_4 = vec![
        updateable_custom_item_without_checkmark(CustomAction::Find, ctx),
        updateable_custom_item_without_checkmark(CustomAction::GoToLine, ctx),
        updateable_custom_item_without_checkmark(CustomAction::FocusInput, ctx),
    ];
    let group_5 = vec![
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::EditUseWarpPrompt),
            move |ctx| ctx.dispatch_global_action("app:toggle_user_ps1", &()),
            move |_props, ctx| MenuItemPropertyChanges {
                name: Some(menu_title(MessageId::EditUseWarpPrompt)),
                checked: Some(
                    SessionSettings::handle(ctx).read(ctx, |session_settings, _ctx| {
                        !session_settings.honor_ps1.value()
                    }),
                ),
                ..Default::default()
            },
            None,
        )),
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::EditCopyOnSelect),
            move |ctx| {
                ctx.dispatch_global_action("app:toggle_copy_on_select", &());
            },
            move |_props, ctx| MenuItemPropertyChanges {
                name: Some(menu_title(MessageId::EditCopyOnSelect)),
                checked: Some(
                    SelectionSettings::handle(ctx)
                        .as_ref(ctx)
                        .copy_on_select_enabled(),
                ),
                ..Default::default()
            },
            None,
        )),
    ];

    edit_menu_items.extend(group_1);
    edit_menu_items.push(MenuItem::Separator);
    edit_menu_items.extend(group_2);
    edit_menu_items.push(MenuItem::Separator);
    edit_menu_items.extend(group_3);
    edit_menu_items.push(MenuItem::Separator);
    edit_menu_items.extend(group_4);
    edit_menu_items.push(MenuItem::Separator);

    edit_menu_items.push(MenuItem::Custom(CustomMenuItem::new_with_submenu(
        &menu_title(MessageId::EditSynchronizeInputs),
        |_| (),
        translated_updater(MessageId::EditSynchronizeInputs),
        None,
        vec![
            updateable_custom_item_without_checkmark(
                CustomAction::ToggleSyncAllTerminalInputsInAllTabs,
                ctx,
            ),
            updateable_custom_item_without_checkmark(
                CustomAction::ToggleSyncTerminalInputsInCurrentTab,
                ctx,
            ),
            updateable_custom_item_with_checkmark(
                CustomAction::DisableSyncTerminalInputs,
                ctx,
                Box::new(|ctx| {
                    if let Some(window_id) = WindowManager::handle(ctx).as_ref(ctx).active_window()
                    {
                        SyncedInputState::handle(ctx)
                            .read(ctx, |status, _| !status.is_syncing_any_inputs(window_id))
                    } else {
                        false
                    }
                }),
            ),
        ],
    )));
    edit_menu_items.push(MenuItem::Separator);

    edit_menu_items.extend(group_5);

    Menu::new(menu_title(MessageId::MenuEdit), edit_menu_items)
}

fn make_new_view_menu(ctx: &AppContext) -> Menu {
    let mut items = vec![
        updateable_custom_item_without_checkmark(CustomAction::ToggleWarpDrive, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::CommandPalette, ctx),
        updateable_custom_item_without_checkmark(CustomAction::NavigationPalette, ctx),
        updateable_custom_item_without_checkmark(CustomAction::LaunchConfigPalette, ctx),
        updateable_custom_item_without_checkmark(CustomAction::FilesPalette, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ToggleConversationListView, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ToggleProjectExplorer, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ToggleGlobalSearch, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::History, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CommandSearch, ctx),
        updateable_custom_item_without_checkmark(CustomAction::Workflows, ctx),
        MenuItem::Separator,
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::ViewToggleMouseReporting),
            move |ctx| {
                ctx.dispatch_global_action("workspace:toggle_mouse_reporting", &());
            },
            move |_props, ctx| {
                let mouse_reporting_enabled = AltScreenReporting::handle(ctx)
                    .as_ref(ctx)
                    .mouse_reporting_enabled
                    .value();
                MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::ViewToggleMouseReporting)),
                    checked: Some(*mouse_reporting_enabled),
                    ..Default::default()
                }
            },
            None,
        )),
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::ViewToggleScrollReporting),
            move |ctx| {
                ctx.dispatch_global_action("workspace:toggle_scroll_reporting", &());
            },
            move |_props, ctx| {
                let reporting = AltScreenReporting::handle(ctx).as_ref(ctx);
                MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::ViewToggleScrollReporting)),
                    disabled: Some(!*reporting.mouse_reporting_enabled.value()),
                    checked: Some(*reporting.scroll_reporting_enabled.value()),
                    ..Default::default()
                }
            },
            None,
        )),
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::ViewToggleFocusReporting),
            move |ctx| {
                ctx.dispatch_global_action("workspace:toggle_focus_reporting", &());
            },
            move |_props, ctx| {
                let reporting = AltScreenReporting::handle(ctx).as_ref(ctx);
                MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::ViewToggleFocusReporting)),
                    checked: Some(*reporting.focus_reporting_enabled.value()),
                    ..Default::default()
                }
            },
            None,
        )),
    ];

    let is_compact_mode = matches!(
        TerminalSettings::handle(ctx)
            .as_ref(ctx)
            .spacing_mode
            .value(),
        SpacingMode::Compact
    );

    items.extend([
        MenuItem::Separator,
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::ViewCompactMode),
            move |ctx| {
                TerminalSettings::handle(ctx).update(ctx, |terminal_settings, ctx| {
                    let current_value = *terminal_settings.spacing_mode;
                    report_if_error!(terminal_settings
                        .spacing_mode
                        .set_value(current_value.other_mode(), ctx));
                });
            },
            move |_props, _| MenuItemPropertyChanges {
                name: Some(menu_title(MessageId::ViewCompactMode)),
                checked: Some(is_compact_mode),
                ..Default::default()
            },
            None,
        )),
        MenuItem::Separator,
    ]);

    if FeatureFlag::UIZoom.is_enabled() {
        items.extend([
            updateable_custom_item_without_checkmark(CustomAction::IncreaseZoom, ctx),
            updateable_custom_item_without_checkmark(CustomAction::DecreaseZoom, ctx),
            updateable_custom_item_without_checkmark(CustomAction::ResetZoom, ctx),
            MenuItem::Separator,
        ]);
    } else {
        items.extend([
            updateable_custom_item_without_checkmark(CustomAction::IncreaseFontSize, ctx),
            updateable_custom_item_without_checkmark(CustomAction::DecreaseFontSize, ctx),
            updateable_custom_item_without_checkmark(CustomAction::ResetFontSize, ctx),
            MenuItem::Separator,
        ]);
    }

    Menu::new(menu_title(MessageId::MenuView), items)
}

fn make_new_tab_menu(ctx: &AppContext) -> Menu {
    let items = vec![
        updateable_custom_item_without_checkmark(CustomAction::RenameTab, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::SplitPaneRight, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SplitPaneLeft, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SplitPaneDown, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SplitPaneUp, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::MoveTabLeft, ctx),
        updateable_custom_item_without_checkmark(CustomAction::MoveTabRight, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::CycleNextSession, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CyclePrevSession, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ActivateNextPane, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ActivatePreviousPane, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ToggleMaximizePane, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::CloseTab, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CloseOtherTabs, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CloseTabsRight, ctx),
    ];
    Menu::new(menu_title(MessageId::MenuTab), items)
}

fn make_new_ai_menu(ctx: &AppContext) -> Menu {
    let mut items = vec![updateable_custom_item_without_checkmark(
        CustomAction::NewAgentModePane,
        ctx,
    )];

    items.push(updateable_custom_item_without_checkmark(
        CustomAction::AttachSelectionAsAgentModeContext,
        ctx,
    ));

    items.extend([
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::AISearch, ctx),
    ]);

    if FeatureFlag::AIRules.is_enabled() {
        items.extend([
            MenuItem::Separator,
            updateable_custom_item_without_checkmark(CustomAction::OpenAIFactCollection, ctx),
        ]);
    }

    if FeatureFlag::McpServer.is_enabled() && ContextFlag::ShowMCPServers.is_enabled() {
        items.push(updateable_custom_item_without_checkmark(
            CustomAction::OpenMCPServerCollection,
            ctx,
        ));
    }

    Menu::new(menu_title(MessageId::MenuAi), items)
}

fn make_new_blocks_menu(ctx: &AppContext) -> Menu {
    let mut items = vec![
        updateable_custom_item_without_checkmark(CustomAction::ClearBlocks, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::SelectBlockAbove, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SelectBlockBelow, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SelectAllBlocks, ctx),
        MenuItem::Separator,
    ];
    items.push(updateable_custom_item_without_checkmark(
        CustomAction::ScrollToTopOfSelectedBlocks,
        ctx,
    ));
    items.push(updateable_custom_item_without_checkmark(
        CustomAction::ScrollToBottomOfSelectedBlocks,
        ctx,
    ));
    items.push(MenuItem::Separator);
    items.extend([
        updateable_custom_item_without_checkmark(CustomAction::CreateBlockPermalink, ctx),
        non_updateable_custom_item(CustomAction::ViewSharedBlocks, ctx),
        updateable_custom_item_without_checkmark(CustomAction::ToggleBookmarkBlock, ctx),
        updateable_custom_item_without_checkmark(CustomAction::FindWithinBlock, ctx),
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::CopyBlock, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CopyBlockCommand, ctx),
        updateable_custom_item_without_checkmark(CustomAction::CopyBlockOutput, ctx),
    ]);

    let debug_items = block_menu_debug_items();
    if !debug_items.is_empty() {
        items.push(MenuItem::Separator);
        items.extend(debug_items);
    }

    Menu::new(menu_title(MessageId::MenuBlocks), items)
}

fn make_new_drive_menu(ctx: &AppContext) -> Menu {
    let mut items = vec![
        updateable_custom_item_without_checkmark(CustomAction::NewPersonalWorkflow, ctx),
        updateable_custom_item_without_checkmark(CustomAction::NewPersonalNotebook, ctx),
        updateable_custom_item_without_checkmark(CustomAction::NewPersonalAIPrompt, ctx),
    ];
    items.push(updateable_custom_item_without_checkmark(
        CustomAction::NewPersonalEnvVars,
        ctx,
    ));
    items.extend([
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::NewTeamWorkflow, ctx),
        updateable_custom_item_without_checkmark(CustomAction::NewTeamNotebook, ctx),
        updateable_custom_item_without_checkmark(CustomAction::NewTeamAIPrompt, ctx),
    ]);
    items.push(updateable_custom_item_without_checkmark(
        CustomAction::NewTeamEnvVars,
        ctx,
    ));
    items.extend([
        MenuItem::Separator,
        updateable_custom_item_without_checkmark(CustomAction::ToggleWarpDrive, ctx),
        updateable_custom_item_without_checkmark(CustomAction::SearchDrive, ctx),
        updateable_custom_item_without_checkmark(CustomAction::OpenTeamSettings, ctx),
        updateable_custom_item_without_checkmark(CustomAction::OpenAIFactCollection, ctx),
        updateable_custom_item_without_checkmark(CustomAction::OpenMCPServerCollection, ctx),
    ]);

    items.push(updateable_custom_item_without_checkmark(
        CustomAction::SharePaneContents,
        ctx,
    ));

    if FeatureFlag::CreatingSharedSessions.is_enabled() {
        items.extend([
            MenuItem::Separator,
            updateable_custom_item_without_checkmark(CustomAction::ShareCurrentSession, ctx),
        ])
    }

    Menu::new(menu_title(MessageId::MenuDrive), items)
}

/// Returns [`MenuItem`]s that aid debugging to be included in the Block menu.
fn block_menu_debug_items() -> Vec<MenuItem> {
    let mut items = vec![];
    if FeatureFlag::ToggleBootstrapBlock.is_enabled() {
        items.push(toggle_bootstrap_block_menu_item());
    }

    items.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::DebugShowInBandCommandBlocks),
        move |ctx| {
            let handle = BlockVisibilitySettings::handle(ctx);
            handle.update(ctx, |block_visibility_settings, ctx| {
                let new_value = !block_visibility_settings
                    .should_show_in_band_command_blocks
                    .value();
                if let Err(e) = block_visibility_settings
                    .should_show_in_band_command_blocks
                    .set_value(new_value, ctx)
                {
                    log::error!("Failed to persist 'Show in-band command blocks' setting: {e}");
                }
            });
        },
        move |_props, ctx| {
            let name = if BlockVisibilitySettings::handle(ctx).read(ctx, |settings, _ctx| {
                *settings.should_show_in_band_command_blocks.value()
            }) {
                menu_title(MessageId::DebugHideInBandCommandBlocks)
            } else {
                menu_title(MessageId::DebugShowInBandCommandBlocks)
            };

            MenuItemPropertyChanges {
                name: Some(name),
                ..Default::default()
            }
        },
        None,
    )));

    items.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::DebugShowSshBlocks),
        move |ctx| {
            let handle = BlockVisibilitySettings::handle(ctx);
            handle.update(ctx, |block_visibility_settings, ctx| {
                let new_value = !block_visibility_settings.should_show_ssh_block.value();
                if let Err(e) = block_visibility_settings
                    .should_show_ssh_block
                    .set_value(new_value, ctx)
                {
                    log::error!("Failed to persist 'Show ssh command blocks' setting: {e}");
                }
            });
        },
        move |_props, ctx| {
            let name = if BlockVisibilitySettings::handle(ctx).read(ctx, |settings, _ctx| {
                *settings.should_show_ssh_block.value()
            }) {
                menu_title(MessageId::DebugHideSshBlocks)
            } else {
                menu_title(MessageId::DebugShowSshBlocks)
            };

            MenuItemPropertyChanges {
                name: Some(name),
                ..Default::default()
            }
        },
        None,
    )));

    items
}

fn toggle_bootstrap_block_menu_item() -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::DebugShowBootstrapBlock),
        move |ctx| {
            BlockVisibilitySettings::handle(ctx).update(ctx, |block_visibility_settings, ctx| {
                let new_value = !block_visibility_settings
                    .should_show_bootstrap_block
                    .value();
                if let Err(e) = block_visibility_settings
                    .should_show_bootstrap_block
                    .set_value(new_value, ctx)
                {
                    log::error!("Failed to persist 'Show bootstrap block' setting: {e}");
                }
            });
        },
        move |_props, ctx| {
            let name = if BlockVisibilitySettings::handle(ctx).read(ctx, |settings, _ctx| {
                *settings.should_show_bootstrap_block.value()
            }) {
                menu_title(MessageId::DebugHideBootstrapBlock)
            } else {
                menu_title(MessageId::DebugShowBootstrapBlock)
            };

            MenuItemPropertyChanges {
                name: Some(name),
                ..Default::default()
            }
        },
        None,
    ))
}

fn make_new_window_menu() -> Menu {
    Menu::new(
        menu_title(MessageId::MenuWindow),
        vec![
            MenuItem::Standard(StandardAction::Minimize),
            MenuItem::Standard(StandardAction::Zoom),
            MenuItem::Standard(StandardAction::ToggleFullScreen),
            MenuItem::Separator,
            MenuItem::Standard(StandardAction::BringAllToFront),
        ],
    )
}

fn debug_menu_items() -> Vec<MenuItem> {
    let mut debug_menu_items = vec![];

    if FeatureFlag::DebugMode.is_enabled() {
        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugEnableShellDebugMode),
            move |ctx| {
                DebugSettings::handle(ctx).update(ctx, |debug_settings, ctx| {
                    let new_value = !debug_settings.is_shell_debug_mode_enabled.value();
                    if let Err(e) = debug_settings
                        .is_shell_debug_mode_enabled
                        .set_value(new_value, ctx)
                    {
                        log::error!("Failed to persist 'Debug mode' setting: {e}");
                    }
                });
            },
            move |_props, ctx| {
                let name = if DebugSettings::handle(ctx).read(ctx, |settings, _ctx| {
                    *settings.is_shell_debug_mode_enabled.value()
                }) {
                    menu_title(MessageId::DebugDisableShellDebugMode)
                } else {
                    menu_title(MessageId::DebugEnableShellDebugMode)
                };

                MenuItemPropertyChanges {
                    name: Some(name),
                    ..Default::default()
                }
            },
            None,
        )));

        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugEnablePtyRecording),
            move |ctx| {
                DebugSettings::handle(ctx).update(ctx, |debug_settings, ctx| {
                    let new_value = !debug_settings.recording_mode.value();
                    let _ = debug_settings.recording_mode.set_value(new_value, ctx);
                });
            },
            move |_props, ctx| {
                let name = if DebugSettings::handle(ctx)
                    .read(ctx, |settings, _ctx| *settings.recording_mode.value())
                {
                    menu_title(MessageId::DebugDisablePtyRecording)
                } else {
                    menu_title(MessageId::DebugEnablePtyRecording)
                };

                MenuItemPropertyChanges {
                    name: Some(name),
                    ..Default::default()
                }
            },
            None,
        )));

        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugEnableInBandGenerators),
            move |ctx| {
                DebugSettings::handle(ctx).update(ctx, |debug_settings, ctx| {
                    let new_value = !debug_settings
                        .are_in_band_generators_for_all_sessions_enabled
                        .value();
                    if let Err(e) = debug_settings
                        .are_in_band_generators_for_all_sessions_enabled
                        .set_value(new_value, ctx)
                    {
                        log::error!("Failed to persist 'Enable in-band generators' setting: {e}");
                    }
                });
            },
            move |_props, ctx| {
                let name = if DebugSettings::handle(ctx).read(ctx, |settings, _ctx| {
                    *settings
                        .are_in_band_generators_for_all_sessions_enabled
                        .value()
                }) {
                    menu_title(MessageId::DebugDisableInBandGenerators)
                } else {
                    menu_title(MessageId::DebugEnableInBandGenerators)
                };

                MenuItemPropertyChanges {
                    name: Some(name),
                    ..Default::default()
                }
            },
            None,
        )));

        if !FeatureFlag::ToggleBootstrapBlock.is_enabled() {
            debug_menu_items.push(toggle_bootstrap_block_menu_item());
        }

        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugToggleNetworkStatus),
            move |ctx| ctx.dispatch_global_action("workspace:toggle_debug_network_status", &()),
            translated_updater(MessageId::DebugToggleNetworkStatus),
            None,
        )));

        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugExportDefaultSettingsCsv),
            move |ctx| {
                let default_settings = SettingsManager::handle(ctx).as_ref(ctx).default_values();
                let mut writer = Writer::from_writer(
                    File::create(
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join(SETTINGS_CSV_FILE_NAME),
                    )
                    .expect("Failed to create settings csv file"),
                );
                writer
                    .write_record(["storage_key", "value"])
                    .expect("Failed to write header record");
                for (storage_key, value) in default_settings {
                    log::debug!("Writing setting: {storage_key} = {value}");
                    writer
                        .write_record(&[storage_key, value])
                        .expect("Failed to write settings record");
                }
                let _ = writer.flush();
            },
            translated_updater(MessageId::DebugExportDefaultSettingsCsv),
            None,
        )));

        debug_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::DebugCreateAnonymousUser),
            move |ctx| ctx.dispatch_global_action("workspace:debug_create_anonymous_user", &()),
            translated_updater(MessageId::DebugCreateAnonymousUser),
            None,
        )));
    }

    if FeatureFlag::RuntimeFeatureFlags.is_enabled() {
        debug_menu_items.extend(runtime_flags_menu_items());
    }

    debug_menu_items
}

fn link_menu_item(message_id: MessageId, link: Cow<'static, str>) -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &menu_title(message_id),
        move |ctx| {
            ctx.open_url(&link);
        },
        translated_updater(message_id),
        None,
    ))
}

fn feedback_menu_item() -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::HelpSendFeedback),
        move |ctx| {
            ctx.dispatch_global_action("root_view:send_feedback", &());
        },
        translated_updater(MessageId::HelpSendFeedback),
        None,
    ))
}

fn make_new_help_menu() -> Menu {
    Menu::new(
        menu_title(MessageId::MenuHelp),
        vec![
            feedback_menu_item(),
            link_menu_item(
                MessageId::HelpWarpDocumentation,
                links::USER_DOCS_URL.into(),
            ),
            link_menu_item(MessageId::HelpGitHubIssues, links::GITHUB_ISSUES_URL.into()),
            link_menu_item(MessageId::HelpWarpSlackCommunity, links::SLACK_URL.into()),
        ],
    )
}

fn make_launch_config_menu_items(ctx: &mut AppContext) -> Vec<MenuItem> {
    let mut launch_config_menu_items = vec![];

    let launch_configs = WarpConfig::handle(ctx).as_ref(ctx).launch_configs();
    for config in launch_configs {
        launch_config_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
            &config.name,
            enclose!((config) move |ctx| {
                ctx.dispatch_global_action(
                    "root_view:open_launch_config",
                    &OpenLaunchConfigArg {
                        launch_config: config.clone(),
                        ui_location: LaunchConfigUiLocation::AppMenu,
                        open_in_active_window: false,
                    }
                );
                ctx.dispatch_global_action("workspace:save_app", &());
            }),
            no_updates,
            None,
        )));
    }

    if !launch_config_menu_items.is_empty() {
        launch_config_menu_items.push(MenuItem::Separator);
    }

    // TODO(vorporeal): use non_updateable_custom_item() here instead
    launch_config_menu_items.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::FileSaveNew),
        custom_action_dispatcher(CustomAction::SaveCurrentConfig),
        translated_updater(MessageId::FileSaveNew),
        custom_shortcut(CustomAction::SaveCurrentConfig),
    )));

    launch_config_menu_items
}

fn make_new_elements_menu_items(ctx: &AppContext) -> Vec<MenuItem> {
    // Dynamically assign the workspace:new_tab keystroke (cmd-t) to whichever item
    // matches the user's "Default mode for new sessions" setting. The non-default item
    // shows its dedicated keystroke instead.
    let mut new_elements_menu = vec![
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::FileNewWindow),
            open_new_window,
            translated_updater(MessageId::FileNewWindow),
            Some(Keystroke::parse("cmd-n").expect("Valid keystroke")),
        )),
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::FileNewTerminalTab),
            open_new_default_tab_or_window,
            move |_props: &MenuItemProperties, ctx: &mut AppContext| {
                let mut changes = MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::FileNewTerminalTab)),
                    ..Default::default()
                };
                let is_default_session_mode_agent =
                    AISettings::handle(ctx).read(ctx, |ai_settings, ctx| {
                        ai_settings.is_any_ai_enabled(ctx)
                            && ai_settings.default_session_mode(ctx) == DefaultSessionMode::Agent
                    });
                let trigger = if is_default_session_mode_agent {
                    Trigger::Custom(CustomAction::NewTerminalTab.into())
                } else {
                    Trigger::Custom(CustomAction::NewTab.into())
                };
                let binding = ctx
                    .get_key_bindings()
                    .find(|b| b.trigger == &trigger || b.original_trigger == Some(&trigger));
                if let Some(binding) = binding {
                    changes.keystroke = Some(bindings::trigger_to_keystroke(binding.trigger));
                }
                changes
            },
            Some(Keystroke::parse("cmd-t").expect("Valid keystroke")),
        )),
        MenuItem::Custom(CustomMenuItem::new(
            &menu_title(MessageId::FileNewAgentTab),
            open_new_agent_tab_or_window,
            move |_props: &MenuItemProperties, ctx: &mut AppContext| {
                let mut changes = MenuItemPropertyChanges {
                    name: Some(menu_title(MessageId::FileNewAgentTab)),
                    ..Default::default()
                };
                let (is_any_ai_enabled, is_default_session_mode_agent) = AISettings::handle(ctx)
                    .read(ctx, |ai_settings, ctx| {
                        let enabled = ai_settings.is_any_ai_enabled(ctx);
                        let agent = enabled
                            && ai_settings.default_session_mode(ctx) == DefaultSessionMode::Agent;
                        (enabled, agent)
                    });
                if !is_any_ai_enabled {
                    changes.disabled = Some(true);
                    return changes;
                }
                let trigger = if is_default_session_mode_agent {
                    Trigger::Custom(CustomAction::NewTab.into())
                } else {
                    Trigger::Custom(CustomAction::NewAgentTab.into())
                };
                let binding = ctx
                    .get_key_bindings()
                    .find(|b| b.trigger == &trigger || b.original_trigger == Some(&trigger));
                if let Some(binding) = binding {
                    changes.keystroke = Some(bindings::trigger_to_keystroke(binding.trigger));
                }
                changes
            },
            None,
        )),
        non_updateable_custom_item(CustomAction::NewFile, ctx),
    ];

    let reopen_session_action_updater =
        custom_action_updater(CustomAction::ReopenClosedSession, Box::new(|_| false));
    new_elements_menu.push(MenuItem::Custom(CustomMenuItem::new(
        &menu_title(MessageId::FileReopenClosedSession),
        |ctx| {
            UndoCloseStack::handle(ctx).update(ctx, |stack, ctx| {
                stack.undo_close(ctx);
            });
        },
        move |props, ctx| {
            let mut changes = reopen_session_action_updater(props, ctx);
            changes.disabled = Some(UndoCloseStack::handle(ctx).as_ref(ctx).is_empty());
            changes.name = Some(menu_title(MessageId::FileReopenClosedSession));
            changes
        },
        Some(Keystroke::parse("cmd-shift-T").expect("Valid keystroke")),
    )));

    new_elements_menu.push(MenuItem::Custom(CustomMenuItem::new_with_submenu(
        &menu_title(MessageId::FileLaunchConfigurations),
        |_| (),
        |_props, ctx| MenuItemPropertyChanges {
            name: Some(menu_title(MessageId::FileLaunchConfigurations)),
            submenu: Some(Some(make_launch_config_menu_items(ctx))),
            ..Default::default()
        },
        None,
        vec![],
    )));

    new_elements_menu
}

/// \return a callback that dispatches a CustomAction, appropriate for plugging into a CustomMenuItem.
fn custom_action_dispatcher(action: CustomAction) -> impl Fn(&mut AppContext) + 'static {
    move |ctx| {
        if let Some(wid) = WindowManager::handle(ctx).as_ref(ctx).active_window() {
            ctx.dispatch_custom_action(action, wid)
        }
    }
}

/// Dispatch events to open the user's default tab type in the active window
/// or make a new window if there is no active window.
fn open_new_default_tab_or_window(ctx: &mut AppContext) {
    if let Some(wid) = WindowManager::handle(ctx).as_ref(ctx).active_window() {
        ctx.dispatch_custom_action(CustomAction::NewTab, wid)
    } else {
        open_new_window(ctx)
    }
}

/// Dispatch events to open an agent tab in the active window
/// or make a new window if there is no active window.
fn open_new_agent_tab_or_window(ctx: &mut AppContext) {
    if let Some(wid) = WindowManager::handle(ctx).as_ref(ctx).active_window() {
        ctx.dispatch_custom_action(CustomAction::NewAgentTab, wid)
    } else {
        open_new_window(ctx)
    }
}

/// Dispatch event to open a new Warp window
fn open_new_window(ctx: &mut AppContext) {
    ctx.dispatch_global_action("root_view:open_new", &());
    ctx.dispatch_global_action("workspace:save_app", &());
}

/// No-op updater function for custom menu items that never change.
fn no_updates(_: &MenuItemProperties, _: &mut AppContext) -> MenuItemPropertyChanges {
    Default::default()
}

fn make_recent_repos_menu_items(ctx: &AppContext) -> Vec<MenuItem> {
    let recent_repos = generate_recent_repos_for_menu(ctx);

    if recent_repos.is_empty() {
        return vec![];
    }

    let home = dirs::home_dir().map(|p| p.display().to_string());

    recent_repos
        .into_iter()
        .map(|path| {
            let full_path = path.display().to_string();
            let display_path = user_friendly_path(&full_path, home.as_deref()).into_owned();

            MenuItem::Custom(CustomMenuItem::new(
                &display_path,
                move |ctx| {
                    ctx.dispatch_global_action("workspace:open_repository", &full_path);
                },
                no_updates,
                None,
            ))
        })
        .collect()
}

fn generate_recent_repos_for_menu(ctx: &AppContext) -> Vec<PathBuf> {
    PersistedWorkspace::handle(ctx)
        .as_ref(ctx)
        .workspaces()
        .sorted_by(WorkspaceMetadata::most_recently_navigated)
        .take(MAX_RECENT_REPOS_IN_MENU)
        .map(|cbm| cbm.path)
        .collect::<Vec<_>>()
}

/// \return a callback that updates a custom action based menu item based on the
/// current keybinding context:
/// 1) Whether the item should be enabled / disabled
/// 2) The current keystroke bound to the item (either the default or a custom one)
/// 3) The name of the item
///
/// This updated is called by the mac menu system before the menu is opened.
///
/// Note, that correctly disabling the key binding is important not just for limiting
/// what a user can do in a given context, but also because if the keybinding is not
/// disabled correctly, the mac menu system will capture key events when they should
/// be flowing into the underlying views (e.g. ctrl-c needs to flow directly to the
/// alt grid in emacs, whereas if our editor is active it's ok to dispatch a custom action)
fn custom_action_updater(
    action: CustomAction,
    checkmark_status: Box<CheckmarkStatusGetter>,
) -> impl Fn(&MenuItemProperties, &mut AppContext) -> MenuItemPropertyChanges + 'static {
    move |_props, ctx| {
        let mut changes = MenuItemPropertyChanges::default();

        // Apply i18n translation if available for non-English locales
        if let Some(message_id) = custom_action_to_message_id(action) {
            let locale = current_locale();
            if locale != crate::i18n::Locale::EnUs {
                changes.name = Some(tr(locale, message_id).into_owned());
            }
        }

        ctx.update_custom_action_binding(action.into(), |binding| {
            changes.disabled = Some(binding.is_none());
            if changes.name.is_none() {
                if let Some(binding) = binding {
                    if let Some(description) = binding.description {
                        changes.name = Some(
                            description
                                .resolve(ctx, bindings::MAC_MENUS_CONTEXT)
                                .into_owned(),
                        );
                    }
                }
            }
            if let Some(binding) = binding {
                changes.keystroke = Some(bindings::trigger_to_keystroke(binding.trigger));
            }
        });

        changes.checked = Some(checkmark_status(ctx));

        changes
    }
}
