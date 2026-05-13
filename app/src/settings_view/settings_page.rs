use crate::ui_components::blended_colors;
use core::fmt::{self, Display};
use itertools::Itertools as _;
use pathfinder_color::ColorU;
use std::borrow::Cow;
use std::collections::HashMap;

use super::{
    about_page::AboutPageView,
    ai_page::{AISettingsPageAction, AISettingsPageView},
    appearance_page::AppearanceSettingsPageView,
    billing_and_usage_page::BillingAndUsagePageView,
    code_page::CodeSettingsPageView,
    environments_page::EnvironmentsPageView,
    features_page::FeaturesPageView,
    keybindings::KeybindingsView,
    language_page::LanguagePageView,
    main_page::MainSettingsPageView,
    mcp_servers_page::MCPServersSettingsPageView,
    privacy_page::PrivacyPageView,
    referrals_page::ReferralsPageView,
    show_blocks_view::ShowBlocksView,
    teams_page::TeamsPageView,
    warp_drive_page::WarpDriveSettingsPageView,
    warpify_page::WarpifyPageView,
    SettingsSection,
};
use crate::i18n::{current_locale, Locale};
use crate::{
    appearance::Appearance,
    settings::CloudPreferencesSettings,
    themes::theme::Fill,
    ui_components::icons::Icon,
    view_components::{Dropdown, SubmittableTextInput},
};
use pathfinder_geometry::vector::vec2f;
use settings::Setting;
use warp_core::{
    settings::SyncToCloud,
    ui::{color::blend::Blend, theme::color::internal_colors},
};
use warpui::{
    elements::{
        new_scrollable::{ClippedAxisConfiguration, DualAxisConfig, SingleAxisConfig},
        Align, Border, ChildAnchor, ChildView, ClippedScrollStateHandle, ConstrainedBox, Container,
        CornerRadius, CrossAxisAlignment, Element, Empty, Expanded, Flex, Hoverable,
        MainAxisAlignment, MainAxisSize, MouseStateHandle, NewScrollable, OffsetPositioning,
        ParentAnchor, ParentElement, ParentOffsetBounds, Radius, SavePosition, ScrollTarget,
        ScrollToPositionMode, Shrinkable, SizeConstraintCondition, SizeConstraintSwitch, Stack,
        Text,
    },
    fonts::{Properties, Weight},
    platform::Cursor,
    ui_components::{
        button::{Button, ButtonVariant},
        components::{Coords, UiComponent, UiComponentStyles},
    },
    units::Pixels,
    Action, AppContext, SingletonEntity, ViewContext, ViewHandle,
};

pub const TOGGLE_BUTTON_RIGHT_PADDING: f32 = 5.;
pub const HEADER_PADDING: f32 = 15.;
pub const CONTENT_FONT_SIZE: f32 = 12.;
pub const SUBHEADER_MARGIN_BOTTOM: f32 = 4.;
pub const PAGE_TITLE_MARGIN_BOTTOM: f32 = 4.;
pub(super) const PAGE_PADDING: f32 = 28.;
pub(super) const HEADER_FONT_SIZE: f32 = 23.;
pub const SUBHEADER_FONT_SIZE: f32 = 16.;
const ALTERNATING_LIST_CLOSE_BUTTON_DIAMETER: f32 = 20.0;
const ALTERNATING_LIST_ITEM_PADDING: f32 = 8.0;
const GREY_TEXT_OPACITY: u8 = 60;
const MIN_PAGE_WIDTH: f32 = 520.;
const MAX_PAGE_WIDTH: f32 = 800.;

/// Left margin for top-level sidebar nav items (pages and umbrella labels).
pub(super) const NAV_ITEM_LEFT_MARGIN: f32 = 12.;

pub struct SettingsPage {
    pub section: SettingsSection,
    pub view_handle: SettingsPageViewHandle,
    button_state_handle: MouseStateHandle,
}

pub trait SettingsPageMeta {
    fn section() -> SettingsSection;

    /// Performs any work necessary to set up the page when it is selected by the user. Pages
    /// should respect the `allow_steal_focus` parameter, abstaining from focusing the page if it's
    /// false.
    fn on_page_selected(&mut self, _allow_steal_focus: bool, _ctx: &mut ViewContext<Self>) {
        log::info!("No updates for the selected view handle.");
    }

    fn should_render(&self, _ctx: &AppContext) -> bool;

    fn on_tab_pressed(&mut self, _ctx: &mut ViewContext<Self>) {}

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData;

    fn scroll_to_widget(&mut self, widget_id: &'static str);

    fn clear_highlighted_widget(&mut self);
}

/// Page enum lists all the pages that we want to support.
/// It is required to allow for SettingsPage struct be put in the collection (ie. vector).
#[derive(Clone)]
pub enum SettingsPageViewHandle {
    Main(ViewHandle<MainSettingsPageView>),
    Appearance(ViewHandle<AppearanceSettingsPageView>),
    Features(ViewHandle<FeaturesPageView>),
    SharedBlocks(ViewHandle<ShowBlocksView>),
    Keybindings(ViewHandle<KeybindingsView>),
    Language(ViewHandle<LanguagePageView>),
    About(ViewHandle<AboutPageView>),
    Code(ViewHandle<CodeSettingsPageView>),
    Teams(ViewHandle<TeamsPageView>),
    OzCloudAPIKeys(ViewHandle<super::platform_page::PlatformPageView>),
    Privacy(ViewHandle<PrivacyPageView>),
    Warpify(ViewHandle<WarpifyPageView>),
    Referrals(ViewHandle<ReferralsPageView>),
    AI(ViewHandle<AISettingsPageView>),
    CloudEnvironments(ViewHandle<EnvironmentsPageView>),
    BillingAndUsage(ViewHandle<BillingAndUsagePageView>),
    MCPServers(ViewHandle<MCPServersSettingsPageView>),
    LocalAIProvider(ViewHandle<super::local_ai_provider_page::LocalAIProviderPageView>),
    WarpDrive(ViewHandle<WarpDriveSettingsPageView>),
}

impl SettingsPageViewHandle {
    pub fn child_view(&self) -> Box<dyn Element> {
        use SettingsPageViewHandle::*;
        match self {
            Main(view_handle) => ChildView::new(view_handle).finish(),
            Appearance(view_handle) => ChildView::new(view_handle).finish(),
            Features(view_handle) => ChildView::new(view_handle).finish(),
            SharedBlocks(view_handle) => ChildView::new(view_handle).finish(),
            Keybindings(view_handle) => ChildView::new(view_handle).finish(),
            Language(view_handle) => ChildView::new(view_handle).finish(),
            About(view_handle) => ChildView::new(view_handle).finish(),
            Code(view_handle) => ChildView::new(view_handle).finish(),
            Teams(view_handle) => ChildView::new(view_handle).finish(),
            OzCloudAPIKeys(view_handle) => ChildView::new(view_handle).finish(),
            Privacy(view_handle) => ChildView::new(view_handle).finish(),
            Warpify(view_handle) => ChildView::new(view_handle).finish(),
            Referrals(view_handle) => ChildView::new(view_handle).finish(),
            AI(view_handle) => ChildView::new(view_handle).finish(),
            CloudEnvironments(view_handle) => ChildView::new(view_handle).finish(),
            BillingAndUsage(view_handle) => ChildView::new(view_handle).finish(),
            MCPServers(view_handle) => ChildView::new(view_handle).finish(),
            LocalAIProvider(view_handle) => ChildView::new(view_handle).finish(),
            WarpDrive(view_handle) => ChildView::new(view_handle).finish(),
        }
    }
}

impl From<ViewHandle<MCPServersSettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<MCPServersSettingsPageView>) -> Self {
        SettingsPageViewHandle::MCPServers(view_handle)
    }
}

impl From<ViewHandle<super::local_ai_provider_page::LocalAIProviderPageView>>
    for SettingsPageViewHandle
{
    fn from(
        view_handle: ViewHandle<super::local_ai_provider_page::LocalAIProviderPageView>,
    ) -> Self {
        SettingsPageViewHandle::LocalAIProvider(view_handle)
    }
}

impl SettingsPage {
    pub fn new<V>(view_handle: ViewHandle<V>) -> Self
    where
        V: SettingsPageMeta,
        ViewHandle<V>: Into<SettingsPageViewHandle>,
    {
        SettingsPage {
            section: V::section(),
            view_handle: view_handle.into(),
            button_state_handle: MouseStateHandle::default(),
        }
    }

    pub fn render_page_button(
        &self,
        appearance: &Appearance,
        match_data: MatchData,
        clicked: bool,
    ) -> Hoverable {
        appearance
            .ui_builder()
            .button(
                if clicked {
                    ButtonVariant::Accent
                } else {
                    ButtonVariant::Text
                },
                self.button_state_handle.clone(),
            )
            .with_text_label(
                self.section.localized_title(current_locale()) + &match_data.to_string(),
            )
            .with_style(
                UiComponentStyles::default()
                    .set_border_width(0.)
                    .set_margin(Coords::default().left(NAV_ITEM_LEFT_MARGIN))
                    .set_padding(Coords::uniform(8.)),
            )
            .build()
    }
}

#[derive(PartialEq, Eq)]
pub enum SettingsPageEvent {
    FocusModal,
    Pane(PaneEventWrapper),
    EnvironmentSetupModeSelectorToggled { is_open: bool },
    AgentAssistedEnvironmentModalToggled { is_open: bool },
}

/// Wrapper for pane events to avoid circular dependency with pane module.
/// The actual handling converts this to the real PaneEvent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaneEventWrapper {
    Close,
}

pub fn render_customer_type_badge(appearance: &Appearance, text: String) -> Box<dyn Element> {
    Container::new(
        Text::new_inline(text, appearance.ui_font_family(), appearance.ui_font_size())
            .with_color(
                appearance
                    .theme()
                    .background()
                    .blend(
                        &appearance
                            .theme()
                            .foreground()
                            .with_opacity(GREY_TEXT_OPACITY),
                    )
                    .into(),
            )
            .with_style(Properties::default().weight(Weight::Medium))
            .finish(),
    )
    .with_uniform_padding(4.)
    .with_background(
        appearance
            .theme()
            .background()
            .blend(&appearance.theme().foreground().with_opacity(25)),
    )
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(3.)))
    .with_margin_left(10.)
    .finish()
}

/// Adds padding to the sub header
pub fn render_sub_header(
    appearance: &Appearance,
    text_name: impl Into<Cow<'static, str>>,
    local_only_icon_state: Option<LocalOnlyIconState>,
) -> Box<dyn Element> {
    let mut sub_header = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Shrinkable::new(
                1.,
                build_sub_header(appearance, text_name, None)
                    .with_padding_bottom(HEADER_PADDING)
                    .finish(),
            )
            .finish(),
        );
    if let Some(LocalOnlyIconState::Visible {
        mouse_state,
        custom_tooltip,
    }) = local_only_icon_state
    {
        sub_header.add_child(
            Container::new(render_local_only_icon(
                appearance,
                mouse_state,
                custom_tooltip,
            ))
            .with_padding_top(3.)
            .finish(),
        );
    }
    sub_header.finish()
}

/// Contains only the sub header
pub fn build_sub_header(
    appearance: &Appearance,
    text_name: impl Into<Cow<'static, str>>,
    color_override: Option<Fill>,
) -> Container {
    let color = color_override.unwrap_or(appearance.theme().active_ui_text_color());
    let text_name = settings_display_text(text_name.into().into_owned());
    Container::new(
        Align::new(
            Text::new_inline(text_name, appearance.ui_font_family(), SUBHEADER_FONT_SIZE)
                .with_style(Properties::default().weight(Weight::Bold))
                .with_color(color.into())
                .finish(),
        )
        .left()
        .finish(),
    )
    .with_margin_bottom(SUBHEADER_MARGIN_BOTTOM)
}

pub fn render_sub_header_with_description(
    appearance: &Appearance,
    text_name: impl Into<Cow<'static, str>>,
    description: impl Into<Cow<'static, str>>,
) -> Box<dyn Element> {
    let description = settings_display_text(description.into().into_owned());
    Container::new(
        Flex::column()
            .with_child(build_sub_header(appearance, text_name, None).finish())
            .with_child(
                Align::new(
                    Text::new(description, appearance.ui_font_family(), CONTENT_FONT_SIZE)
                        .with_color(appearance.theme().nonactive_ui_text_color().into())
                        .finish(),
                )
                .left()
                .finish(),
            )
            .finish(),
    )
    .with_padding_bottom(HEADER_PADDING)
    .finish()
}

#[cfg_attr(target_family = "wasm", allow(unused))]
pub fn render_sub_sub_header(
    appearance: &Appearance,
    text_name: impl Into<Cow<'static, str>>,
    local_only_icon_state: Option<LocalOnlyIconState>,
) -> Box<dyn Element> {
    let text_name = settings_display_text(text_name.into().into_owned());
    let mut sub_sub_header = Flex::row().with_child(
        Container::new(
            Align::new(
                Text::new_inline(text_name, appearance.ui_font_family(), CONTENT_FONT_SIZE)
                    .with_style(Properties::default().weight(Weight::Bold))
                    .with_color(appearance.theme().active_ui_text_color().into())
                    .finish(),
            )
            .left()
            .finish(),
        )
        .with_padding_bottom(4.)
        .finish(),
    );
    if let Some(LocalOnlyIconState::Visible {
        mouse_state,
        custom_tooltip,
    }) = local_only_icon_state
    {
        sub_sub_header.add_child(render_local_only_icon(
            appearance,
            mouse_state.clone(),
            custom_tooltip,
        ));
    }
    sub_sub_header.finish()
}

