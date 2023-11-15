use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eframe::{
    egui::{Context, Image, Ui},
    epaint::Rect,
};
use egui_extras::RetainedImage;

use crate::POP_DURATION;

/// The minimum time a speaking frame must be visible.
const MINIMUM_FRAME_TIME: Duration = Duration::from_millis(1000 / 24);

pub struct Head {
    /// The threshold at which the character is considered to be half speaking, in dBFS.
    half_speak_threshold_dbfs: f32,

    /// The threshold at which the character is considered to be fully speaking, in dBFS.
    full_speak_threshold_dbfs: f32,

    /// The threshold at which the character is considered to be yelling, in dBFS.
    yelling_threshold_dbfs: f32,

    /// Base images to use for the character's head.
    head_bases: HashMap<(HeadExpression, SpeakPhase), RetainedImage>,

    /// The default head base to use.
    default_head_base: RetainedImage,

    /// The current speaking phase.
    speak_phase: SpeakPhase,

    /// The previous speaking phase.
    last_speak_phase: SpeakPhase,

    /// The time at which the current speaking phase started.
    last_phase_change: Instant,

    /// The time at which speaking last started (phase went from silent to not silent)
    last_speak_start: Instant,
}

impl Head {
    pub fn paint(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        rect: Rect,
        volume: f32,
        expression: HeadExpression,
    ) {
        // determine head_base to use
        if self.last_phase_change.elapsed() > MINIMUM_FRAME_TIME {
            self.last_speak_phase = self.speak_phase;
            self.speak_phase = if volume > self.yelling_threshold_dbfs {
                SpeakPhase::Yell
            } else if volume > self.full_speak_threshold_dbfs {
                SpeakPhase::FullSpeak
            } else if volume > self.half_speak_threshold_dbfs {
                SpeakPhase::HalfSpeak
            } else {
                SpeakPhase::Quiet
            };

            if self.last_speak_phase != self.speak_phase {
                self.last_phase_change = Instant::now();
            }
        }

        if self.last_speak_phase == SpeakPhase::Quiet
            && self.speak_phase != SpeakPhase::Quiet
            && self.last_speak_start.elapsed().as_secs_f32() > POP_DURATION
        {
            self.last_speak_start = Instant::now();
        }

        let head_base = self
            .head_bases
            .get(&(expression, self.speak_phase))
            .unwrap_or(&self.default_head_base);
        Image::new(head_base.texture_id(ctx), rect.size()).paint_at(ui, rect);
    }

    pub fn get_last_speak_start(&self) -> &Instant {
        &self.last_speak_start
    }
}

impl Default for Head {
    fn default() -> Self {
        Self {
            half_speak_threshold_dbfs: -30.0,
            full_speak_threshold_dbfs: -20.0,
            yelling_threshold_dbfs: 5.0,

            head_bases: HashMap::from([
                // happy faces
                (
                    (HeadExpression::Happy, SpeakPhase::Quiet),
                    RetainedImage::from_image_bytes(
                        "head_happy_quiet",
                        include_bytes!("assets/head_happy_quiet.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Happy, SpeakPhase::HalfSpeak),
                    RetainedImage::from_image_bytes(
                        "head_happy_halfspeak",
                        include_bytes!("assets/head_happy_halfspeak.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Happy, SpeakPhase::FullSpeak),
                    RetainedImage::from_image_bytes(
                        "head_happy_speak",
                        include_bytes!("assets/head_happy_speak.png"),
                    )
                    .unwrap(),
                ),
                // frowny faces
                (
                    (HeadExpression::Frown, SpeakPhase::Quiet),
                    RetainedImage::from_image_bytes(
                        "head_frown_quiet",
                        include_bytes!("assets/head_frown_quiet.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Frown, SpeakPhase::HalfSpeak),
                    RetainedImage::from_image_bytes(
                        "head_frown_halfspeak",
                        include_bytes!("assets/head_frown_halfspeak.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Frown, SpeakPhase::FullSpeak),
                    RetainedImage::from_image_bytes(
                        "head_frown_speak",
                        include_bytes!("assets/head_frown_speak.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Frown, SpeakPhase::Yell),
                    RetainedImage::from_image_bytes(
                        "head_frown_yell",
                        include_bytes!("assets/head_frown_yell.png"),
                    )
                    .unwrap(),
                ),

                // wavy faces
                (
                    (HeadExpression::Wavy, SpeakPhase::Quiet),
                    RetainedImage::from_image_bytes(
                        "head_wavy_quiet",
                        include_bytes!("assets/head_wavy_quiet.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Wavy, SpeakPhase::HalfSpeak),
                    RetainedImage::from_image_bytes(
                        "head_wavy_halfspeak",
                        include_bytes!("assets/head_wavy_halfspeak.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Wavy, SpeakPhase::FullSpeak),
                    RetainedImage::from_image_bytes(
                        "head_wavy_speak",
                        include_bytes!("assets/head_wavy_speak.png"),
                    )
                    .unwrap(),
                ),
                (
                    (HeadExpression::Wavy, SpeakPhase::Yell),
                    RetainedImage::from_image_bytes(
                        "head_wavy_yell",
                        include_bytes!("assets/head_wavy_yell.png"),
                    )
                    .unwrap(),
                ),
            ]),

            // RetainedImage does not implement Clone >:c
            default_head_base: RetainedImage::from_image_bytes(
                "head_default",
                include_bytes!("assets/head_happy_quiet.png"),
            )
            .unwrap(),

            speak_phase: SpeakPhase::Quiet,
            last_speak_phase: SpeakPhase::Quiet,
            last_phase_change: Instant::now(),
            last_speak_start: Instant::now(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SpeakPhase {
    Quiet,
    HalfSpeak,
    FullSpeak,
    Yell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeadExpression {
    Happy,
    Frown,
    Wavy,
}
