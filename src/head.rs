use eframe::{egui::Image, egui::{Context, Ui}, epaint::Rect};
use egui_extras::RetainedImage;

pub struct Head {
    /// The threshold at which the character is considered to be fully speaking, in dBFS.
    full_speak_threshold_dbfs: f32,

    /// The threshold at which the character is considered to be half speaking, in dBFS.
    half_speak_threshold_dbfs: f32,

    /// The head base image to use when the character is quiet.
    quiet: RetainedImage,

    /// The head base image to use when the character is half speaking.
    half_speak: RetainedImage,

    /// The head base image to use when the character is fully speaking.
    full_speak: RetainedImage,
}

impl Head {
    pub fn paint(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect, volume: f32) {
        // determine head_base to use
        let head_base = {
            if volume > self.full_speak_threshold_dbfs {
                &self.full_speak
            } else if volume > self.half_speak_threshold_dbfs {
                &self.half_speak
            } else {
                &self.quiet
            }
        };

        Image::new(head_base.texture_id(ctx), rect.size())
            .paint_at(ui, rect);
    }
}

impl Default for Head {
    fn default() -> Self {
        Self {
            full_speak_threshold_dbfs: -20.0,
            half_speak_threshold_dbfs: -30.0,

            quiet: RetainedImage::from_image_bytes(
                "quiet",
                include_bytes!("assets/head_happy_quiet.png"),
            )
            .unwrap(),
            half_speak: RetainedImage::from_image_bytes(
                "half_speak",
                include_bytes!("assets/head_happy_halfspeak.png"),
            )
            .unwrap(),
            full_speak: RetainedImage::from_image_bytes(
                "full_speak",
                include_bytes!("assets/head_happy_speak.png"),
            )
            .unwrap(),
        }
    }
}

enum SpeakPhase {
    Quiet,
    HalfSpeak,
    FullSpeak,
}

enum HeadExpression {
    Happy,
    Frown,
}