pub fn render_custom_size_header(
    appearance: &Appearance,
    text_name: impl Into<Cow<'static, str>>,
    font_size: f32,
    color_override: Option<Fill>,
) -> Box<dyn Element> {
    let text_name = settings_display_text(text_name.into().into_owned());
    Flex::row()
        .with_child(
            Container::new(
                Align::new(
                    Text::new_inline(text_name, appearance.ui_font_family(), font_size)
                        .with_style(Properties::default().weight(Weight::Bold))
                        .with_color(
                            color_override
                                .unwrap_or(appearance.theme().active_ui_text_color())
                                .into(),
                        )
                        .finish(),
                )
                .left()
                .finish(),
            )
            .with_padding_bottom(4.)
            .finish(),
        )
        .finish()
}

pub fn render_separator(appearance: &Appearance) -> Box<dyn Element> {
    Container::new(Empty::new().finish())
        .with_border(Border::bottom(2.).with_border_fill(appearance.theme().outline()))
        .with_margin_bottom(HEADER_PADDING)
        .finish()
}

pub fn render_full_pane_width_ai_button(
    text: &str,
    is_any_ai_enabled: bool,
    mouse_state: MouseStateHandle,
    action: AISettingsPageAction,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let text = settings_display_text(text);
    let (text_color, bg, icon_bg) = if is_any_ai_enabled {
        (
            appearance
                .theme()
                .main_text_color(appearance.theme().background())
                .into(),
            internal_colors::neutral_3(appearance.theme()),
            appearance.theme().background(),
        )
    } else {
        (
            appearance.theme().disabled_ui_text_color().into(),
            internal_colors::neutral_2(appearance.theme()),
            appearance.theme().disabled_ui_text_color(),
        )
    };

    let mut button = Hoverable::new(mouse_state, |_| {
        Container::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .with_child(
                    Expanded::new(
                        1.,
                        appearance
                            .ui_builder()
                            .wrappable_text(text.clone(), true)
                            .with_style(UiComponentStyles {
                                font_size: Some(CONTENT_FONT_SIZE),
                                font_color: Some(text_color),
                                ..Default::default()
                            })
                            .build()
                            .finish(),
                    )
                    .finish(),
                )
                .with_child(
                    ConstrainedBox::new(
                        Icon::ChevronRight
                            .to_warpui_icon(appearance.theme().main_text_color(icon_bg))
                            .finish(),
                    )
                    .with_width(16.)
                    .with_height(16.)
                    .finish(),
                )
                .finish(),
        )
        .with_background(bg)
        .with_border(
            Border::new(1.).with_border_fill(internal_colors::neutral_4(appearance.theme())),
        )
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
        .with_horizontal_padding(16.)
        .with_vertical_padding(11.)
        .with_margin_bottom(12.)
        .finish()
    });

    if is_any_ai_enabled {
        button = button
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
            })
            .with_cursor(Cursor::PointingHand);
    }

    button.finish()
}

#[derive(Default)]
pub struct AdditionalInfo<T> {
    pub mouse_state: MouseStateHandle,
    pub on_click_action: Option<T>,
    pub secondary_text: Option<String>,
    pub tooltip_override_text: Option<String>,
}

#[derive(Default)]
pub enum ToggleState {
    #[default]
    Enabled,
    Disabled,
}

