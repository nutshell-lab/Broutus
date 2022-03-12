use bevy_inspector_egui::egui::*;

enum ProgressBarText {
    Custom(WidgetText),
    Percentage,
}

pub struct LightProgressBar {
    progress: f32,
    text: Option<ProgressBarText>,
    fg_color: Option<Color32>,
    bg_color: Option<Color32>,
    text_color: Option<Color32>,
    desired_width: Option<f32>,
    desired_height: Option<f32>,
}

impl LightProgressBar {
    /// Progress in the `[0, 1]` range, where `1` means "completed".
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            text: None,
            fg_color: None,
            bg_color: None,
            text_color: None,
            desired_width: None,
            desired_height: None,
        }
    }

    /// A custom text to display on the progress bar.
    pub fn custom_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.text = Some(ProgressBarText::Custom(text.into()));
        self
    }

    /// A custom text to display on the progress bar.
    pub fn percentage_text(mut self) -> Self {
        self.text = Some(ProgressBarText::Percentage);
        self
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn fg_color(mut self, fg_color: impl Into<Color32>) -> Self {
        self.fg_color = Some(fg_color.into());
        self
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn bg_color(mut self, bg_color: impl Into<Color32>) -> Self {
        self.bg_color = Some(bg_color.into());
        self
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn text_color(mut self, text_color: impl Into<Color32>) -> Self {
        self.text_color = Some(text_color.into());
        self
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = Some(desired_width);
        self
    }

    /// The desired height of the bar. Will use 4.0px if not set.
    pub fn desired_height(mut self, desired_height: f32) -> Self {
        self.desired_height = Some(desired_height);
        self
    }
}

impl Widget for LightProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let LightProgressBar {
            progress,
            text,
            fg_color,
            bg_color,
            text_color,
            desired_width,
            desired_height,
        } = self;

        let fg_color = fg_color.unwrap_or(Color32::DARK_BLUE);
        let bg_color = bg_color.unwrap_or(Color32::LIGHT_GRAY);
        let text_color = text_color.unwrap_or(Color32::DARK_GRAY);

        let desired_width =
            desired_width.unwrap_or_else(|| ui.available_size_before_wrap().x.at_least(96.0));

        let desired_height = desired_height.unwrap_or(4.0);

        let (outer_rect, response) = ui.allocate_exact_size(
            vec2(desired_width, desired_height),
            Sense::focusable_noninteractive(),
        );

        if ui.is_rect_visible(response.rect) {
            ui.painter().rect(outer_rect, 0.0, bg_color, Stroke::none());

            if progress > 0.0 {
                let inner_rect = Rect::from_min_size(
                    outer_rect.min,
                    vec2(
                        (outer_rect.width() * progress).at_least(outer_rect.height()),
                        outer_rect.height(),
                    ),
                );

                ui.painter().rect(inner_rect, 0.0, fg_color, Stroke::none());
            }

            if let Some(text) = text {
                let widget = match text {
                    ProgressBarText::Custom(widget) => widget,
                    ProgressBarText::Percentage => {
                        format!("{}%", (progress * 100.0) as usize).into()
                    }
                };

                let galley = widget.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Button);
                let text_pos = outer_rect.left_center() - Vec2::new(0.0, galley.size().y / 2.0)
                    + vec2(ui.spacing().item_spacing.x, 0.0);
                galley.paint_with_fallback_color(
                    &ui.painter().sub_region(outer_rect),
                    text_pos,
                    text_color,
                );
            }
        }

        response
    }
}
