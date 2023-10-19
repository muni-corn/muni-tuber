mod audio;
mod eyes;
mod head;

use cpal::Stream;
use eframe::{
    egui::{self, CentralPanel, Context, Ui, Vec2},
    Frame,
};
use egui_extras::RetainedImage;
use eyes::Eyes;
use head::Head;
use std::time::Instant;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "muni-tuber",
        options,
        Box::new(|_cc| Box::<MuniTuberApp>::default()),
    )
}

struct MuniTuberApp {
    /// The time at which the app started.
    start: Instant,

    /// The state of the audio input volume.
    audio_state: audio::AudioState,

    /// The image of the body to use.
    body: RetainedImage,

    /// The head of the character
    head: Head,

    /// The eyes state of the character.
    eyes: Eyes,

    /// The expression of the character.
    expression: ExpressionState,

    /// The audio input stream, stored here so that it isn't dropped.
    _audio_stream: Stream,
}

impl Default for MuniTuberApp {
    fn default() -> Self {
        let (audio_state, _audio_stream) = audio::start_default_stream();
        Self {
            start: Instant::now(),
            audio_state,
            _audio_stream,

            body: RetainedImage::from_image_bytes("body", include_bytes!("assets/body.png"))
                .unwrap(),

            head: Default::default(),
            eyes: Default::default(),
            expression: Default::default(),
        }
    }
}

impl MuniTuberApp {
    fn paint(&mut self, ctx: &Context, ui: &mut Ui) {
        let breath_value = (self.start.elapsed().as_secs_f32() * 1.5).sin() / 200.0;
        let breath_scale_x = 1.0 - breath_value;
        let breath_scale_y = 1.0 + breath_value;

        // draw body
        let image_to_ui_height_ratio = ui.max_rect().height() / self.body.size_vec2().y;
        let show_body_response = self.body.show_size(
            ui,
            image_to_ui_height_ratio
                * self.body.size_vec2()
                * Vec2::new(breath_scale_x, breath_scale_y),
        );

        // draw head and eyes
        let volume = *self.audio_state.volume.lock().unwrap();
        self.head.paint(ctx, ui, show_body_response.rect, volume, self.expression.head);
        self.eyes.paint(ctx, ui, show_body_response.rect, self.expression.eyes);
    }
}

impl eframe::App for MuniTuberApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                self.paint(ctx, ui);
            });
        });
        ctx.request_repaint();
    }
}

struct ExpressionState {
    /// The current expression of the character.
    eyes: eyes::EyesExpression,
    head: head::HeadExpression,
}

impl Default for ExpressionState {
    fn default() -> Self {
        Self {
            eyes: eyes::EyesExpression::Normal,
            head: head::HeadExpression::Happy,
        }
    }
}