impl From<bool> for ToggleState {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

/// Whether to show an icon indicating a setting is not cloud-synced
#[derive(Default, Clone)]
pub enum LocalOnlyIconState {
    #[default]
    Hidden,
    Visible {
        mouse_state: MouseStateHandle,
        custom_tooltip: Option<String>,
    },
}

impl LocalOnlyIconState {
    /// Creates a `LocalOnlyIconState` for a given setting.
    ///
    /// This function determines whether to show an icon indicating that a setting
    /// is not cloud-synced based on the `SyncToCloud` value of the setting.
    ///
    /// # Arguments
    ///
    /// * `storage_key` - A string slice that holds the storage key for the setting.
    /// * `sync_to_cloud` - The `SyncToCloud` value for the setting.
    /// * `mouse_states` - A mutable reference to a `HashMap` storing `MouseStateHandle`s.
    ///
    /// # Returns
    ///
    /// Returns a `LocalOnlyIconState` enum variant:
    /// - `LocalOnlyIconState::Visible` with a `MouseStateHandle` if the setting is never synced to cloud.
    /// - `LocalOnlyIconState::Hidden` if the setting is synced to cloud.
    pub fn for_setting(
        storage_key: &str,
        sync_to_cloud: SyncToCloud,
        mouse_states: &mut HashMap<String, MouseStateHandle>,
        app: &AppContext,
    ) -> Self {
        if !*CloudPreferencesSettings::as_ref(app).settings_sync_enabled {
            // Only show the local-only icon if settings sync is enabled.
            return Self::Hidden;
        }

        match sync_to_cloud {
            SyncToCloud::Never => {
                let mouse_state = mouse_states
                    .entry(storage_key.to_string())
                    .or_default()
                    .clone();
                Self::Visible {
                    mouse_state,
                    custom_tooltip: None,
                }
            }
            _ => Self::Hidden,
        }
    }
}

pub fn render_info_icon<T: Clone + Action>(
    appearance: &Appearance,
    additional_info: AdditionalInfo<T>,
) -> Box<dyn Element> {
    let info_button = appearance
        .ui_builder()
        .info_button_with_tooltip(
            13.,
            additional_info
                .tooltip_override_text
                .unwrap_or("Click to learn more in docs".to_owned()),
            additional_info.mouse_state.clone(),
        )
        .on_click(move |ctx, _, _| {
            if let Some(on_click_action) = &additional_info.on_click_action {
                ctx.dispatch_typed_action(on_click_action.clone());
            }
        })
        .finish();

    Container::new(info_button)
        .with_margin_left(4.)
        // Since the icon is smaller than the font, we need some margin to be in alignment.
        .with_margin_top(1.5)
        .finish()
}

pub fn render_local_only_icon(
    appearance: &Appearance,
    mouse_state: MouseStateHandle,
    custom_tooltip: Option<String>,
) -> Box<dyn Element> {
    let info_button = appearance
        .ui_builder()
        .local_only_icon_with_tooltip(
            13.,
            custom_tooltip.unwrap_or_else(|| {
                settings_display_text("This setting is not synced to your other devices")
            }),
            mouse_state.clone(),
        )
        .finish();

    Container::new(info_button)
        .with_margin_left(4.)
        // Since the icon is smaller than the font, we need some margin to be in alignment.
        .with_margin_top(1.5)
        .finish()
}

pub fn render_body_item_label<T: Clone + Action>(
    label_text: String,
    label_color_override: Option<Fill>,
    additional_info: Option<AdditionalInfo<T>>,
    local_only_icon_state: LocalOnlyIconState,
    toggle_state: ToggleState,
    appearance: &Appearance,
) -> Box<dyn Element> {
    render_body_item_label_internal(
        label_text,
        None,
        label_color_override,
        additional_info,
        local_only_icon_state,
        toggle_state,
        appearance,
    )
}

pub fn render_body_item_label_with_icon<T: Clone + Action>(
    label_text: String,
    icon: Icon,
    label_color_override: Option<Fill>,
    additional_info: Option<AdditionalInfo<T>>,
    local_only_icon_state: LocalOnlyIconState,
    toggle_state: ToggleState,
    appearance: &Appearance,
) -> Box<dyn Element> {
    render_body_item_label_internal(
        label_text,
        Some(icon),
        label_color_override,
        additional_info,
        local_only_icon_state,
        toggle_state,
        appearance,
    )
}

pub fn render_body_item_label_internal<T: Clone + Action>(
    label_text: String,
    label_icon: Option<Icon>,
    label_color_override: Option<Fill>,
    additional_info: Option<AdditionalInfo<T>>,
    local_only_icon_state: LocalOnlyIconState,
    toggle_state: ToggleState,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let label_text = settings_display_text(label_text);
    let mut label = Flex::row();
    let label_color = match label_color_override {
        Some(color) => color,
        None => match toggle_state {
            ToggleState::Enabled => appearance.theme().active_ui_text_color(),
            ToggleState::Disabled => appearance.theme().disabled_ui_text_color(),
        },
    };
    let label_text = Text::new_inline(label_text, appearance.ui_font_family(), CONTENT_FONT_SIZE)
        .with_color(label_color.into());
    if let Some(icon) = label_icon {
        label.add_child(
            Container::new(
                ConstrainedBox::new(icon.to_warpui_icon(label_color).finish())
                    .with_width(16.)
                    .with_height(16.)
                    .finish(),
            )
            .with_margin_right(4.)
            .finish(),
        );
    }
    label.add_child(label_text.finish());

    let label = label.finish();
    if let Some(additional_info) = additional_info {
        // Construct a child element for the secondary text, if necessary, before
        // `additional_info` gets moved into `render_info_icon()`.
        let secondary_text_child =
            if let Some(secondary_text) = additional_info.secondary_text.clone() {
                let warp_theme = appearance.theme();
                Some(
                    appearance
                        .ui_builder()
                        .span(secondary_text)
                        .with_style(UiComponentStyles {
                            font_color: Some(
                                warp_theme
                                    .sub_text_color(warp_theme.surface_2())
                                    .into_solid(),
                            ),
                            margin: Some(Coords {
                                left: 8.,
                                ..Default::default()
                            }),
                            ..Default::default()
                        })
                        .build()
                        .finish(),
                )
            } else {
                None
            };

        let mut row = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(label)
            .with_child(render_info_icon(appearance, additional_info));
        if let LocalOnlyIconState::Visible {
            mouse_state,
            custom_tooltip,
        } = local_only_icon_state
        {
            row.add_child(render_local_only_icon(
                appearance,
                mouse_state,
                custom_tooltip,
            ));
        }
        if let Some(child) = secondary_text_child {
            row.add_child(child);
        }
        row.finish()
    } else if let LocalOnlyIconState::Visible {
        mouse_state,
        custom_tooltip,
    } = local_only_icon_state
    {
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(label)
            .with_child(render_local_only_icon(
                appearance,
                mouse_state,
                custom_tooltip,
            ))
            .finish()
    } else {
        label
    }
}

pub fn render_page_title(text: &str, size: f32, appearance: &Appearance) -> Box<dyn Element> {
    Container::new(
        Align::new(
            Text::new_inline(
                settings_display_text(text),
                appearance.ui_font_family(),
                size,
            )
            .with_style(Properties::default().weight(Weight::Bold))
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
        )
        .left()
        .finish(),
    )
    .with_margin_bottom(PAGE_TITLE_MARGIN_BOTTOM)
    .finish()
}

/// Renders a toggle with a label on the left and a toggle on the right,
/// including bottom padding.
pub fn render_body_item<T: Clone + Action>(
    label_text: String,
    additional_info: Option<AdditionalInfo<T>>,
    local_only_icon_state: LocalOnlyIconState,
    toggle_state: ToggleState,
    appearance: &Appearance,
    child_element: Box<dyn Element>,
    description_text: Option<String>,
) -> Box<dyn Element> {
    build_toggle_element(
        render_body_item_label(
            label_text,
            None,
            additional_info,
            local_only_icon_state,
            toggle_state,
            appearance,
        ),
        child_element,
        appearance,
        description_text,
    )
}

/// Builds a custom toggle with a label on the left and a toggle on the right.
pub fn build_toggle_element(
    name_element: Box<dyn Element>,
    toggle_element: Box<dyn Element>,
    appearance: &Appearance,
    description_text: Option<String>,
) -> Box<dyn Element> {
    let mut column = Flex::column();
    let header = Shrinkable::new(
        1.0,
        Container::new(Align::new(name_element).left().finish()).finish(),
    )
    .finish();
    let toggle = Container::new(toggle_element)
        .with_padding_right(TOGGLE_BUTTON_RIGHT_PADDING)
        .finish();

    let mut header_row = Container::new(
        Flex::row()
            .with_child(header)
            .with_child(toggle)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .finish(),
    );
    if description_text.is_none() {
        header_row = header_row.with_padding_bottom(HEADER_PADDING);
    }
    column.add_child(header_row.finish());
    if let Some(description_text) = description_text {
        let description = appearance
            .ui_builder()
            .paragraph(settings_display_text(description_text))
            .with_style(UiComponentStyles {
                font_color: Some(blended_colors::text_sub(
                    appearance.theme(),
                    appearance.theme().surface_1(),
                )),
                font_size: Some(12.),
                margin: Some(Coords {
                    top: 4.,
                    bottom: 0.,
                    left: 0.,
                    right: 0.,
                }),
                ..Default::default()
            })
            .build()
            .finish();
        column.add_child(
            Container::new(description)
                .with_margin_right(100.)
                .with_padding_bottom(HEADER_PADDING)
                .finish(),
        );
    }
    column.finish()
}

pub fn render_dropdown_item_label(
    label_text: String,
    secondary_text: Option<String>,
    local_only_icon_state: LocalOnlyIconState,
    color_override: Option<Fill>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let label = Text::new(label_text, appearance.ui_font_family(), CONTENT_FONT_SIZE)
        .with_color(
            color_override
                .unwrap_or(appearance.theme().active_ui_text_color())
                .into(),
        )
        .finish();
    let label = if let Some(secondary_text) = secondary_text {
        let warp_theme = appearance.theme();
        let secondary_text_child = appearance
            .ui_builder()
            .span(secondary_text)
            .with_style(UiComponentStyles {
                font_color: Some(
                    color_override
                        .unwrap_or(warp_theme.sub_text_color(warp_theme.surface_2()))
                        .into_solid(),
                ),
                margin: Some(Coords {
                    top: 4.,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with_soft_wrap()
            .build()
            .finish();

        Flex::column()
            .with_child(label)
            .with_child(secondary_text_child)
            .finish()
    } else {
        label
    };

    if let LocalOnlyIconState::Visible {
        mouse_state,
        custom_tooltip,
    } = local_only_icon_state
    {
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Shrinkable::new(1.0, label).finish())
            .with_child(render_local_only_icon(
                appearance,
                mouse_state,
                custom_tooltip,
            ))
            .finish()
    } else {
        label
    }
}

pub(crate) fn render_dropdown_item<T: Clone + Action>(
    appearance: &Appearance,
    label: &str,
    secondary_text: Option<&str>,
    dropdown_subtext: Option<Box<dyn Element>>,
    local_only_icon_state: LocalOnlyIconState,
    color_override: Option<Fill>,
    handle: &ViewHandle<Dropdown<T>>,
) -> Box<dyn Element> {
    let row = Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

    let dropdown_item_label = Align::new(render_dropdown_item_label(
        label.to_string(),
        secondary_text.map(|secondary_text| secondary_text.to_string()),
        local_only_icon_state,
        color_override,
        appearance,
    ))
    .left()
    .finish();

    let mut dropdown = Flex::column().with_child(ChildView::new(handle).finish());
    if let Some(dropdown_subtext) = dropdown_subtext {
        dropdown.add_child(dropdown_subtext);
    }

    row.with_child(
        Shrinkable::new(
            1.0,
            Container::new(dropdown_item_label)
                .with_margin_bottom(4.)
                .with_padding_right(16.)
                .finish(),
        )
        .finish(),
    )
    .with_child(dropdown.finish())
    .finish()
}

pub(crate) fn render_settings_info_banner(
    text: &str,
    subtext: Option<&str>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let icon = Container::new(
        ConstrainedBox::new(
            Icon::AlertCircle
                .to_warpui_icon(appearance.theme().active_ui_text_color())
                .finish(),
        )
        .with_width(16.)
        .with_height(16.)
        .finish(),
    )
    .with_margin_right(8.)
    .finish();

    let text = {
        let mut children = vec![Container::new(
            Text::new(
                text.to_string(),
                appearance.ui_font_family(),
                appearance.ui_font_size(),
            )
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
        )
        .finish()];

        if let Some(subtext) = subtext {
            children.push(
                Container::new(
                    Text::new(
                        subtext.to_string(),
                        appearance.ui_font_family(),
                        appearance.ui_font_size() - 1.,
                    )
                    .with_color(
                        appearance
                            .theme()
                            .sub_text_color(appearance.theme().background())
                            .into(),
                    )
                    .finish(),
                )
                .with_margin_top(4.)
                .finish(),
            );
        }

        Shrinkable::new(1.0, Flex::column().with_children(children).finish()).finish()
    };

    Container::new(
        Flex::row()
            .with_children(vec![icon, text])
            .with_main_axis_size(MainAxisSize::Max)
            .finish(),
    )
    .with_background_color(appearance.theme().accent_overlay().into())
    .with_uniform_padding(12.)
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    .finish()
}

const WORKSPACE_OVERRIDE_TOOLTIP_TEXT: &str =
    "This option is enforced by your organization's settings and cannot be customized.";

pub struct InputListItem<SettingsPageAction: Action + Clone> {
    pub item: String,
    pub mouse_state_handle: MouseStateHandle,
    pub on_remove_action: SettingsPageAction,
    pub is_disabled: bool,
    /// Must be pre-created (not inline during render) to preserve mouse tracking.
    pub tooltip_mouse_state: Option<MouseStateHandle>,
}
/// Renders a title, an input field to add new items and a list of already
/// added items.
///
/// TODO: standardize this and remove [`render_alternating_color_list`].
pub fn render_input_list<SettingsPageAction: Action + Clone>(
    title: Option<&str>,
    items: impl IntoIterator<Item = InputListItem<SettingsPageAction>>,
    handle: Option<&ViewHandle<SubmittableTextInput>>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let mut column = Flex::column();

    if let Some(title) = title {
        column.add_child(
            appearance
                .ui_builder()
                .span(title.to_string())
                .with_style(UiComponentStyles {
                    font_size: Some(CONTENT_FONT_SIZE),
                    ..Default::default()
                })
                .build()
                .finish(),
        );
    }

    if let Some(handle) = handle {
        column.add_child(ChildView::new(handle).finish());
    }

    let background = appearance.theme().surface_1();
    let peekable = items.into_iter().peekable();
    for item in peekable {
        let disabled = item.is_disabled;
        let row_element = render_alternating_color_list_item(
            background,
            item.item,
            item.mouse_state_handle,
            item.on_remove_action,
            disabled,
            appearance,
        );
        let row_element = if let Some(tooltip_mouse_state) = item.tooltip_mouse_state {
            render_workspace_override_row_tooltip(row_element, tooltip_mouse_state, appearance)
        } else {
            row_element
        };
        let container = Container::new(row_element).with_margin_bottom(4.);
        column.add_child(container.finish());
    }

    column.finish()
}

pub fn render_alternating_color_list<
    ListItem: Display,
    SettingsPageAction: Action + Clone,
    F: Fn(usize) -> SettingsPageAction,
>(
    body: &mut Flex,
    patterns: &[ListItem],
    mouse_states: &[MouseStateHandle],
    create_action: F,
    appearance: &Appearance,
) {
    debug_assert!(
        mouse_states.len() >= patterns.len(),
        "mouse_states length ({}) is less than patterns length ({})",
        mouse_states.len(),
        patterns.len()
    );
    for (i, pattern) in patterns.iter().enumerate() {
        let background = if i % 2 == 0 {
            internal_colors::fg_overlay_1(appearance.theme())
        } else {
            Fill::Solid(ColorU::transparent_black())
        };

        body.add_child(render_alternating_color_list_item::<SettingsPageAction>(
            background,
            pattern.to_string(),
            mouse_states[i].clone(),
            create_action(i),
            false,
            appearance,
        ));
    }
}

fn render_workspace_override_row_tooltip(
    child: Box<dyn Element>,
    mouse_state: MouseStateHandle,
    appearance: &Appearance,
) -> Box<dyn Element> {
    Hoverable::new(mouse_state, |state| {
        let mut stack = Stack::new().with_child(child);
        if state.is_hovered() {
            let tooltip = appearance
                .ui_builder()
                .tool_tip(WORKSPACE_OVERRIDE_TOOLTIP_TEXT.to_string())
                .build()
                .finish();
            stack.add_positioned_child(
                tooltip,
                OffsetPositioning::offset_from_parent(
                    vec2f(0., -4.),
                    ParentOffsetBounds::Unbounded,
                    ParentAnchor::TopLeft,
                    ChildAnchor::BottomLeft,
                ),
            );
        }
        stack.finish()
    })
    .finish()
}

fn render_alternating_color_list_item<SettingsPageAction: Action + Clone>(
    background: impl Into<Fill>,
    item_label: String,
    mouse_state: MouseStateHandle,
    action: SettingsPageAction,
    disabled: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let mut remove_button = appearance
        .ui_builder()
        .close_button(ALTERNATING_LIST_CLOSE_BUTTON_DIAMETER, mouse_state);

    if disabled {
        remove_button = remove_button.disabled();
    }

    let mut remove_button = remove_button.build();
    if !disabled {
        remove_button =
            remove_button.on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()));
    }
    let remove_button = remove_button.finish();

    let background = background.into();
    let font_color = if disabled {
        appearance.theme().disabled_text_color(background)
    } else {
        appearance.theme().foreground()
    };

    Container::new(
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_children([
                Shrinkable::new(
                    1.,
                    Align::new(
                        appearance
                            .ui_builder()
                            .wrappable_text(item_label, true)
                            .with_style(UiComponentStyles {
                                font_color: Some(font_color.into_solid()),
                                font_family_id: Some(appearance.monospace_font_family()),
                                font_size: Some(appearance.ui_font_size()),
                                ..Default::default()
                            })
                            .build()
                            .finish(),
                    )
                    .left()
                    .finish(),
                )
                .finish(),
                Container::new(remove_button)
                    .with_margin_left(ALTERNATING_LIST_ITEM_PADDING)
                    .finish(),
            ])
            .finish(),
    )
    .with_background(background)
    .with_uniform_padding(ALTERNATING_LIST_ITEM_PADDING)
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    // The bottom has a bit of extra padding b/c lines of text have more space above the text
    // than below. This visually balances that to make it look vertically centered.
    .with_padding_bottom(ALTERNATING_LIST_ITEM_PADDING + 2.)
    .finish()
}

/// Adds a setting (e.g., "Background opacity") to the parent flex if it is supported on the current platform. Returns
/// true if the setting was added to the flex, false if not.
///
/// This is the default method to use when rendering a setting in the settings menu, across all pages
/// (Appearance, Features, etc).
pub fn add_setting<F>(
    parent_flex: &mut Flex,
    setting_model: &impl Setting,
    setting_element: F,
) -> bool
where
    F: FnOnce() -> Box<dyn Element>,
{
    if setting_model.is_supported_on_current_platform() {
        parent_flex.add_child(setting_element());
        true
    } else {
        false
    }
}

/// Structured contents of a settings tab page. This type breaks all the content into
/// [`SettingsWidget`]s.
pub(super) enum PageType<V: warpui::View> {
    /// A page where the contents cannot be separated for showing search results. If any part
    /// matches the search query, the whole page must show. The whole page is one big
    /// [`SettingsWidget`].
    ///
    /// The vertical and horizontal scroll states are optional to let Monolith pages
    /// handle and render their own scrollable elements.
    Monolith {
        widget: Box<dyn SettingsWidget<View = V>>,
        title: Option<String>,
        filter: bool,
        vertical_scroll_state: Option<ClippedScrollStateHandle>,
        horizontal_scroll_state: Option<ClippedScrollStateHandle>,
        min_page_width: f32,
    },
    /// A page which is a series of [`SettingsWidget`]s that don't fall under sub-categories.
    Uncategorized {
        widgets: Vec<Box<dyn SettingsWidget<View = V>>>,
        title: Option<String>,
        filter: Vec<usize>,
        vertical_scroll_state: ClippedScrollStateHandle,
        horizontal_scroll_state: ClippedScrollStateHandle,
        highlighted_widget_id: Option<&'static str>,
        min_page_width: f32,
    },
    /// A page which is a series of [`SettingsWidget`]s that fall under sub-categories.
    Categorized {
        categories: Vec<Category<V>>,
        title: Option<String>,
        filter: Vec<Vec<usize>>,
        vertical_scroll_state: ClippedScrollStateHandle,
        horizontal_scroll_state: ClippedScrollStateHandle,
        highlighted_widget_id: Option<&'static str>,
        min_page_width: f32,
    },
}

/// Some settings pages break down into a collection of smaller widgets while others are
/// "monoliths". The way the matches are presented differs between them.
#[derive(Clone, Copy, Debug)]
pub(crate) enum MatchData {
    /// The monoliths use the Uncounted variant to indicate that they match a search
    /// term. Alternatively, we may use this variant for non-monolithic pages if we shouldn't
    /// bother counting the number of matches, say if the search query has become empty.
    Uncounted(bool),
    /// Used for non-monolithic pages when we want to display a specific count for the number of
    /// matches to a search query.
    Countable(usize),
}

impl MatchData {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            MatchData::Countable(n) => *n > 0,
            MatchData::Uncounted(flag) => *flag,
        }
    }
}

impl Display for MatchData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchData::Countable(n) => write!(f, " ({n})"),
            MatchData::Uncounted(_) => write!(f, ""),
        }
    }
}

impl From<bool> for MatchData {
    fn from(value: bool) -> Self {
        MatchData::Uncounted(value)
    }
}

impl From<usize> for MatchData {
    fn from(value: usize) -> Self {
        MatchData::Countable(value)
    }
}

impl<V: warpui::View> PageType<V> {
    /// A page where the contents cannot be separated for showing search results. If any part
    /// matches the search query, the whole page must show. The whole page is one big
    /// [`SettingsWidget`].
    pub(super) fn new_monolith(
        widget: impl SettingsWidget<View = V> + 'static,
        title: Option<String>,
        is_dual_scrollable: bool,
    ) -> Self {
        let (vertical_scroll_state, horizontal_scroll_state) = if is_dual_scrollable {
            (
                Some(ClippedScrollStateHandle::default()),
                Some(ClippedScrollStateHandle::default()),
            )
        } else {
            (None, None)
        };

        Self::Monolith {
            filter: true,
            widget: Box::new(widget),
            title,
            vertical_scroll_state,
            horizontal_scroll_state,
            min_page_width: MIN_PAGE_WIDTH,
        }
    }

    /// A page which is a series of [`SettingsWidget`]s that don't fall under sub-categories.
    pub(super) fn new_uncategorized(
        widgets: Vec<Box<dyn SettingsWidget<View = V>>>,
        title: Option<String>,
    ) -> Self {
        Self::Uncategorized {
            filter: widgets.iter().enumerate().map(|(i, _)| i).collect(),
            widgets,
            title,
            vertical_scroll_state: Default::default(),
            horizontal_scroll_state: Default::default(),
            highlighted_widget_id: Default::default(),
            min_page_width: MIN_PAGE_WIDTH,
        }
    }

