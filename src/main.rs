mod audio;
mod eyes;
mod head;
mod keys;

use cpal::Stream;
use eframe::{
    egui::{self, CentralPanel, Context, Key, Ui, Vec2},
    epaint::Color32,
    Frame,
};
use egui_extras::RetainedImage;
use eyes::Eyes;
use head::Head;
use std::{collections::HashMap, time::Instant};

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

    /// The hotkey manager for the character's expressions.
    hotkey_manager: keys::ExpressionHotkeyManager,

    /// The audio input stream, stored here so that it isn't dropped.
    _audio_stream: Stream,
}

impl Default for MuniTuberApp {
    fn default() -> Self {
        let (audio_state, _audio_stream) = audio::start_default_stream();

        let hotkey_manager = keys::ExpressionHotkeyManager {
            force_blink_key: Key::F12,
            expression_switches: HashMap::from([
                (
                    Key::F1,
                    ExpressionChange {
                        eyes: Some("normal".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
                (
                    Key::F2,
                    ExpressionChange {
                        eyes: Some("angry".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
                (
                    Key::F3,
                    ExpressionChange {
                        eyes: Some("sad".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
                (
                    Key::F4,
                    ExpressionChange {
                        eyes: Some("dreamy".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
                (
                    Key::F5,
                    ExpressionChange {
                        eyes: Some("normal".to_string()),
                        head: Some("frown".to_string()),
                    },
                ),
                (
                    Key::F6,
                    ExpressionChange {
                        eyes: Some("angry".to_string()),
                        head: Some("frown".to_string()),
                    },
                ),
                (
                    Key::F7,
                    ExpressionChange {
                        eyes: Some("sad".to_string()),
                        head: Some("frown".to_string()),
                    },
                ),
                (
                    Key::F8,
                    ExpressionChange {
                        eyes: Some("dreamy".to_string()),
                        head: Some("frown".to_string()),
                    },
                ),
                (
                    Key::F9,
                    ExpressionChange {
                        eyes: Some("sad".to_string()),
                        head: Some("wavy".to_string()),
                    },
                ),
                (
                    Key::F10,
                    ExpressionChange {
                        eyes: Some("wide".to_string()),
                        head: Some("wavy".to_string()),
                    },
                ),
                (
                    Key::F11,
                    ExpressionChange {
                        eyes: Some("happy".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
                (
                    Key::F12,
                    ExpressionChange {
                        eyes: Some("tight".to_string()),
                        head: Some("happy".to_string()),
                    },
                ),
            ]),
            expression_holds: HashMap::new(),
        };

        Self {
            start: Instant::now(),
            audio_state,
            _audio_stream,

            body: RetainedImage::from_image_bytes("body", include_bytes!("assets/body.png"))
                .unwrap(),

            head: Default::default(),
            eyes: Default::default(),
            expression: Default::default(),
            hotkey_manager,
        }
    }
}

/// The duration of the "pop" when the character begins speaking.
const POP_DURATION: f32 = 0.25;

/// The influence of the pop animation on the character.
const POP_AMOUNT: f32 = 2.0;

impl MuniTuberApp {
    fn paint(&mut self, ctx: &Context, ui: &mut Ui) {
        let pop_value = {
            // quadratic function
            let x = self.head.get_last_speak_start().elapsed().as_secs_f32();
            let a = -4.0 / POP_DURATION.powi(2);
            let b = -1.0 * a * POP_DURATION;

            a * x.powi(2) + b * x
        }
        .max(0.0);

        let breath_value =
            (self.start.elapsed().as_secs_f32() * 1.5).sin() + pop_value * POP_AMOUNT;
        let breath_scale_x = 1.0 - breath_value / 200.0;
        let breath_scale_y = 1.0 + breath_value / 200.0;

        // draw body
        let image_to_ui_height_ratio = ui.max_rect().height() / self.body.size_vec2().y;
        let show_body_response = self.body.show_size(
            ui,
            image_to_ui_height_ratio
                * self.body.size_vec2()
                * Vec2::new(breath_scale_x, breath_scale_y),
        );

        // get some variables
        let should_force_blink = self.hotkey_manager.should_force_blink(ctx);
        if let Some(new_expression) = self.hotkey_manager.get_expression(ctx) {
            self.expression.apply(new_expression)
        }
        let temporary_expression = self.hotkey_manager.get_temporary_expression(ctx);
        let head_to_use = temporary_expression
            .and_then(|e| e.head.as_ref())
            .unwrap_or(&self.expression.head);
        let eyes_to_use = temporary_expression
            .and_then(|e| e.eyes.as_ref())
            .unwrap_or(&self.expression.eyes);

        // draw head and eyes
        let volume = *self.audio_state.volume.lock().unwrap();
        self.head
            .paint(ctx, ui, show_body_response.rect, volume, head_to_use);
        self.eyes.paint(
            ctx,
            ui,
            show_body_response.rect,
            eyes_to_use,
            should_force_blink,
        );
    }
}

impl eframe::App for MuniTuberApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default()
            .frame(egui::Frame {
                fill: Color32::YELLOW,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    self.paint(ctx, ui);
                });
            });
        ctx.request_repaint();
    }
}

/// A change in the expression of the character. `None` means no change to the expression.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExpressionChange {
    eyes: Option<String>,
    head: Option<String>,
}

/// The current expression of the character.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExpressionState {
    eyes: String,
    head: String,
}

impl ExpressionState {
    pub fn apply(&mut self, change: &ExpressionChange) {
        if let Some(eyes) = &change.eyes {
            self.eyes = eyes.clone();
        }
        if let Some(head) = &change.head {
            self.head = head.clone();
        }
    }

    pub fn with(mut self, change: &ExpressionChange) -> Self {
        self.apply(change);
        self
    }
}
