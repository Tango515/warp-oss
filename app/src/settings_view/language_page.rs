#[cfg(target_os = "macos")]
use crate::app_menus::rebuild_menu_bar;
use crate::appearance::Appearance;
use crate::i18n::{current_locale, persist_locale, set_current_locale, tr, Locale, MessageId};
use crate::settings_view::settings_page::{
    render_body_item, MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle,
    SettingsWidget, ToggleState,
};
use crate::settings_view::SettingsSection;
use warpui::elements::{
    Align, Container, CrossAxisAlignment, Element, Flex, MainAxisAlignment, MainAxisSize,
    MouseStateHandle, ParentElement, Radius, Shrinkable, Text,
};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

fn rebuild_platform_menu_bar(ctx: &mut AppContext) {
    #[cfg(target_os = "macos")]
    rebuild_menu_bar(ctx);

    #[cfg(not(target_os = "macos"))]
    let _ = ctx;
}

pub struct LanguagePageView {
    page: PageType<Self>,
}

#[derive(Clone, Copy)]
pub enum LanguagePageViewEvent {
    LocaleChanged,
}

impl LanguagePageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![Box::<LanguageSelectorWidget>::default()], None),
        }
    }
}

impl View for LanguagePageView {
    fn ui_name() -> &'static str {
        "LanguagePageView"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

impl Entity for LanguagePageView {
    type Event = LanguagePageViewEvent;
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguagePageAction {
    SetLocale(Locale),
}

impl TypedActionView for LanguagePageView {
    type Action = LanguagePageAction;

    fn handle_action(&mut self, action: &LanguagePageAction, ctx: &mut ViewContext<Self>) {
        match action {
            LanguagePageAction::SetLocale(locale) => {
                if current_locale() != *locale {
                    set_current_locale(*locale);
                    persist_locale(ctx, *locale);
                    rebuild_platform_menu_bar(ctx);
                    ctx.emit(LanguagePageViewEvent::LocaleChanged);
                }
                ctx.notify();
            }
        }
    }
}

impl SettingsPageMeta for LanguagePageView {
    fn section() -> SettingsSection {
        SettingsSection::Language
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<LanguagePageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<LanguagePageView>) -> Self {
        SettingsPageViewHandle::Language(view_handle)
    }
}

#[derive(Default)]
struct LanguageSelectorWidget {
    english_button: MouseStateHandle,
    simplified_chinese_button: MouseStateHandle,
}

impl LanguageSelectorWidget {
    fn render_locale_button(
        &self,
        locale: Locale,
        label: String,
        mouse_state: MouseStateHandle,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let selected = current_locale() == locale;
        let button = appearance
            .ui_builder()
            .button(
                if selected {
                    ButtonVariant::Accent
                } else {
                    ButtonVariant::Text
                },
                mouse_state,
            )
            .with_text_label(label)
            .with_style(UiComponentStyles {
                border_radius: Some(warpui::elements::CornerRadius::with_all(Radius::Pixels(4.))),
                padding: Some(Coords {
                    top: 8.,
                    bottom: 8.,
                    left: 16.,
                    right: 16.,
                }),
                ..Default::default()
            })
            .build()
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(LanguagePageAction::SetLocale(locale));
            })
            .finish();

        Container::new(button).with_margin_left(8.).finish()
    }
}

impl SettingsWidget for LanguageSelectorWidget {
    type View = LanguagePageView;

    fn search_terms(&self) -> &str {
        "language locale english simplified chinese"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        let locale = current_locale();
        let selector = Flex::row()
            .with_main_axis_size(MainAxisSize::Min)
            .with_main_axis_alignment(MainAxisAlignment::End)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(self.render_locale_button(
                Locale::EnUs,
                tr(locale, MessageId::LanguagePageOptionEnglish).into_owned(),
                self.english_button.clone(),
                appearance,
            ))
            .with_child(self.render_locale_button(
                Locale::ZhCn,
                tr(locale, MessageId::LanguagePageOptionSimplifiedChinese).into_owned(),
                self.simplified_chinese_button.clone(),
                appearance,
            ))
            .finish();

        let note = Text::new(
            tr(locale, MessageId::LanguagePageRestartNote).into_owned(),
            appearance.ui_font_family(),
            12.,
        )
        .with_color(
            appearance
                .theme()
                .sub_text_color(appearance.theme().surface_2())
                .into(),
        )
        .finish();

        let child = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::End)
            .with_child(selector)
            .with_child(Container::new(note).with_margin_top(8.).finish())
            .finish();

        render_body_item::<LanguagePageAction>(
            tr(locale, MessageId::LanguagePageSelectorLabel).into_owned(),
            None,
            super::settings_page::LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            Shrinkable::new(1., Align::new(child).right().finish()).finish(),
            Some(tr(locale, MessageId::LanguagePageDescription).into_owned()),
        )
    }
}