    /// A page which is a series of [`SettingsWidget`]s that fall under sub-categories.
    pub(super) fn new_categorized(categories: Vec<Category<V>>, title: Option<String>) -> Self {
        Self::Categorized {
            filter: categories
                .iter()
                .map(|category| {
                    category
                        .widgets
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i)
                        .collect()
                })
                .collect(),
            categories,
            title,
            vertical_scroll_state: Default::default(),
            horizontal_scroll_state: Default::default(),
            highlighted_widget_id: Default::default(),
            min_page_width: MIN_PAGE_WIDTH,
        }
    }

    /// Apply the search query by matching against all the widgets and storing the results.
    /// Uses all-words matching: every word in the query must appear somewhere in the
    /// widget's search terms (but not necessarily contiguously).
    pub(super) fn update_filter(&mut self, query: &str, app: &AppContext) -> MatchData {
        /// Returns true if every whitespace-delimited word in `query` appears
        /// somewhere in `terms` (case-insensitive). An empty query matches everything.
        fn search_terms_match(terms: &str, query: &str) -> bool {
            if query.is_empty() {
                return true;
            }
            let terms_lower = terms.to_lowercase();
            query
                .to_lowercase()
                .split_whitespace()
                .all(|word| terms_lower.contains(word))
        }
        match self {
            Self::Monolith { widget, filter, .. } => {
                *filter =
                    widget.should_render(app) && search_terms_match(widget.search_terms(), query);
                (*filter).into()
            }
            Self::Uncategorized {
                widgets, filter, ..
            } => {
                *filter = widgets
                    .iter()
                    .enumerate()
                    .filter_map(|(i, widget)| {
                        (widget.should_render(app)
                            && search_terms_match(widget.search_terms(), query))
                        .then_some(i)
                    })
                    .collect();
                if query.is_empty() {
                    MatchData::Uncounted(true)
                } else {
                    filter.len().into()
                }
            }
            Self::Categorized {
                categories, filter, ..
            } => {
                *filter = categories
                    .iter()
                    .map(|category| {
                        category
                            .widgets
                            .iter()
                            .enumerate()
                            .filter_map(|(i, widget)| {
                                (widget.should_render(app)
                                    && search_terms_match(widget.search_terms(), query))
                                .then_some(i)
                            })
                            .collect_vec()
                    })
                    .collect();
                if query.is_empty() {
                    MatchData::Uncounted(true)
                } else {
                    filter
                        .iter()
                        .map(|indices| indices.len())
                        .sum::<usize>()
                        .into()
                }
            }
        }
    }

    pub fn scroll_to_widget(&mut self, widget_id: &'static str) {
        match self {
            Self::Monolith { .. } => {}
            Self::Uncategorized {
                vertical_scroll_state: scrollable_state,
                highlighted_widget_id,
                ..
            }
            | Self::Categorized {
                vertical_scroll_state: scrollable_state,
                highlighted_widget_id,
                ..
            } => {
                *highlighted_widget_id = Some(widget_id);
                scrollable_state.scroll_to_position(ScrollTarget {
                    position_id: widget_id.to_string(),
                    mode: ScrollToPositionMode::FullyIntoView,
                })
            }
        }
    }

    pub fn clear_highlighted_widget(&mut self) {
        match self {
            Self::Monolith { .. } => {}
            Self::Uncategorized {
                highlighted_widget_id,
                ..
            }
            | Self::Categorized {
                highlighted_widget_id,
                ..
            } => {
                *highlighted_widget_id = None;
            }
        }
    }

    /// Set the minimum page width for narrow panes.
    pub fn set_min_page_width(&mut self, width: f32) {
        match self {
            Self::Monolith { min_page_width, .. }
            | Self::Uncategorized { min_page_width, .. }
            | Self::Categorized { min_page_width, .. } => {
                *min_page_width = width;
            }
        }
    }

    /// Apply the filter we saved from the last matching of the search query to return only the
    /// relevant results.
    pub(super) fn get_filtered(&self) -> FilteredPageType<'_, V> {
        match self {
            Self::Monolith {
                widget,
                filter,
                title,
                vertical_scroll_state,
                horizontal_scroll_state,
                ..
            } => FilteredPageType::Monolith {
                widget: filter.then_some(widget.as_ref()),
                title: title.as_deref(),
                vertical_scroll_state: vertical_scroll_state.clone(),
                horizontal_scroll_state: horizontal_scroll_state.clone(),
            },
            Self::Uncategorized {
                widgets,
                filter,
                title,
                vertical_scroll_state,
                horizontal_scroll_state,
                highlighted_widget_id,
                ..
            } => FilteredPageType::Uncategorized {
                widgets: filter.iter().map(|i| widgets[*i].as_ref()).collect(),
                title: title.as_deref(),
                vertical_scroll_state: vertical_scroll_state.clone(),
                horizontal_scroll_state: horizontal_scroll_state.clone(),
                highlighted_widget_id: *highlighted_widget_id,
            },
            Self::Categorized {
                categories,
                filter,
                title,
                vertical_scroll_state,
                horizontal_scroll_state,
                highlighted_widget_id,
                ..
            } => FilteredPageType::Categorized {
                categories: filter
                    .iter()
                    .enumerate()
                    .filter(|(_, indices)| !indices.is_empty())
                    .map(|(i, indices)| {
                        let category = &categories[i];
                        FilteredCategory {
                            title: category.title.clone(),
                            subtitle: category.subtitle.clone(),
                            widgets: indices
                                .iter()
                                .map(|i| category.widgets[*i].as_ref())
                                .collect(),
                        }
                    })
                    .collect(),
                title: title.as_deref(),
                vertical_scroll_state: vertical_scroll_state.clone(),
                horizontal_scroll_state: horizontal_scroll_state.clone(),
                highlighted_widget_id: *highlighted_widget_id,
            },
        }
    }

    fn get_scroll_states(
        &self,
    ) -> (
        Option<ClippedScrollStateHandle>,
        Option<ClippedScrollStateHandle>,
    ) {
        match self.get_filtered() {
            FilteredPageType::Monolith {
                vertical_scroll_state,
                horizontal_scroll_state,
                ..
            } => (vertical_scroll_state, horizontal_scroll_state),
            FilteredPageType::Uncategorized {
                vertical_scroll_state,
                horizontal_scroll_state,
                ..
            } => (Some(vertical_scroll_state), Some(horizontal_scroll_state)),
            FilteredPageType::Categorized {
                vertical_scroll_state,
                horizontal_scroll_state,
                ..
            } => (Some(vertical_scroll_state), Some(horizontal_scroll_state)),
        }
    }

    #[cfg_attr(not(any(target_os = "linux", target_os = "freebsd")), allow(dead_code))]
    pub fn scroll_by(&self, delta: Pixels) {
        match self {
            PageType::Monolith {
                vertical_scroll_state: Some(scrollable_state),
                ..
            }
            | PageType::Uncategorized {
                vertical_scroll_state: scrollable_state,
                ..
            }
            | PageType::Categorized {
                vertical_scroll_state: scrollable_state,
                ..
            } => scrollable_state.scroll_by(delta),
            _ => {}
        }
    }

    pub(super) fn render_page(&self, view: &V, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let page = match self.get_filtered() {
            FilteredPageType::Monolith { widget, title, .. } => {
                let mut page = Empty::new().finish();
                if let Some(widget) = widget {
                    if widget.should_render(app) {
                        if let Some(title) = title {
                            let col = Flex::column()
                                .with_child(render_page_title(title, HEADER_FONT_SIZE, appearance))
                                .with_child(widget.render_widget(view, false, appearance, app));
                            page = col.finish();
                        } else {
                            page = widget.render_widget(view, false, appearance, app);
                        }
                    }
                }
                page
            }
            FilteredPageType::Uncategorized {
                widgets,
                title,
                highlighted_widget_id,
                ..
            } => {
                let mut page = Flex::column();
                if let Some(title) = title {
                    page.add_child(render_page_title(title, HEADER_FONT_SIZE, appearance));
                }
                for widget in widgets {
                    let highlighted =
                        highlighted_widget_id.is_some_and(|id| id == widget.widget_id());
                    if widget.should_render(app) {
                        page.add_child(widget.render_widget(view, highlighted, appearance, app));
                    }
                }
                page.finish()
            }
            FilteredPageType::Categorized {
                categories,
                title,
                highlighted_widget_id,
                ..
            } => {
                let mut page = Flex::column();
                if let Some(title) = title {
                    page.add_child(render_page_title(title, HEADER_FONT_SIZE, appearance));
                }
                let num_categories = categories.len();
                for (i, category) in categories.into_iter().enumerate() {
                    if !category.title.is_empty() {
                        if let Some(subtitle) = category.subtitle {
                            page.add_child(render_sub_header_with_description(
                                appearance,
                                category.title,
                                subtitle,
                            ));
                        } else {
                            page.add_child(render_sub_header(appearance, category.title, None));
                        }
                    }
                    for widget in &category.widgets {
                        let highlighted =
                            highlighted_widget_id.is_some_and(|id| id == widget.widget_id());
                        if widget.should_render(app) {
                            page.add_child(widget.render_widget(
                                view,
                                highlighted,
                                appearance,
                                app,
                            ));
                        }
                    }
                    if i < num_categories - 1 {
                        page.add_child(render_separator(appearance));
                    }
                }
                page.finish()
            }
        };

        Container::new(
            Align::new(
                ConstrainedBox::new(page)
                    .with_max_width(MAX_PAGE_WIDTH)
                    .finish(),
            )
            .top_center()
            .finish(),
        )
        .with_uniform_padding(PAGE_PADDING)
        .finish()
    }

    fn wrap_dual_scrollable(
        &self,
        view: &V,
        horizontal_scroll_state: ClippedScrollStateHandle,
        vertical_scroll_state: ClippedScrollStateHandle,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        // Get the minimum page width from the PageType configuration
        let min_width = match self {
            Self::Monolith { min_page_width, .. }
            | Self::Uncategorized { min_page_width, .. }
            | Self::Categorized { min_page_width, .. } => *min_page_width,
        };

        // Use SizeConstraintSwitch to add horizontal scrolling only when width < min_width
        let switch = SizeConstraintSwitch::new(
            NewScrollable::vertical(
                SingleAxisConfig::Clipped {
                    handle: vertical_scroll_state.clone(),
                    child: Align::new(self.render_page(view, app))
                        .top_center()
                        .finish(),
                },
                theme.nonactive_ui_detail().into(),
                theme.active_ui_detail().into(),
                warpui::elements::Fill::None,
            )
            .finish(),
            vec![(
                SizeConstraintCondition::WidthLessThan(min_width),
                NewScrollable::horizontal_and_vertical(
                    DualAxisConfig::Clipped {
                        horizontal: ClippedAxisConfiguration {
                            handle: horizontal_scroll_state,
                            max_size: None,
                            stretch_child: true,
                        },
                        vertical: ClippedAxisConfiguration {
                            handle: vertical_scroll_state,
                            max_size: None,
                            stretch_child: false,
                        },
                        child: Align::new(
                            ConstrainedBox::new(self.render_page(view, app))
                                .with_max_width(min_width)
                                .finish(),
                        )
                        .top_center()
                        .finish(),
                    },
                    theme.nonactive_ui_detail().into(),
                    theme.active_ui_detail().into(),
                    warpui::elements::Fill::None,
                )
                .finish(),
            )],
        )
        .finish();

        Flex::column()
            .with_child(Expanded::new(1., switch).finish())
            .finish()
    }

    pub fn render(&self, view: &V, app: &AppContext) -> Box<dyn Element> {
        if let (Some(vertical_scroll_state), Some(horizontal_scroll_state)) =
            self.get_scroll_states()
        {
            self.wrap_dual_scrollable(view, horizontal_scroll_state, vertical_scroll_state, app)
        } else {
            self.render_page(view, app)
        }
    }
}

/// The results from a [`PageType`] with only matching [`SettingsWidget`]s.
pub(super) enum FilteredPageType<'a, V: warpui::View> {
    Monolith {
        widget: Option<&'a dyn SettingsWidget<View = V>>,
        title: Option<&'a str>,
        vertical_scroll_state: Option<ClippedScrollStateHandle>,
        horizontal_scroll_state: Option<ClippedScrollStateHandle>,
    },
    Uncategorized {
        widgets: Vec<&'a dyn SettingsWidget<View = V>>,
        title: Option<&'a str>,
        vertical_scroll_state: ClippedScrollStateHandle,
        horizontal_scroll_state: ClippedScrollStateHandle,
        highlighted_widget_id: Option<&'static str>,
    },
    Categorized {
        categories: Vec<FilteredCategory<'a, V>>,
        title: Option<&'a str>,
        vertical_scroll_state: ClippedScrollStateHandle,
        horizontal_scroll_state: ClippedScrollStateHandle,
        highlighted_widget_id: Option<&'static str>,
    },
}

/// A grouping of related [`SettingsWidget`]s that fall under the same sub-header.
pub(super) struct Category<V: warpui::View> {
    title: String,
    subtitle: Option<String>,
    widgets: Vec<Box<dyn SettingsWidget<View = V>>>,
}

impl<V: warpui::View> Category<V> {
    pub(super) fn new(
        title: impl Into<String>,
        widgets: Vec<Box<dyn SettingsWidget<View = V>>>,
    ) -> Self {
        Self {
            title: settings_display_text(title.into()),
            subtitle: None,
            widgets,
        }
    }

    pub(super) fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(settings_display_text(subtitle.into()));
        self
    }
}

/// A [`Category`] with only the results which match a search query.
pub(super) struct FilteredCategory<'a, V: warpui::View> {
    pub(super) title: String,
    pub(super) subtitle: Option<String>,
    pub(super) widgets: Vec<&'a dyn SettingsWidget<View = V>>,
}

/// Widgets are pieces of renderable settings modal content which can be associated with search
/// content to match against.
pub(super) trait SettingsWidget {
    /// Which View (settings page) this widget belongs to.
    type View: warpui::View;

    fn static_widget_id() -> &'static str
    where
        Self: Sized,
    {
        std::any::type_name::<Self>()
    }

