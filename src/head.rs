use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eframe::{
    egui::{Image, Ui},
    epaint::Rect,
};

use crate::POP_DURATION;

/// The minimum time a speaking frame must be visible.
const MINIMUM_FRAME_TIME: Duration = Duration::from_millis(1000 / 24);

pub struct Head<'a> {
    /// The threshold at which the character is considered to be half speaking, in dBFS.
    half_speak_threshold_dbfs: f32,

    /// The threshold at which the character is considered to be fully speaking, in dBFS.
    full_speak_threshold_dbfs: f32,

    /// The threshold at which the character is considered to be yelling, in dBFS.
    yelling_threshold_dbfs: f32,

    /// Base images to use for the character's head.
    expressions: HashMap<String, HeadExpression<'a>>,

    /// The expression to use when an expression is not found.
    default_expression: HeadExpression<'a>,

    /// The current speaking phase.
    speak_phase: SpeakPhase,

    /// The previous speaking phase.
    last_speak_phase: SpeakPhase,

    /// The time at which the current speaking phase started.
    last_phase_change: Instant,

    /// The time at which speaking last started (phase went from silent to not silent)
    last_speak_start: Instant,
}

impl Head<'_> {
    pub fn paint(&mut self, ui: &mut Ui, rect: Rect, volume: f32, expression_name: &str) {
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
            .expressions
            .get(expression_name)
            .unwrap_or(&self.default_expression)
            .get_image(self.speak_phase);

        head_base.paint_at(ui, rect);
    }

    pub fn get_last_speak_start(&self) -> &Instant {
        &self.last_speak_start
    }
}

impl Default for Head<'_> {
    fn default() -> Self {
        Self {
            half_speak_threshold_dbfs: -35.0,
            full_speak_threshold_dbfs: -23.0,
            yelling_threshold_dbfs: 0.,

            expressions: HashMap::from([
                (
                    "happy".to_string(),
                    HeadExpression {
                        idle: Image::from_bytes(
                            "bytes://head_happy_quiet",
                            include_bytes!("assets/head_happy_quiet.png"),
                        ),
                        half_speak: Some(Image::from_bytes(
                            "bytes://head_happy_halfspeak",
                            include_bytes!("assets/head_happy_halfspeak.png"),
                        )),
                        full_speak: Some(Image::from_bytes(
                            "bytes://head_happy_speak",
                            include_bytes!("assets/head_happy_speak.png"),
                        )),
                        yell: None,
                    },
                ),
                (
                    "frown".to_string(),
                    HeadExpression {
                        idle: Image::from_bytes(
                            "bytes://head_frown_quiet",
                            include_bytes!("assets/head_frown_quiet.png"),
                        ),
                        half_speak: Some(Image::from_bytes(
                            "bytes://head_frown_halfspeak",
                            include_bytes!("assets/head_frown_halfspeak.png"),
                        )),
                        full_speak: Some(Image::from_bytes(
                            "bytes://head_frown_speak",
                            include_bytes!("assets/head_frown_speak.png"),
                        )),
                        yell: Some(Image::from_bytes(
                            "bytes://head_frown_yell",
                            include_bytes!("assets/head_frown_yell.png"),
                        )),
                    },
                ),
                (
                    "wavy".to_string(),
                    HeadExpression {
                        idle: Image::from_bytes(
                            "bytes://head_wavy_quiet",
                            include_bytes!("assets/head_wavy_quiet.png"),
                        ),
                        half_speak: Some(Image::from_bytes(
                            "bytes://head_wavy_halfspeak",
                            include_bytes!("assets/head_wavy_halfspeak.png"),
                        )),
                        full_speak: Some(Image::from_bytes(
                            "bytes://head_wavy_speak",
                            include_bytes!("assets/head_wavy_speak.png"),
                        )),
                        yell: Some(Image::from_bytes(
                            "bytes://head_wavy_yell",
                            include_bytes!("assets/head_wavy_yell.png"),
                        )),
                    },
                ),
            ]),

            default_expression: HeadExpression {
                idle: Image::from_bytes(
                    "bytes://head_default_quiet",
                    include_bytes!("assets/head_happy_quiet.png"),
                ),
                half_speak: Some(Image::from_bytes(
                    "bytes://head_default_halfspeak",
                    include_bytes!("assets/head_happy_halfspeak.png"),
                )),
                full_speak: Some(Image::from_bytes(
                    "bytes://head_default_speak",
                    include_bytes!("assets/head_happy_speak.png"),
                )),
                yell: Some(Image::from_bytes(
                    "bytes://head_default_yell",
                    include_bytes!("assets/head_happy_yell.png"),
                )),
            },

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

pub struct HeadExpression<'a> {
    idle: Image<'a>,
    half_speak: Option<Image<'a>>,
    full_speak: Option<Image<'a>>,
    yell: Option<Image<'a>>,
}

impl HeadExpression<'_> {
    fn get_image(&self, phase: SpeakPhase) -> &Image {
        match phase {
            SpeakPhase::Quiet => &self.idle,
            SpeakPhase::HalfSpeak => self.get_half_speak_image(),
            SpeakPhase::FullSpeak => self.get_full_speak_image(),
            SpeakPhase::Yell => self.get_yell_image(),
        }
    }

    pub fn get_half_speak_image(&self) -> &Image {
        self.half_speak.as_ref().unwrap_or(&self.idle)
    }

    pub fn get_full_speak_image(&self) -> &Image {
        self.full_speak
            .as_ref()
            .unwrap_or(self.get_half_speak_image())
    }

    pub fn get_yell_image(&self) -> &Image {
        self.yell.as_ref().unwrap_or(self.get_full_speak_image())
    }
}
