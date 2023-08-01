mod audio;

use cpal::Stream;
use eframe::{
    egui::{self, CentralPanel, Context, Vec2},
    Frame,
};
use egui_extras::RetainedImage;
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
    start: Instant,
    audio_state: audio::AudioState,
    _audio_stream: Stream,

    quiet: RetainedImage,
    half_speak: RetainedImage,
    full_speak: RetainedImage,
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
        }
    }
}

const SIZE: Vec2 = Vec2::new(300.0, 300.0);
const HALF_SPEAK_THRESHOLD_DBFS: f32 = -30.0;
const FULL_SPEAK_THRESHOLD_DBFS: f32 = -20.0;

impl eframe::App for MuniTuberApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                let breath_value = self.start.elapsed().as_secs_f32().sin() / 75.0;
                let breath_scale_x = 1.0 - breath_value;
                let breath_scale_y = 1.0 + breath_value;

                let img = {
                    let volume = *self.audio_state.volume.lock().unwrap();
                    if volume > FULL_SPEAK_THRESHOLD_DBFS {
                        &self.full_speak
                    } else if volume > HALF_SPEAK_THRESHOLD_DBFS {
                        &self.half_speak
                    } else {
                        &self.quiet
                    }
                };
                img.show_size(ui, SIZE * Vec2::new(breath_scale_x, breath_scale_y))
            });
        });
        ctx.request_repaint();
    }
}
