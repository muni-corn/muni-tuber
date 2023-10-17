use std::time::{Duration, Instant};

use eframe::{
    egui::{Context, Image, Ui},
    epaint::Rect,
};
use egui_extras::RetainedImage;

/// The minimum time a speaking frame must be visible.
const MINIMUM_FRAME_TIME: Duration = Duration::from_millis(100);

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

    /// The current speaking phase.
    speak_phase: SpeakPhase,

    /// The time at which the current speaking phase started.
    last_phase_change: Instant,
}

impl Head {
    pub fn paint(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect, volume: f32) {
        // determine head_base to use
        if self.last_phase_change.elapsed() > MINIMUM_FRAME_TIME {
            let last_phase = self.speak_phase;
            self.speak_phase = if volume > self.full_speak_threshold_dbfs {
                SpeakPhase::FullSpeak
            } else if volume > self.half_speak_threshold_dbfs {
                SpeakPhase::HalfSpeak
            } else {
                SpeakPhase::Quiet
            };

            if last_phase != self.speak_phase {
                self.last_phase_change = Instant::now();
            }
        }

        let head_base = match self.speak_phase {
            SpeakPhase::Quiet => &self.quiet,
            SpeakPhase::HalfSpeak => &self.half_speak,
            SpeakPhase::FullSpeak => &self.full_speak,
        };

        Image::new(head_base.texture_id(ctx), rect.size()).paint_at(ui, rect);
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

            speak_phase: SpeakPhase::Quiet,
            last_phase_change: Instant::now(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SpeakPhase {
    Quiet,
    HalfSpeak,
    FullSpeak,
}

enum HeadExpression {
    Happy,
    Frown,
}
