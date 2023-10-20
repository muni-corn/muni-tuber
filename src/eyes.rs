use eframe::{
    egui::{Context, Image, Ui},
    epaint::Rect,
};
use egui_extras::RetainedImage;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

/// The minimum delay between blinks, in seconds.
const BLINK_MIN_DELAY: f32 = 1.0;

/// The maximum delay between blinks, in seconds.
const BLINK_MAX_DELAY: f32 = 5.0;

/// The duration of a blink, in seconds.
const BLINK_SECONDS: f32 = 0.2;

pub struct Eyes {
    /// Instant to keep track of when the character last changed phases.
    last_blink: Instant,

    /// The expression used in the last frame.
    last_expression: EyesExpression,

    /// The duration until the next blink.
    next_blink_time: Duration,

    /// The current phase of the blink animation.
    blink_phase: BlinkPhase,

    /// The default open eyes to use.
    default_open_img: RetainedImage,

    /// The default closed eyes to use.
    default_closed_img: RetainedImage,

    /// The image to use when the character's eyes are open.
    eyes_open_imgs: HashMap<EyesExpression, RetainedImage>,

    /// The image to use when the character's eyes are closed.
    eyes_closed_imgs: HashMap<EyesExpression, RetainedImage>,
}

impl Default for Eyes {
    fn default() -> Self {
        let now = Instant::now();
        let next_blink_time = Self::random_blink_delay();

        Self {
            last_blink: now,
            last_expression: EyesExpression::Normal,
            next_blink_time,
            blink_phase: BlinkPhase::Open,
            default_open_img: RetainedImage::from_image_bytes(
                "eyes_default_open",
                include_bytes!("assets/eyes_normal_open.png"),
            )
            .unwrap(),
            default_closed_img: RetainedImage::from_image_bytes(
                "eyes_default_closed",
                include_bytes!("assets/eyes_normal_closed.png"),
            )
            .unwrap(),
            eyes_open_imgs: HashMap::from([
                (
                    EyesExpression::Normal,
                    RetainedImage::from_image_bytes(
                        "eyes_open",
                        include_bytes!("assets/eyes_normal_open.png"),
                    )
                    .unwrap(),
                ),
                (
                    EyesExpression::Sad,
                    RetainedImage::from_image_bytes(
                        "eyes_sad_open",
                        include_bytes!("assets/eyes_sad_open.png"),
                    )
                    .unwrap(),
                ),
                (
                    EyesExpression::Angry,
                    RetainedImage::from_image_bytes(
                        "eyes_angry_open",
                        include_bytes!("assets/eyes_angry_open.png"),
                    )
                    .unwrap(),
                ),
            ]),
            eyes_closed_imgs: HashMap::from([
                (
                    EyesExpression::Normal,
                    RetainedImage::from_image_bytes(
                        "eyes_closed",
                        include_bytes!("assets/eyes_normal_closed.png"),
                    )
                    .unwrap(),
                ),
                (
                    EyesExpression::Sad,
                    RetainedImage::from_image_bytes(
                        "eyes_sad_closed",
                        include_bytes!("assets/eyes_sad_closed.png"),
                    )
                    .unwrap(),
                ),
                (
                    EyesExpression::Angry,
                    RetainedImage::from_image_bytes(
                        "eyes_angry_closed",
                        include_bytes!("assets/eyes_angry_closed.png"),
                    )
                    .unwrap(),
                ),
            ]),
        }
    }
}

impl Eyes {
    /// Returns a random duration between `BLINK_MIN_DELAY` and `BLINK_MAX_DELAY`.
    fn random_blink_delay() -> Duration {
        let delay = rand::random::<f32>() * (BLINK_MAX_DELAY - BLINK_MIN_DELAY) + BLINK_MIN_DELAY;
        Duration::from_secs_f32(delay)
    }

    /// Updates the state of the blinking animation.
    pub fn update(&mut self) {
        // get the time now
        let now = Instant::now();

        // if the time now has passed the next blink time, blink. set last_blink to now and
        // next_blink to some random delay.
        if now >= self.last_blink + self.next_blink_time {
            self.last_blink = now;
            self.next_blink_time = Self::random_blink_delay();
        }

        // determine if the eyes are closed now or not
        self.blink_phase = if now <= self.last_blink + Duration::from_secs_f32(BLINK_SECONDS) {
            BlinkPhase::Closed
        } else {
            BlinkPhase::Open
        }
    }

    /// Paints the eyes over the given rectangle. The rectangle should be the rectangle over which
    /// the head base was painted.
    pub fn paint(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        rect: Rect,
        expression: EyesExpression,
        force_shut: bool,
    ) {
        if expression != self.last_expression {
            self.last_blink = Instant::now();
            self.last_expression = expression;
        } else {
            self.update();
        }

        // decide which image to use
        let img = if matches!(self.blink_phase, BlinkPhase::Closed) || force_shut {
            self.eyes_closed_imgs
                .get(&expression)
                .or_else(|| self.eyes_open_imgs.get(&expression))
                .unwrap_or(&self.default_closed_img)
        } else {
            self.eyes_open_imgs
                .get(&expression)
                .or_else(|| self.eyes_closed_imgs.get(&expression))
                .unwrap_or(&self.default_open_img)
        };

        // paint the image over the given rectangle
        Image::new(img.texture_id(ctx), rect.size()).paint_at(ui, rect)
    }
}

enum BlinkPhase {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EyesExpression {
    Normal,
    Sad,
    Angry,
}
