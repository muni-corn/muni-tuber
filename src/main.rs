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
                    ExpressionState {
                        eyes: eyes::EyesExpression::Normal,
                        head: head::HeadExpression::Happy,
                    },
                ),
                (
                    Key::F2,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Angry,
                        head: head::HeadExpression::Frown,
                    },
                ),
                (
                    Key::F3,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Sad,
                        head: head::HeadExpression::Frown,
                    },
                ),
                (
                    Key::F4,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Angry,
                        head: head::HeadExpression::Happy,
                    },
                ),
                (
                    Key::F5,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Sad,
                        head: head::HeadExpression::Happy,
                    },
                ),
                (
                    Key::F7,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Sad,
                        head: head::HeadExpression::Wavy,
                    },
                ),
                (
                    Key::F8,
                    ExpressionState {
                        eyes: eyes::EyesExpression::Wide,
                        head: head::HeadExpression::Wavy,
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
            let a = 1.0 / POP_DURATION + 0.5;

            -1.0 * (a * x - POP_DURATION / 2.0).powi(2) + 1.0
        }
        .max(0.0);

        let breath_value = (self.start.elapsed().as_secs_f32() * 1.5).sin() + pop_value * POP_AMOUNT;
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
            self.expression = *new_expression;
        }
        let expression_to_use = self
            .hotkey_manager
            .get_temporary_expression(ctx)
            .unwrap_or(&self.expression);

        // draw head and eyes
        let volume = *self.audio_state.volume.lock().unwrap();
        self.head.paint(
            ctx,
            ui,
            show_body_response.rect,
            volume,
            expression_to_use.head,
        );
        self.eyes.paint(
            ctx,
            ui,
            show_body_response.rect,
            expression_to_use.eyes,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionState {
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