    fn widget_id(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// The terms to match search queries against.
    fn search_terms(&self) -> &str;

    fn should_render(&self, _app: &AppContext) -> bool {
        true
    }

    fn render_widget(
        &self,
        view: &Self::View,
        highlighted: bool,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let mut content = self.render(view, appearance, app);
        if highlighted {
            content = Container::new(content)
                .with_border(Border::all(1.).with_border_fill(appearance.theme().accent()))
                .with_background(internal_colors::accent_overlay_1(appearance.theme()))
                .with_horizontal_padding(8.)
                .finish()
        }
        SavePosition::new(content, self.widget_id()).finish()
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element>;
}

/// Builds a standardized button for resetting a setting to its default value.
/// Callers should add an `on_click` handler and add the button to the UI below
/// the setting.
pub(super) fn build_reset_button(
    appearance: &Appearance,
    mouse_state: MouseStateHandle,
    changed_from_default: bool,
) -> Button {
    let theme = appearance.theme();
    appearance
        .ui_builder()
        .reset_button(
            ButtonVariant::Text,
            mouse_state,
            changed_from_default,
            theme.disabled_text_color(theme.background()).into(),
        )
        .with_style(UiComponentStyles {
            padding: Some(Coords::default().bottom(HEADER_PADDING).top(5.)),
            font_size: Some(appearance.ui_font_size() * 0.8),
            ..Default::default()
        })
        .with_text_label(settings_display_text("Reset to default"))
}

/// Returns a `'static` translated string by leaking memory.
/// Use only for UI strings in long-lived views (settings pages).
pub(crate) fn settings_display_text_static(text: &'static str) -> &'static str {
    if current_locale() != Locale::ZhCn {
        return text;
    }
    if let Some(translated) = settings_zh_cn_literal(text) {
        Box::leak(translated.to_owned().into_boxed_str())
    } else {
        text
    }
}

pub(crate) fn settings_display_text(text: impl Into<String>) -> String {
    let text = text.into();
    if current_locale() != Locale::ZhCn {
        return text;
    }

    if let Some(translated) = settings_zh_cn_literal(&text) {
        translated.to_owned()
    } else if let Some(count) = text.strip_suffix(" credits remaining") {
        format!("剩余 {count} 额度")
    } else if let Some(count) = text.strip_suffix(" credit remaining") {
        format!("剩余 {count} 额度")
    } else if let Some(count) = text.strip_suffix(" credits") {
        format!("{count} 额度")
    } else if let Some(count) = text.strip_suffix(" credit") {
        format!("{count} 额度")
    } else if let Some(count) = text.strip_suffix(" teammates") {
        format!("{count} 位队友")
    } else if let Some(discount) = text.strip_suffix("% off") {
        format!("{discount}% 优惠")
    } else if let Some(date) = text.strip_prefix("Usage resets on ") {
        format!("用量将在 {date} 重置")
    } else if let Some(date) = text.strip_prefix("Resets ") {
        format!("{date} 重置")
    } else if let Some(url) = text.strip_prefix("Open image at ") {
        format!("打开镜像 {url}")
    } else if let Some(domain) = text
        .strip_prefix("Allow Warp users with an @")
        .and_then(|text| text.strip_suffix(" email to find and join the team."))
    {
        format!("允许使用 @{domain} 邮箱的 Warp 用户发现并加入此团队。")
    } else if let Some(amount) = text
        .strip_prefix("When enabled, auto reload will automatically purchase ")
        .and_then(|t| {
            t.strip_suffix(
                " credits when your add-on credit balance reaches 100 credits remaining.",
            )
        })
    {
        let amount = if amount == "your selected" {
            "您选择的"
        } else {
            amount
        };
        format!("启用后，当您的额外额度余额剩余 100 时，自动重新加载将自动购买 {amount} 额度。")
    } else if let Some(rest) = text.strip_prefix("Discovered ") {
        if let Some(suffix) = rest.strip_suffix(" chunks") {
            format!("已发现 {suffix} 个代码块")
        } else {
            text
        }
    } else if let Some(rest) = text.strip_prefix("Syncing - ") {
        format!("同步中 - {rest}")
    } else if let Some(max_rows) = text
        .strip_prefix(
            "Setting the limit above 100k lines may impact performance. Maximum rows supported is ",
        )
        .and_then(|rest| rest.strip_suffix("."))
    {
        format!("将限制设置为超过 10 万行可能影响性能。支持的最大行数为 {max_rows}。")
    } else {
        text
    }
}

fn settings_zh_cn_literal(text: &str) -> Option<&'static str> {
    Some(match text {
        "This setting is not synced to your other devices" => "此设置不会同步到您的其他设备",
        "Reset to default" => "恢复默认",
        "Default" => "默认",
        "Custom" => "自定义",
        "Cancel" => "取消",
        "Clear" => "清除",
        "Save" => "保存",
        "Search" => "搜索",
        "No settings match your search." => "没有匹配的设置。",
        "You may want to try using different keywords or checking for any possible typos." => {
            "可以尝试使用其他关键词，或检查是否有拼写错误。"
        }

        "Account" => "账户",
        "Appearance" => "外观",
        "Features" => "功能",
        "Keyboard shortcuts" => "键盘快捷键",
        "Privacy" => "隐私",
        "Billing and usage" => "账单和用量",
        "Codebase Indexing" => "代码库索引",
        "Code Editor and Review" => "代码编辑器和代码审查",
        "MCP Servers" => "MCP 服务器",
        "Search MCP Servers" => "搜索 MCP 服务器",
        "Edit config" => "编辑配置",
        "Set up" => "设置",
        "Oz Cloud API Keys" => "Oz Cloud API 密钥",

        // ── Billing and usage Page ──────────────────────────────────────────
        "Continue using premium models beyond your plan's limits. Usage is charged in $20 increments up to your spending limit, with any remaining balance charged on your scheduled billing date." => {
            "继续使用超出套餐限制的高级模型。用量将以 20 美元为增量收费，最高至您的支出限额，任何剩余余额将在您的预定计费日期收取。"
        }
        "Ask a team admin to enable overages for more AI usage." => "请联系团队管理员启用超额用量以获得更多 AI 用量。",
        "Auto reload is disabled, as the next reload would exceed your monthly spend limit. Increase your limit to use auto reload." => {
            "自动重新加载已禁用，因为下一次重新加载将超出您的每月支出限额。请提高限额以使用自动重新加载。"
        }
        "Restricted due to billing issue. Update your payment method to purchase add-on credits." => {
            "由于账单问题受限。请更新您的付款方式以购买额外额度。"
        }
        "Auto reload is disabled due to recent failed reload. Please update your payment method and try again." => {
            "由于最近一次重新加载失败，自动重新加载已禁用。请更新您的付款方式并重试。"
        }
        "Reloading would exceed your monthly limit. " => "重新加载将超出您的每月限额。",
        "Increase your limit" => "提高限额",
        " to continue." => " 以继续。",
        "Usage reporting is currently limited" => "用量报告目前受限",
        "Enterprise credit usage isn't fully available in this view yet. For the most accurate spend tracking, " => {
            "企业版额度用量在此视图中尚不完全可用。为了获得最准确的支出跟踪，"
        }
        "visit the admin panel" => "访问管理面板",
        "Enterprise credit usage isn't fully available in this view yet. Contact a team admin for detailed usage reporting." => {
            "企业版额度用量在此视图中尚不完全可用。请联系团队管理员获取详细的用量报告。"
        }
        "Add-on credits are purchased in prepaid packages that roll over each billing cycle and expire after one year. The more you purchase, the better the per-credit rate. Once your base plan credits are used, add-on credits will be consumed." => {
            "额外额度通过预付包购买，可在每个计费周期结转，并在一年后过期。购买越多，单价越低。一旦您的基础套餐额度用完，将消耗额外额度。"
        }
        "Add-on credits are purchased in prepaid packages that roll over each billing cycle and expire after one year. The more you purchase, the better the per-credit rate. Once your base plan credits are used, add-on credits will be consumed. Purchased add-on credits are shared across your team." => {
            "额外额度通过预付包购买，可在每个计费周期结转，并在一年后过期。购买越多，单价越低。一旦您的基础套餐额度用完，将消耗额外额度。购买的额外额度由您的团队共享。"
        }
        "Purchased add-on credits are shared across your team." => "购买的额外额度由您的团队共享。",
        "Cloud agent trial" => "云智能体试用",
        "Overage spending limit" => "超额支出限额",
        "Monthly spending limit" => "每月支出限额",
        "Load more" => "加载更多",
        "Failed to update workspace settings" => "更新工作空间设置失败",
        "Successfully purchased add-on credits" => "成功购买额外额度",
        "New agent" => "新建智能体",
        "Buy more" => "购买更多",
        "Monthly overage spending limit" => "每月超额支出限额",
        "Not set" => "未设置",
        "Sets the monthly overage spending limit beyond the plan amount" => "设置超出套餐金额的每月超额支出限额",
        "Add-on credits" => "额外额度",
        "Switch to the Build plan" => "切换到 Build 套餐",
        " to purchase add-on credits." => " 以购买额外额度。",
        "Upgrade to the Build plan" => "升级到 Build 套餐",
        "Contact your Account Executive for more add-on credits." => "请联系您的客户经理获取更多额外额度。",
        "Contact a team admin to purchase add-on credits." => "请联系团队管理员购买额外额度。",
        "Monthly spend limit" => "每月支出限额",
        "Sets the monthly limit spent on add-on credits" => "设置每月购买额外额度的限额",
        "Purchased this month" => "本月已购买",
        "Auto reload" => "自动重新加载",
        "1 credit remaining" => "剩余 1 额度",
        "1 credit" => "1 额度",
        "Please enter a valid currency amount" => "请输入有效的货币金额",
        "Please enter a price between $0.01 and $10,000,000" => "请输入 0.01 美元到 10,000,000 美元之间的价格",
        "Warp will prevent use of premium models when this dollar limit is reached. Resets on a monthly basis." => {
            "达到此美元限额时，Warp 将阻止使用高级模型。每月重置一次。"
        }
        "Note that AI credits made near your chosen limit may exceed it by a few dollars." => {
            "请注意，在接近您选择的限额时产生的 AI 额度可能会超出限额几美元。"
        }
        "Create and manage API keys to allow other Oz cloud agents to access your Warp account." => {
            "创建并管理 API 密钥，允许其他 Oz 云端智能体访问您的 Warp 账户。"
        }
        "Create and manage API keys to allow other Oz cloud agents to access your Warp account.\nFor more information, visit the " => {
            "创建并管理 API 密钥，允许其他 Oz 云端智能体访问您的 Warp 账户。\n欲了解更多信息，请访问 "
        }
        "For more information, visit the " => "欲了解更多信息，请访问 ",
        "Documentation." => "文档。",
        "Name" => "名称",
        "Key" => "密钥",
        "Scope" => "作用域",
        "Created" => "创建时间",
        "Last used" => "最后使用",
        "Expires at" => "过期时间",
        "Never" => "永不过期",
        "Personal" => "个人",
        "Team" => "团队",
        "No API Keys" => "暂无 API 密钥",
        "Create a key to manage external access to Warp" => "创建密钥以管理 Warp 的外部访问",
        "New API key" => "新建 API 密钥",
        "+ Create API Key" => "+ 创建 API 密钥",
        "Warp API Key" => "Warp API 密钥",
        "Done" => "完成",
        "Create key" => "创建密钥",
        "Creating..." => "正在创建...",
        "Type" => "类型",
        "Expiration" => "过期时间",
        "1 day" => "1 天",
        "30 days" => "30 天",
        "90 days" => "90 天",
        "This API key is tied to your user and can make requests against your Warp account." => {
            "此 API 密钥绑定到您的用户，可向您的 Warp 账户发起请求。"
        }
        "This API key is tied to your team and can make requests on behalf of your team." => {
            "此 API 密钥绑定到您的团队，可代表您的团队发起请求。"
        }
        "Compact" => "紧凑模式",
        "Warp" => "Warp 模式",
        "Classic" => "经典模式",
        "Light" => "浅色主题",
        "Dark" => "深色主题",
        "Input type" => "输入类型",

        "Themes" => "主题",
        "Icon" => "图标",
        "Window" => "窗口",
        "Input" => "输入",
        "Panes" => "面板",
        "Blocks" => "块",
        "Text" => "文本",
        "Cursor" => "光标",
        "Tabs" => "标签页",
        "Fullscreen apps" => "全屏应用",
        "General" => "常规",
        "Session" => "会话",
        "Keys" => "按键",
        "Text Editing" => "文本编辑",
        "Terminal Input" => "终端输入",
        "Terminal" => "终端",
        "Notifications" => "通知",
        "Workflows" => "工作流",
        "System" => "系统",
        "Usage" => "用量",
        "Plan" => "方案",
        "Referrals" => "推荐",

        "compact mode" | "Compact Mode" | "Compact mode" => "紧凑模式",
        "themes: sync with OS" | "Sync with OS" => "与系统同步主题",
        "cursor blink" | "Cursor blink" => "光标闪烁",
        "jump to bottom of block button" => "跳到底部块按钮",
        "block dividers" | "Block dividers" => "块分隔线",
        "dim inactive panes" | "Dim inactive panes" => "淡化非活动面板",
        "tab indicators" | "Tab indicators" => "标签页指示器",
        "focus follows mouse" | "Focus follows mouse" => "焦点跟随鼠标",
        "zen mode" | "Zen mode" => "禅模式",
        "Always show tab bar" => "始终显示标签栏",
        "Hide tab bar if fullscreen" => "全屏时隐藏标签栏",
        "Only show tab bar on hover" => "悬停时显示标签栏",
        "Start Input at the Top" => "输入框置顶",
        "Pin Input to the Top" => "固定输入框到顶部",
        "Pin Input to the Bottom" => "固定输入框到底部",
        "Toggle Input Mode (Warp/Classic)" => "切换输入模式（Warp/经典）",
        "Show code review button in tab bar" => "在标签栏显示代码审查按钮",
        "Hide code review button in tab bar" => "在标签栏隐藏代码审查按钮",
        "Match terminal" => "匹配终端",
        "View all available system fonts" => "查看所有可用系统字体",
        "Line height" => "行高",
        "Font weight" => "字体粗细",
        "Font size (px)" => "字号 (px)",
        "Notebook font size" => "笔记本字体大小",
        "Ligatures may reduce performance" => "连字可能会降低性能",
        "Cursor type is disabled in Vim mode" => "Vim 模式下禁用光标类型设置",

        "Copy on Select" => "选中即复制",
        "Autocomplete quotes and brackets" => "自动补全引号和括号",
        "Restore windows, tabs, and panes on startup" => "启动时恢复窗口、标签页和面板",
        "Scroll reporting" => "滚动上报",
        "Open completions menu as you type" => "输入时打开补全菜单",
        "Directory path" => "目录路径",
        "Executable path" => "可执行文件路径",
        "Command corrections" => "命令纠错",
        "Error underlining" => "错误下划线",
        "Syntax highlighting" => "语法高亮",
        "Audible terminal bell" => "终端提示音",
        "Autosuggestions" => "自动建议",
        "Enable SSH Wrapper" => "启用 SSH Wrapper",
        "Link tooltip" => "链接提示",
        "Vim unnamed register" => "Vim 未命名寄存器",
        "Vim status bar" => "Vim 状态栏",
        "Make Warp default terminal" => "将 Warp 设为默认终端",
        "Left Option as Meta" => "左 Option 作为 Meta",
        "Right Option as Meta" => "右 Option 作为 Meta",
        "Left Alt as Meta" => "左 Alt 作为 Meta",
        "Right Alt as Meta" => "右 Alt 作为 Meta",
        "Tab key behavior" => "Tab 键行为",
        "Keybinding" => "快捷键",
        "Click to set global hotkey" => "点击设置全局热键",
        "Change keybinding" => "更改快捷键",
        "seconds" => "秒",
        "Not supported on Wayland. " => "Wayland 不支持。",
        "Autohides on loss of keyboard focus" => "失去键盘焦点时自动隐藏",
        "See docs." => "查看文档。",

        "Code" => "代码",
        "Initialization Settings" => "初始化设置",
        "Codebase indexing" => "代码库索引",
        "Index new folders by default" => "默认索引新文件夹",
        "Initialized / indexed folders" => "已初始化/索引的文件夹",
        "No folders have been initialized yet." => "尚未初始化任何文件夹。",
        "No index created" => "尚未创建索引",
        "INDEXING" => "索引",
        "LSP SERVERS" => "LSP 服务器",
        "Restart server" => "重启服务器",
        "View logs" => "查看日志",
        "Failed" => "失败",

        "Secret redaction" => "密钥脱敏",
        "Custom secret redaction" => "自定义密钥脱敏",
        "Add regex pattern" => "添加正则表达式",
        "e.g. \"Google API Key\"" => "例如 \"Google API Key\"",
        "Name (optional)" => "名称（可选）",
        "Regex pattern" => "正则表达式",
        "Add regex" => "添加正则",
        "Invalid regex" => "无效正则",
        "Telemetry" => "遥测",
        "Data management" => "数据管理",
        "Privacy policy" => "隐私政策",
        "No enterprise regexes have been configured by your organization." => {
            "您的组织尚未配置企业正则表达式。"
        }
        "ZDR" => "ZDR",

        "Sign up" => "注册",
        "MODELS" => "模型",
        "Agents" => "智能体",
        "Models" => "模型",
        "PERMISSIONS" => "权限",
        "None" => "无",
        "Exclusive theme" => "专属主题",
        "Keycaps + stickers" => "键帽和贴纸",
        "T-shirt" => "T 恤",
        "Notebook" => "笔记本",
        "Baseball cap" => "棒球帽",
        "Hoodie" => "连帽衫",
        "Premium Hydro Flask" => "高级保温瓶",
        "Backpack" => "背包",
        "Free" => "免费",
        "Compare plans" => "比较方案",
        "Up to date" => "已是最新",
        "Version" => "版本",
        "Log out" => "退出登录",
        "Contact support" => "联系支持",
        "Manage billing" => "管理账单",
        "Check for updates" => "检查更新",
        "Relaunch Warp" => "重启 Warp",
        "Update available" => "有可用更新",
        "Updating" => "正在更新",
        "Installed update" => "已安装更新",
        "Update manually" => "手动更新",
        "Settings Sync" => "设置同步",

        "Overview" => "概览",
        "Usage History" => "用量历史",
        "View details on overage usage" => "查看超额用量详情",
        "Enable premium model usage overages" => "启用高级模型超额用量",
        "Premium model usage overages are enabled" => "高级模型超额用量已启用",
        "Premium model usage overages are not enabled" => "高级模型超额用量未启用",
        "A to Z" => "A 到 Z",
        "Z to A" => "Z 到 A",
        "Usage ascending" => "用量升序",
        "Usage descending" => "用量降序",
        "Cloud agent trial" => "云端智能体试用",
        "New agent" => "新建智能体",
        "Buy more" => "购买更多",
        "Add-on credits" => "附加额度",
        "Monthly spend limit" => "每月消费上限",
        "Purchased this month" => "本月已购买",
        "One-time purchase" => "一次性购买",
        "Total overages" => "总超额用量",
        "Last 30 days" => "最近 30 天",
        "No usage history" => "暂无用量历史",
        "Sort by" => "排序方式",
        "Overage spending limit" => "超额消费上限",
        "Monthly spending limit" => "每月消费上限",
        "Unlimited" => "无限制",
        "Credits" => "额度",
        "Warp Agent" => "Warp 智能体",
        "To use AI features, please create an account." => "要使用 AI 功能，请先创建账户。",
        "Next Command" => "下一条命令",
        "Prompt Suggestions" => "提示词建议",
        "Suggested Code Banners" => "建议代码横幅",
        "Natural Language Autosuggestions" => "自然语言自动建议",
        "Shared Block Title Generation" => "共享块标题生成",
        "Commit & Pull Request Generation" => "提交和拉取请求生成",
        "Profiles" => "配置",
        "Context window (tokens)" => "上下文窗口（Token）",
        "Toolbar layout" => "工具栏布局",
        "Read only" => "只读",
        "Supervised" => "受监督",
        "Allow in specific directories" => "允许在特定目录中使用",
        "Add Profile" => "添加配置",
        "Show model picker in prompt" => "在提示词中显示模型选择器",
        "Restricted due to billing issue" => "因账单问题受限",
        "e.g. ~/code-repos/repo" => "例如 ~/code-repos/repo",
        "Commands, comma separated" => "命令，用逗号分隔",
        "e.g. ls .*" => "例如 ls .*",
        "e.g. rm .*" => "例如 rm .*",
        "Select MCP servers" => "选择 MCP 服务器",
        "Agent decides" => "智能体决定",
        "Always allow" => "始终允许",
        "Always ask" => "始终询问",
        "Ask on first write" => "首次写入时询问",
        "Never ask" => "从不询问",
        "Ask unless auto-approve" => "除非自动批准否则询问",
        "On" => "开启",
        "Off" => "关闭",
        "None" => "无",

        "Let AI suggest the next command to run based on your command history, outputs, and common workflows." => {
            "让 AI 根据您的命令历史、输出和常见工作流建议下一条运行的命令。"
        }
        "Let AI suggest natural language prompts, as inline banners in the input, based on recent commands and their outputs." => {
            "让 AI 根据最近的命令及其输出，在输入框中以行内横幅的形式建议自然语言提示词。"
        }
        "Let AI suggest code diffs and queries as inline banners in the blocklist, based on recent commands and their outputs." => {
            "让 AI 根据最近的命令及其输出，在块列表中以行内横幅的形式建议代码差异和查询。"
        }
        "Let AI suggest natural language autosuggestions, based on recent commands and their outputs." => {
            "让 AI 根据最近的命令及其输出，提供自然语言自动建议。"
        }
        "Let AI generate a title for your shared block based on the command and output." => {
            "让 AI 根据命令和输出为您的共享块生成标题。"
        }
        "Let AI generate commit messages and pull request titles and descriptions." => {
            "让 AI 生成提交信息以及拉取请求的标题和描述。"
        }
        "Set the boundaries for how your Agent operates. Choose what it can access, how much autonomy it has, and when it must ask for your approval. You can also fine-tune behavior around natural language input, codebase awareness, and more." => {
            "为您的智能体设定操作边界。选择它可以访问的内容、它拥有的自主程度以及何时必须征得您的批准。您还可以微调有关自然语言输入、代码库感知等方面的行为。"
        }
        "Profiles let you define how your Agent operates — from the actions it can take and when it needs approval, to the models it uses for tasks like coding and planning. You can also scope them to individual projects." => {
            "配置允许您定义智能体的操作方式——从它可以采取的操作和何时需要批准，到它用于编码和规划等任务的模型。您还可以将它们的应用范围限制在单个项目。"
        }
        "This model serves as the primary engine behind the Warp Agent. It powers most interactions and invokes other models for tasks like planning or code generation when necessary. Warp may automatically switch to alternate models based on model availability or for auxiliary tasks such as conversation summarization." => {
            "该模型作为 Warp 智能体背后的主要引擎。它驱动大多数交互，并在必要时调用其他模型来执行规划或代码生成等任务。Warp 可能会根据模型可用性或会话摘要等辅助任务自动切换到备选模型。"
        }
        "Apply code diffs" => "应用代码差异",
        "Read files" => "读取文件",
        "Execute commands" => "执行命令",
        "Interact with running commands" => "与运行中的命令交互",
        "Call MCP servers" => "调用 MCP 服务器",
        "Some of your permissions are managed by your workspace." => "您的某些权限由您的工作空间管理。",
        "Allow the Warp Agent to generate an outline of your codebase that can be used for context. No code is ever stored on our servers. " => {
            "允许 Warp 智能体生成代码库的大纲，以便用作上下文。我们的服务器上不会存储任何代码。"
        }
        "You haven't added any MCP servers yet. Once you do, you'll be able to control how much autonomy the Warp Agent has when interacting with them. " => {
            "您尚未添加任何 MCP 服务器。添加后，您将能够控制 Warp 智能体在与其交互时的自主程度。"
        }
        "Add a server" => "添加服务器",
        " or " => " 或 ",
        "learn more about MCPs." => "了解有关 MCP 的更多信息。",
        "MCP allowlist" => "MCP 白名单",
        "Allow the Warp Agent to call these MCP servers." => "允许 Warp 智能体调用这些 MCP 服务器。",
        "MCP denylist" => "MCP 黑名单",
        "The Warp Agent will always ask for permission before calling any MCP servers on this list." => {
            "在调用此列表中的任何 MCP 服务器之前，Warp 智能体将始终请求许可。"
        }
        "Input" => "输入",
        "Command denylist" => "命令黑名单",
        "Regular expressions to match commands that the Warp Agent should always ask permission to execute." => {
            "匹配 Warp 智能体执行前应始终请求许可的命令的正则表达式。"
        }
        "Command allowlist" => "命令白名单",
        "Regular expressions to match commands that can be automatically executed by the Warp Agent." => {
            "匹配可由 Warp 智能体自动执行的命令的正则表达式。"
        }
        "Directory allowlist" => "目录白名单",
        "Give the agent file access to certain directories." => "授予智能体对某些目录的文件访问权限。",
        "Search by name or by keys (ex. \"cmd d\")" => "按名称或按键搜索（例如 \"cmd d\"）",
        "Your credit limit is prorated because you joined midway through the billing cycle." => {
            "由于您是在账单周期中途加入，额度会按比例折算。"
        }
        "This credit limit is prorated because this user joined midway through the billing cycle." => {
            "由于该用户是在账单周期中途加入，额度会按比例折算。"
        }

        "Teams" => "团队",
        "Team name" => "团队名称",
        "Team members" | "Team Members" => "团队成员",
        "Invite by Link" => "通过链接邀请",
        "Invite by Email" => "通过邮箱邀请",
        "Restrict by domain" => "按域名限制",
        "Make team discoverable" => "允许发现团队",
        "Create a team" => "创建团队",
        "Or, join an existing team within your company" => "或者，加入您公司内现有的团队",
        "Leave team" => "离开团队",
        "Leave Team" => "离开团队",
        "Delete team" => "删除团队",
        "Delete Team" => "删除团队",
        "Create" => "创建",
        "Create Team" => "创建团队",
        "Transfer team ownership?" => "转让团队所有权？",
        "Cancel invite" => "取消邀请",
        "Transfer ownership" => "转让所有权",
        "Transfer" => "转让",
        "Demote from admin" => "取消管理员身份",
        "Promote to admin" => "提升为管理员",
        "Remove from team" => "从团队移除",
        "Remove domain" => "移除域名",
        "Reset links" => "重置链接",
        "PAST DUE" => "逾期",
        "UNPAID" => "未付款",
        "EXPIRED" => "已过期",
        "PENDING" => "待处理",
        "OWNER" => "所有者",
        "ADMIN" => "管理员",
        "Manage plan" => "管理方案",
        "Open admin panel" => "打开管理面板",
        "Upgrade to Build" => "升级到 Build",
        "Upgrade to Turbo plan" => "升级到 Turbo 方案",
        "Upgrade to Lightspeed plan" => "升级到 Lightspeed 方案",
        "Free plan usage limits" => "免费方案用量限制",
        "Plan usage limits" => "方案用量限制",
        "Shared Notebooks" => "共享笔记本",
        "Shared Workflows" => "共享工作流",
        "Join" => "加入",
        "1 teammate" => "1 位队友",
        "Domains, comma separated" => "域名，用逗号分隔",
        "Emails, comma separated" => "邮箱地址，用逗号分隔",
        "Set" => "设置",
        "Invite" => "邀请",
        "Your new team name" => "您的新团队名称",
        "Contact Admin to request access" => "联系管理员申请访问权限",
        "You appear to be offline." => "您似乎处于离线状态。",
        "Failed to load invite link." => "无法加载邀请链接。",
        "Join this team and start collaborating on workflows, notebooks, and more." => {
            "加入此团队，开始协作处理工作流、笔记本等内容。"
        }
        "Allow Warp users with the same email domain as you to find and join the team." => {
            "允许与您使用相同邮箱域名的 Warp 用户发现并加入此团队。"
        }

        "Copy link" => "复制链接",
        "Unshare" => "取消共享",
        "Send" => "发送",
        "Sending" => "正在发送",
        "Loading" => "正在加载",
        "Link copied" => "链接已复制",

        "Search by name or by keys (ex. \"cmd d\")" => "按名称或按键搜索（例如 \"cmd d\"）",
        "This shortcut conflicts with other keybinds" => "此快捷键与其他键绑定冲突",
        "Keyboard shortcuts are not synced to the cloud" => "键盘快捷键不会同步到云端",

        "Subshells" => "子 Shell",
        "SSH" => "SSH",
        "e.g. ~/code-repos/repo" => "例如 ~/code-repos/repo",
        "Commands, comma separated" => "命令，用逗号分隔",
        "e.g. ls .*" => "例如 ls .*",
        "e.g. rm .*" => "例如 rm .*",
        "Third party CLI agents" => "第三方 CLI 智能体",
        "Commands that enable the toolbar" => "启用工具栏的命令",
        " to get more AI usage." => " 以获得更多 AI 用量。",
        " for more AI usage." => " 以获得更多 AI 用量。",
        " to use your own API keys." => " 以使用您自己的 API 密钥。",
        "Encountered an incorrect detection? " => "检测结果不正确？",
        "Show coding agent toolbar" => "显示编码智能体工具栏",
        "Show a toolbar with quick actions when running coding agents like " => "运行 ",
        ", " => "、",
        ", or " => " 或 ",
        "." => " 等编码智能体时，显示带快捷操作的工具栏。",
        "Auto show/hide Rich Input based on agent status" => "根据智能体状态自动显示/隐藏富输入",
        "Requires the Warp plugin for your coding agent" => "需要为您的编码智能体安装 Warp 插件",
        "Auto open Rich Input when a coding agent session starts" => {
            "编码智能体会话开始时自动打开富输入"
        }
        "Auto dismiss Rich Input after prompt submission" => "提交提示词后自动关闭富输入",
        "Characters considered part of a word" => "单词界定字符",
        "New window" => "新窗口",
        "New tab" => "新标签页",
        "Split pane" => "分屏",
        "Unshare block" => "取消共享块",
        "Available chips" => "可用标签",
        "Restore default" => "恢复默认",
        "Left side" => "左侧",
        "Right side" => "右侧",
        "Save changes" => "保存更改",
        "Context Chip" => "上下文标签",
        "Model Selector" => "模型选择器",
        "Autodetection" => "自动检测",
        "Voice Input" => "语音输入",
        "Attach File" => "附加文件",
        "Context Usage" => "上下文用量",
        "File Explorer" => "文件浏览器",
        "Rich Input" => "富输入",
        "Settings" => "设置",
        "Fast Forward" => "快速前进",
        "Hand off to cloud" => "移交到云端",
        "Your organization disallows AI when the active pane contains content from a remote session" => {
            "当活动面板包含远程会话内容时，您的组织不允许使用 AI"
        }
        "Show model picker in prompt" => "在提示词中显示模型选择器",
        "aws login" => "aws login",
        "default" => "default",
        "command (supports regex)" => "命令（支持正则表达式）",
        "host (supports regex)" => "主机（支持正则表达式）",
        "Open Settings File" | "Open settings file" => "打开设置文件",
        "Copy Link" => "复制链接",
        "Deleting" => "正在删除",
        "Deleting..." => "正在删除...",
        "Getting blocks..." => "正在获取共享块...",
        "Failed to load blocks. Please try again." => "加载共享块失败。请重试。",
        "You don't have any shared blocks yet." => "您还没有任何共享块。",
        "Warp Drive is a workspace in your terminal where you can save Workflows, Notebooks, Prompts, and Environment Variables for personal use or to share with a team." => {
            "Warp Drive 是终端中的工作空间，可保存工作流、笔记本、提示词和环境变量，供个人使用或与团队共享。"
        }
        "e.g. cd my-repo && pip install -r requirements.txt" => {
            "例如 cd my-repo && pip install -r requirements.txt"
        }
        "Share with team" => "与团队共享",

        // ── MCP Servers ────────────────────────────────────────────────────
        "Add MCP servers to extend the Warp Agent's capabilities. MCP servers expose data sources or tools to agents through a standardized interface, essentially acting like plugins. Add a custom server, or use the presets to get started with popular servers. You can also find team servers that have been shared with you here. " => {
            "添加 MCP 服务器以扩展 Warp 智能体的功能。MCP 服务器通过标准化接口向智能体公开数据源或工具，本质上充当插件。您可以添加自定义服务器，或使用预设快速上手热门服务器。您还可以在此找到团队共享给您的服务器。"
        }
        "Once you add a MCP server, it will be shown here." => "添加 MCP 服务器后，将在此处显示。",
        "No search results found" => "未找到搜索结果",
        "Add" => "添加",
        "Available to install" => "可安装",
        "Auto-spawn servers from third-party agents" => "自动启动第三方智能体的服务器",
        "Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually in the \"Detected from\" sections below. " => {
            "自动检测并启动来自全局第三方 AI 智能体配置文件（例如主目录中）的 MCP 服务器。在仓库中检测到的服务器不会自动启动，必须在下方的\u{201c}检测来源\u{201d}部分中单独启用。"
        }
        "See supported providers." => "查看支持的提供商。",
        "Learn more." => "了解更多。",
        "My MCPs" => "我的 MCP",
        "Shared from Warp" => "从 Warp 共享",
        "Shared by Warp and" => "由 Warp 和",
        "Shared by Warp and from other devices" => "由 Warp 共享及来自其他设备",
        "Detected from" => "检测来源",
        "global" => "全局",
        "Detected from config file" => "从配置文件检测",
        "Shared by a team member" => "由团队成员共享",
        "From another device" => "来自其他设备",
        "Shared by" => "共享者",
        "MCP server updated" => "MCP 服务器已更新",

        // ── MCP Edit Page ──────────────────────────────────────────────────
        "Edit Variables" => "编辑变量",
        "Delete MCP" => "删除 MCP",
        "Only team admins and the creator of the MCP server can edit the MCP server." => {
            "只有团队管理员和 MCP 服务器的创建者才能编辑该服务器。"
        }
        "Add New MCP Server" => "添加新 MCP 服务器",
        "Edit MCP Server" => "编辑 MCP 服务器",
        "JSON" => "JSON",
        "This MCP server contains secrets. Visit Settings > Privacy to modify your secret redaction settings." => {
            "此 MCP 服务器包含密钥。请前往设置 > 隐私 修改密钥脱敏设置。"
        }
        "No MCP Server specified." => "未指定 MCP 服务器。",
        "Cannot add multiple MCP servers while editing a single server." => {
            "编辑单个服务器时不能添加多个 MCP 服务器。"
        }

        // ── MCP Destructive Dialog ─────────────────────────────────────────
        "Delete MCP server?" => "删除 MCP 服务器？",
        "This will uninstall and remove this MCP server from all your devices." => {
            "这将从您所有设备上卸载并移除此 MCP 服务器。"
        }
        "Delete shared MCP server?" => "删除共享 MCP 服务器？",
        "This will not only delete this MCP server for yourself, but also uninstall and remove this MCP server from Warp and across all of your teammates' devices." => {
            "这不仅会为您删除此 MCP 服务器，还会从 Warp 及所有队友的设备上卸载并移除此服务器。"
        }
        "Remove shared MCP server from team?" => "从团队中移除共享 MCP 服务器？",
        "This will uninstall and remove this MCP server from Warp and across all of your teammates' devices." => {
            "这将从 Warp 及所有队友的设备上卸载并移除此 MCP 服务器。"
        }

        // ── MCP Server Card ────────────────────────────────────────────────
        "Offline" => "离线",
        "Starting server..." => "正在启动服务器...",
        "Authenticating..." => "正在认证...",
        "Shutting down..." => "正在关闭...",
        "No tools available" => "没有可用工具",
        "Show logs" => "显示日志",
        "Share server" => "共享服务器",
        "Edit" => "编辑",
        "Server update available" => "服务器有可用更新",

        // ── MCP Update Modal ───────────────────────────────────────────────
        "Server" => "服务器",
        "No updates available" => "没有可用更新",
        "another device" => "其他设备",
        "a team member" => "团队成员",
        "This server has" => "此服务器有",
        "updates available, which would you like to proceed with?" => "个更新可用，您想继续哪个？",

        // ── MCP Installation Modal ─────────────────────────────────────────
        "Shared from team" => "从团队共享",
        "Install" => "安装",
        "No MCP server selected" => "未选择 MCP 服务器",

        // ── Execution Profile View ─────────────────────────────────────────
        "Auto" => "自动",
        "Base model:" => "基础模型：",
        "Full terminal use:" => "完整终端使用：",
        "Computer use:" => "计算机使用：",
        "Apply code diffs:" => "应用代码差异：",
        "Read files:" => "读取文件：",
        "Execute commands:" => "执行命令：",
        "Interact with running commands:" => "与运行中的命令交互：",
        "Ask questions:" => "提问：",
        "Call MCP servers:" => "调用 MCP 服务器：",
        "Call web tools:" => "调用 Web 工具：",
        "Auto-sync plans to Warp Drive:" => "自动同步计划到 Warp Drive：",
        "Agent decides" => "智能体决定",
        "Always allow" => "始终允许",
        "Always ask" => "始终询问",
        "Unknown" => "未知",
        "Ask on first write" => "首次写入时询问",
        "Never" => "从不",
        "Never ask" => "从不询问",
        "Ask unless auto-approve" => "除非自动批准否则询问",
        "On" => "开启",
        "Off" => "关闭",
        "Directory allowlist:" => "目录白名单：",
        "Command allowlist:" => "命令白名单：",
        "Command denylist:" => "命令黑名单：",
        "MCP allowlist:" => "MCP 白名单：",
        "MCP denylist:" => "MCP 黑名单：",

        // ── Teams Page ─────────────────────────────────────────────────────
        "Link" => "链接",
        "Email" => "邮箱",
        "Your invite is on the way!" => "您的邀请已发送！",
        "Link copied to clipboard!" => "链接已复制到剪贴板！",
        "Successfully joined team" => "已成功加入团队",
        "Failed to join team" => "加入团队失败",
        "Successfully transferred team ownership" => "已成功转让团队所有权",
        "Failed to transfer team ownership" => "转让团队所有权失败",
        "Successfully updated team member role" => "已成功更新团队成员角色",
        "Failed to update team member role" => "更新团队成员角色失败",
        "Error leaving team" => "离开团队时出错",
        "Successfully left team" => "已成功离开团队",
        "Successfully renamed team" => "已成功重命名团队",
        "Failed to rename team" => "重命名团队失败",
        "Failed to send invite" => "发送邀请失败",
        "Toggled invite links" => "邀请链接已切换",
        "Failed to toggle invite links" => "切换邀请链接失败",
        "Reset invite links" => "邀请链接已重置",
        "Failed to reset invite links" => "重置邀请链接失败",
        "Deleted invite" => "邀请已删除",
        "Failed to delete invite" => "删除邀请失败",
        "Failed to add domain restriction" => "添加域名限制失败",
        "Failed to delete domain restriction" => "删除域名限制失败",
        "Failed to generate upgrade link. Please contact us at feedback@warp.dev" => {
            "生成升级链接失败。请通过 feedback@warp.dev 联系我们"
        }
        "Failed to generate billing link. Please contact us at feedback@warp.dev" => {
            "生成账单链接失败。请通过 feedback@warp.dev 联系我们"
        }
        "Toggled team discoverability" => "团队可发现性已切换",
        "Failed to toggle team discoverability" => "切换团队可发现性失败",
        "Invalid domains: {}" => "无效域名：{}",
        "Domain restrictions added: {}" => "已添加域名限制：{}",
        "Invalid emails: {}" => "无效邮箱：{}",

        // ── Delete Environment Dialog ───────────────────────────────────────
        "Delete environment" => "删除环境",
        "Are you sure you want to remove the {} environment?" => "您确定要删除 {} 环境吗？",
        "Delete environment?" => "删除环境？",

        // ── Directory Color Add Picker ──────────────────────────────────────
        "+ Add directory…" => "+ 添加目录…",
        "Add directory color" => "添加目录颜色",

        // ── Agent Assisted Environment Modal ──────────────────────────────
        "Add repo" => "添加仓库",
        "Create environment" => "创建环境",
        "Selected repos" => "已选仓库",
        "No repos selected yet" => "尚未选择仓库",
        "(unknown)" => "（未知）",
        "Available indexed repos" => "可用的已索引仓库",
        "Loading locally indexed repos…" => "正在加载本地已索引仓库…",
        "No locally indexed repos found yet. Index a repo, then try again." => {
            "尚未找到本地已索引仓库。请先索引一个仓库，然后重试。"
        }
        "Local repo selection is unavailable in this build." => "此版本不支持本地仓库选择。",
        "All locally indexed repos are already selected." => "所有本地已索引仓库均已选中。",
        "Selected folder is not a Git repository" => "所选文件夹不是 Git 仓库",
        "No directory selected" => "未选择目录",
        "Select locally indexed repos to provide context for the environment creation agent." => {
            "选择本地已索引仓库，为环境创建智能体提供上下文。"
        }
        "Select repos to provide context for the environment creation agent." => {
            "选择仓库，为环境创建智能体提供上下文。"
        }
        "Select repos for your environment" => "为您的环境选择仓库",

        // ── MCP Servers Page ─────────────────────────────────────────────
        "Successfully logged out of" => "已成功退出",
        "Successfully logged out of MCP server" => "已成功退出 MCP 服务器",
        "Finish the current MCP install before opening another install link." => {
            "请先完成当前的 MCP 安装，再打开其他安装链接。"
        }
        "Unknown MCP server" => "未知的 MCP 服务器",
        "cannot be installed from this link." => "无法通过此链接安装。",
        "MCP server" => "MCP 服务器",
        "tools available" => "个工具可用",
        "Update from" => "更新来源",
        "Update" => "更新",
        "MCP Server" => "MCP 服务器",

        // ── Environments Page ────────────────────────────────────────────
        "New environment" => "新建环境",
        "Environments" => "环境",
        "Environments define where your ambient agents run. Set one up in minutes via GitHub (recommended), Warp-assisted setup, or manual configuration." => {
            "环境定义了您的智能体运行的位置。通过 GitHub（推荐）、Warp 辅助设置或手动配置，几分钟即可完成设置。"
        }
        "Search environments..." => "搜索环境...",
        "No environments match your search." => "没有匹配的环境。",
        "Shared by Warp and your team" => "由 Warp 和您的团队共享",
        "Personal" => "个人",
        "Loading..." => "加载中...",
        "Retry" => "重试",
        "Authorize" => "授权",
        "Get started" => "开始",
        "Launch agent" => "启动智能体",
        "Quick setup" => "快速设置",
        "Suggested" => "推荐",
        "Select the GitHub repositories you'd like to work with and we'll suggest a base image and config" => {
            "选择您想要使用的 GitHub 仓库，我们将为您推荐基础镜像和配置"
        }
        "Use the agent" => "使用智能体",
        "Choose a locally set up project and we'll help you set up an environment based on it" => {
            "选择一个本地项目，我们将帮助您基于此设置环境"
        }
        "You haven't set up any environments yet." => "您尚未设置任何环境。",
        "Choose how you'd like to set up your environment:" => "选择您希望如何设置环境：",
        "You haven’t set up any environments yet." => "您尚未设置任何环境。",
        "Choose how you’d like to set up your environment:" => "选择您希望如何设置环境：",
        "Enter repos (owner/repo format)" => "输入仓库（owner/repo 格式）",
        "Paste repo URL(s)" => "粘贴仓库 URL",
        "API key deleted" => "API 密钥已删除",
        " to regain access to AI features." => " 以恢复 AI 功能访问。",
        " for more AI credits." => " 以获得更多 AI 额度。",
        " for custom limits and dedicated support." => " 以获得自定义限额和专属支持。",
        "View my runs" => "查看我的运行记录",
        "Share" => "共享",
        "Last used: never" => "最后使用：从未",
        "Successfully updated environment" => "环境更新成功",
        "Successfully created environment" => "环境创建成功",
        "Environment deleted successfully" => "环境删除成功",
        "Successfully shared environment" => "环境共享成功",
        "Failed to share environment with team" => "与团队共享环境失败",
        "Unable to create environment: not logged in." => "无法创建环境：未登录。",
        "Unable to save: environment no longer exists." => "无法保存：环境已不存在。",
        "Unable to share environment: you are not currently on a team." => {
            "无法共享环境：您当前未加入任何团队。"
        }
        "Unable to share environment: environment is not yet synced." => {
            "无法共享环境：环境尚未同步。"
        }
        "Shared by Warp and " => "由 Warp 和 ",
        "Env ID: " => "环境 ID：",
        "Image: " => "镜像：",
        "Repos: " => "仓库：",
        "Setup commands: " => "设置命令：",
        "Last edited: " => "最后编辑：",
        "Last used: " => "最后使用：",

        // Local AI Provider page
        "Enabled" => "启用",
        "Base URL" => "Base URL",
        "Model" => "模型",
        "API Key" => "API 密钥",
        "Enable local AI provider to route agent requests to a custom OpenAI-compatible endpoint." => {
            "启用本地 AI Provider，将智能体请求路由到自定义 OpenAI 兼容端点。"
        }
        "The base URL of the OpenAI-compatible API endpoint. Edit in settings.toml." => {
            "OpenAI 兼容 API 端点的 Base URL。请在 settings.toml 中编辑。"
        }
        "The base URL of the OpenAI-compatible API endpoint." => {
            "OpenAI 兼容 API 端点的 Base URL。"
        }
        "The model name to use for API requests (e.g. gpt-4, mimo-v2.5). Edit in settings.toml." => {
            "用于 API 请求的模型名称（例如 gpt-4、mimo-v2.5）。请在 settings.toml 中编辑。"
        }
        "The model name to use for API requests (e.g. gpt-4, mimo-v2.5)." => {
            "用于 API 请求的模型名称（例如 gpt-4、mimo-v2.5）。"
        }
        "The API key for authentication. Leave empty if no key is required. Edit in settings.toml." => {
            "用于认证的 API 密钥。如不需要密钥请留空。请在 settings.toml 中编辑。"
        }
        "The API key for authentication. Leave empty if no key is required." => {
            "用于认证的 API 密钥。如不需要密钥请留空。"
        }
        "(not set)" => "（未设置）",

        // ── Knowledge / Rules (ai_page.rs FormattedText) ─────────────────────
        "Rules help the Warp Agent follow your conventions, whether for codebases or specific workflows. " => {
            "规则帮助 Warp 智能体遵循您的规范，无论是代码库还是特定工作流。"
        }
        "Learn more" => "了解更多",

        // ── Code Editor and Review (code_page.rs + external_editor.rs) ───────
        "Auto open code review panel" => "自动打开代码审查面板",
        "When this setting is on, the code review panel will open on the first accepted diff of a conversation" => {
            "启用后，代码审查面板将在会话中第一个被接受的差异时自动打开"
        }
        "Show code review button" => "显示代码审查按钮",
        "Show a button in the top right of the window to toggle the code review panel." => {
            "在窗口右上角显示一个按钮，用于切换代码审查面板。"
        }
        "Show diff stats on code review button" => "在代码审查按钮上显示差异统计",
        "Show lines added and removed counts on the code review button." => {
            "在代码审查按钮上显示添加和删除的行数。"
        }
        "Project explorer" => "项目浏览器",
        "Adds an IDE-style project explorer / file tree to the left side tools panel." => {
            "在左侧工具面板添加 IDE 风格的项目浏览器/文件树。"
        }
        "Global file search" => "全局文件搜索",
        "Adds global file search to the left side tools panel." => "在左侧工具面板添加全局文件搜索。",
        "Open Markdown files in Warp's Markdown Viewer by default" => "默认在 Warp 的 Markdown 查看器中打开 Markdown 文件",
        "Index new folder" => "索引新文件夹",
        "Open project rules" => "打开项目规则",
        "Discovered {total_nodes} chunks" => "已发现 {total_nodes} 个代码块",
        "Syncing - {completed_nodes} / {total_nodes}" => "同步中 - {completed_nodes} / {total_nodes}",
        "Syncing..." => "同步中...",
        "Synced" => "已同步",
        "Codebase too large" => "代码库过大",
        "Stale" => "已过时",
        "No index built" => "尚未构建索引",
        "Installed" => "已安装",
        "Installing..." => "安装中...",
        "Checking..." => "检查中...",
        "Available for download" => "可下载",
        "Available" => "可用",
        "Busy" => "忙碌",
        "Stopped" => "已停止",
        "Not running" => "未运行",

        // ── External Editor (external_editor.rs) ─────────────────────────────
        "Group files into single editor pane" => "将文件归入单个编辑器窗格",
        "When this setting is on, any files opened in the same tab will be automatically grouped into a single editor pane." => {
            "启用后，在同一标签页中打开的文件将自动归入单个编辑器窗格。"
        }
        "Split Pane" => "分屏",
        "New Tab" => "新标签页",
        "Default App" => "默认应用",
        "Choose an editor to open file links" => "选择打开文件链接的编辑器",
        "Choose an editor to open files from the code review panel, project explorer, and global search" => {
            "选择从代码审查面板、项目浏览器和全局搜索打开文件时使用的编辑器"
        }
        "Choose a layout to open files in Warp" => "选择在 Warp 中打开文件的布局",

        // ── Teams (teams_page.rs raw strings) ────────────────────────────────
        "update your payment information" => "更新您的付款信息",

        // ── Appearance (appearance_page.rs) ───────────────────────────────────
        "Full-screen Apps" => "全屏应用",
        "Create your own custom theme" => "创建您自己的自定义主题",
        "Current theme" => "当前主题",
        "Automatically switch between light and dark themes when your system does." => {
            "当系统切换时自动在浅色和深色主题之间切换。"
        }
        "Customize your app icon" => "自定义应用图标",
        "Changing the app icon requires the app to be bundled." => "更改应用图标需要将应用打包。",
        "Open new windows with custom size" => "以自定义大小打开新窗口",
        "Columns" => "列",
        "Rows" => "行",
        "Tools panel visibility is consistent across tabs" => "工具面板在各标签页间保持一致的可见性",
        "Input position" => "输入位置",
        "Show Jump to Bottom of Block button" => "显示跳到底部按钮",
        "Show block dividers" => "显示块分隔线",
        "Agent font" => "智能体字体",
        "Terminal font" => "终端字体",
        "Use thin strokes" => "使用细笔画",
        "Enforce minimum contrast" => "强制最低对比度",
        "Show ligatures in terminal" => "在终端中显示连字",
        "Cursor type" => "光标类型",
        "Blinking cursor" => "闪烁光标",
        "Tab close button position" => "标签页关闭按钮位置",
        "Show tab indicators" => "显示标签页指示器",

        // ── Features (features_page.rs + sub-files) ──────────────────────────
        "Default mode for new sessions" => "新会话的默认模式",
        "Agent" => "智能体",
        "Cloud Oz" => "云端 Oz",
        "Tab Config" => "标签配置",
        "Local Docker Sandbox" => "本地 Docker 沙盒",
        "Show sticky command header" => "显示固定命令标题",
        "Show tooltip on click on links" => "点击链接时显示提示",
        "Show warning before quitting/logging out" => "退出或注销前显示警告",
        "Start Warp at login (requires macOS 13+)" => "登录时启动 Warp（需要 macOS 13+）",
        "Start Warp at login" => "登录时启动 Warp",
        "Quit when all windows are closed" => "关闭所有窗口时退出",
        "Show changelog toast after updates" => "更新后显示更新日志提示",
        "Lines scrolled by mouse wheel interval" => "鼠标滚轮每次滚动的行数",
        "Supports floating point values between 1 and 20." => "支持 1 到 20 之间的小数。",
        "Allowed Values: 1-20" => "允许值：1-20",
        "Make Warp the default terminal" => "将 Warp 设为默认终端",
        "Warp is the default terminal" => "Warp 已是默认终端",
        "Maximum rows in a block" => "块中的最大行数",
        "Default shell for new sessions" => "新会话的默认 Shell",
        "Working directory for new sessions" => "新会话的工作目录",
        "Enable reopening of closed sessions" => "允许重新打开已关闭的会话",
        "Grace period (seconds)" => "宽限期（秒）",
        "Advanced" => "高级",
        "Home directory" => "主目录",
        "Previous session's directory" => "上一个会话的目录",
        "Custom directory" => "自定义目录",

        // ── Warpify (warpify_page.rs) ────────────────────────────────────────
        "SSH session detection for Warpification" => "Warpify 的 SSH 会话检测",
        "The tmux ssh wrapper works in many situations where the default one does not, but may require you to hit a button to warpify. Takes effect in new tabs." => {
            "tmux SSH 包装器在许多默认包装器不适用的情况下仍然有效，但可能需要您点击按钮来执行 Warpify。在新标签页中生效。"
        }
        "Controls the installation behavior for Warp's SSH extension when a remote host doesn't have it installed." => {
            "控制当远程主机未安装 Warp SSH 扩展时的安装行为。"
        }
        "Subshells supported: bash, zsh, and fish." => "支持的子 Shell：bash、zsh 和 fish。",
        "Warpify your interactive SSH sessions." => "对您的交互式 SSH 会话执行 Warpify。",
        "Configure whether Warp attempts to \u{201c}Warpify\u{201d} (add support for blocks, input modes, etc) certain shells. " => {
            "配置 Warp 是否尝试对某些 Shell 执行\u{201c}Warpify\u{201d}（添加对块、输入模式等的支持）。"
        }
        "Added commands" => "已添加的命令",
        "Denylisted commands" => "已拒绝的命令",
        "Warpify SSH Sessions" => "Warpify SSH 会话",
        "Install SSH extension" => "安装 SSH 扩展",
        "Use Tmux Warpification" => "使用 Tmux Warpify",
        "Denylisted hosts" => "已拒绝的主机",

        _ => return None,
    })
}
