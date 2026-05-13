use super::settings_page::{
    render_body_item, settings_display_text, LocalOnlyIconState, MatchData, PageType,
    SettingsPageMeta, SettingsWidget, ToggleState,
};
use super::SettingsSection;
use crate::appearance::Appearance;
use crate::editor::{EditorView, SingleLineEditorOptions, TextColors, TextOptions};
use crate::settings::AISettings;
use settings::Setting as _;
use warp_core::report_if_error;
use warpui::{
    elements::{
        Align, Container, CrossAxisAlignment, Element, Flex, MainAxisSize, MouseStateHandle,
        ParentElement, Shrinkable,
    },
    ui_components::{
        button::ButtonVariant,
        components::{Coords, UiComponent, UiComponentStyles},
    },
    AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

pub struct LocalAIProviderPageView {
    page: PageType<Self>,
    base_url_input: ViewHandle<EditorView>,
    model_input: ViewHandle<EditorView>,
    api_key_input: ViewHandle<EditorView>,
}

pub enum LocalAIProviderPageEvent {}

#[derive(Debug, Clone)]
pub enum LocalAIProviderPageAction {
    SaveSettings,
}

impl LocalAIProviderPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let settings = AISettings::as_ref(ctx);
        let base_url = settings.local_agent_provider_base_url.value().clone();
        let model = settings.local_agent_provider_model.value().clone();
        let api_key = settings.local_agent_provider_api_key.value().clone();

        let base_url_input = Self::create_editor("https://api.example.com/v1", ctx);
        let model_input = Self::create_editor("mimo-v2.5", ctx);
        let api_key_input = Self::create_editor("sk-...", ctx);

        Self::set_input_text(&base_url_input, &base_url, ctx);
        Self::set_input_text(&model_input, &model, ctx);
        Self::set_input_text(&api_key_input, &api_key, ctx);

        Self {
            page: PageType::new_monolith(ProviderSettingsWidget::default(), None, false),
            base_url_input,
            model_input,
            api_key_input,
        }
    }

    fn create_editor(placeholder: &str, ctx: &mut ViewContext<Self>) -> ViewHandle<EditorView> {
        ctx.add_typed_action_view(|ctx| {
            let appearance = Appearance::as_ref(ctx);
            let theme = appearance.theme();
            let input_background = theme.surface_2();
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_size_override: Some(appearance.ui_font_size()),
                    font_family_override: Some(appearance.monospace_font_family()),
                    text_colors_override: Some(TextColors {
                        default_color: theme.main_text_color(input_background),
                        disabled_color: theme.disabled_text_color(input_background),
                        hint_color: theme.hint_text_color(input_background),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text(&settings_display_text(placeholder), ctx);
            editor
        })
    }

    fn set_input_text(
        input_handle: &ViewHandle<EditorView>,
        value: &str,
        ctx: &mut ViewContext<Self>,
    ) {
        input_handle.update(ctx, |editor, ctx| {
            editor.set_buffer_text(value, ctx);
        });
    }
}

impl View for LocalAIProviderPageView {
    fn ui_name() -> &'static str {
        "LocalAIProviderPageView"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

impl Entity for LocalAIProviderPageView {
    type Event = LocalAIProviderPageEvent;
}

impl TypedActionView for LocalAIProviderPageView {
    type Action = LocalAIProviderPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            LocalAIProviderPageAction::SaveSettings => {
                let base_url = self.base_url_input.as_ref(ctx).buffer_text(ctx);
                let model = self.model_input.as_ref(ctx).buffer_text(ctx);
                let api_key = self.api_key_input.as_ref(ctx).buffer_text(ctx);

                AISettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.local_agent_provider_enabled.set_value(true, ctx));
                    report_if_error!(settings
                        .local_agent_provider_base_url
                        .set_value(base_url.trim().to_string(), ctx));
                    report_if_error!(settings
                        .local_agent_provider_model
                        .set_value(model.trim().to_string(), ctx));
                    report_if_error!(settings
                        .local_agent_provider_api_key
                        .set_value(api_key.trim().to_string(), ctx));
                });
                ctx.notify();
            }
        }
    }
}

impl SettingsPageMeta for LocalAIProviderPageView {
    fn section() -> SettingsSection {
        SettingsSection::LocalAIProvider
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

#[derive(Default)]
struct ProviderSettingsWidget {
    save_button_state: MouseStateHandle,
}

impl ProviderSettingsWidget {
    fn render_input_item(
        &self,
        label: &str,
        description: &str,
        input_handle: &ViewHandle<EditorView>,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        render_body_item::<LocalAIProviderPageAction>(
            settings_display_text(label).to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            Shrinkable::new(
                1.,
                appearance
                    .ui_builder()
                    .text_input(input_handle.clone())
                    .with_style(UiComponentStyles {
                        width: Some(360.),
                        padding: Some(Coords {
                            top: 4.,
                            bottom: 4.,
                            left: 6.,
                            right: 6.,
                        }),
                        background: Some(appearance.theme().surface_2().into()),
                        ..Default::default()
                    })
                    .build()
                    .finish(),
            )
            .finish(),
            Some(settings_display_text(description).to_string()),
        )
    }
}

impl SettingsWidget for ProviderSettingsWidget {
    type View = LocalAIProviderPageView;

    fn search_terms(&self) -> &str {
        "local ai provider base url model api key"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        let base_url_item = self.render_input_item(
            "Base URL",
            "The base URL of the OpenAI-compatible API endpoint.",
            &view.base_url_input,
            appearance,
        );
        let model_item = self.render_input_item(
            "Model",
            "The model name to use for API requests (e.g. gpt-4, mimo-v2.5).",
            &view.model_input,
            appearance,
        );
        let api_key_item = self.render_input_item(
            "API Key",
            "The API key for authentication. Leave empty if no key is required.",
            &view.api_key_input,
            appearance,
        );

        let save_button = Container::new(
            Align::new(
                appearance
                    .ui_builder()
                    .button(ButtonVariant::Accent, self.save_button_state.clone())
                    .with_text_label(settings_display_text("Save"))
                    .build()
                    .on_click(|ctx, _, _| {
                        ctx.dispatch_typed_action(LocalAIProviderPageAction::SaveSettings);
                    })
                    .finish(),
            )
            .right()
            .finish(),
        )
        .with_margin_top(12.)
        .finish();

        Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(base_url_item)
            .with_child(model_item)
            .with_child(api_key_item)
            .with_child(save_button)
            .with_main_axis_size(MainAxisSize::Min)
            .finish()
    }
}
