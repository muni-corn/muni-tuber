mod audio;
mod eyes;

use cpal::Stream;
use eframe::{
    egui::{self, CentralPanel, Context, Ui, Vec2},
    Frame,
};
use egui_extras::RetainedImage;
use eyes::Eyes;
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

    /// The head base image to use when the character is quiet.
    quiet: RetainedImage,

    /// The head base image to use when the character is half speaking.
    half_speak: RetainedImage,

    /// The head base image to use when the character is fully speaking.
    full_speak: RetainedImage,

    /// The eyes state of the character.
    eyes: Eyes,

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

            quiet: RetainedImage::from_image_bytes("quiet", include_bytes!("assets/quiet.png"))
                .unwrap(),
            half_speak: RetainedImage::from_image_bytes(
                "half_speak",
                include_bytes!("assets/half_speak.png"),
            )
            .unwrap(),
            full_speak: RetainedImage::from_image_bytes(
                "full_speak",
                include_bytes!("assets/full_speak.png"),
            )
            .unwrap(),

            eyes: Default::default(),
        }
    }
}

/// The threshold at which the character is considered to be half speaking, in dBFS.
const HALF_SPEAK_THRESHOLD_DBFS: f32 = -30.0;

/// The threshold at which the character is considered to be fully speaking, in dBFS.
const FULL_SPEAK_THRESHOLD_DBFS: f32 = -20.0;

impl MuniTuberApp {
    fn paint(&mut self, ctx: &Context, ui: &mut Ui) {
        let breath_value = self.start.elapsed().as_secs_f32().sin() / 75.0;
        let breath_scale_x = 1.0 - breath_value;
        let breath_scale_y = 1.0 + breath_value;

        // determine head_base to use
        let head_base = {
            let volume = *self.audio_state.volume.lock().unwrap();
            if volume > FULL_SPEAK_THRESHOLD_DBFS {
                &self.full_speak
            } else if volume > HALF_SPEAK_THRESHOLD_DBFS {
                &self.half_speak
            } else {
                &self.quiet
            }
        };


        // draw head and eyes
        let image_to_ui_height_ratio = ui.max_rect().height() / head_base.size_vec2().y;
        let head_base_response = head_base.show_size(
            ui,
            image_to_ui_height_ratio
                * head_base.size_vec2()
                * Vec2::new(breath_scale_x, breath_scale_y),
        );
        self.eyes.paint(ctx, ui, head_base_response.rect);
    }
}

impl eframe::App for MuniTuberApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                self.paint(ctx, ui);
            });
        });
        ctx.request_repaint();
    }
}
