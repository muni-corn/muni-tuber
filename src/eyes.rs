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

    /// The time of the next blink.
    next_blink: Instant,

    /// The current phase of the blink animation.
    blink_phase: BlinkPhase,

    /// The current expression of the eyes.
    expression: EyesExpression,

    /// The image to use when the character's eyes are open.
    eyes_open_imgs: HashMap<EyesExpression, RetainedImage>,

    /// The image to use when the character's eyes are closed.
    eyes_closed_imgs: HashMap<EyesExpression, RetainedImage>,
}

impl Default for Eyes {
    fn default() -> Self {
        let now = Instant::now();
        let next_blink = now + Self::random_blink_delay();

        Self {
            last_blink: now,
            next_blink,
            blink_phase: BlinkPhase::Open,
            expression: EyesExpression::Normal,
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
        if now >= self.next_blink {
            self.last_blink = now;
            self.next_blink = now + Self::random_blink_delay();
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
    pub fn paint(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
        self.update();

        // decide which image to use
        let map = match self.blink_phase {
            BlinkPhase::Open => &self.eyes_open_imgs,
            BlinkPhase::Closed => &self.eyes_closed_imgs,
        };
        let img = map.get(&self.expression).unwrap();

        // paint the image over the given rectangle
        Image::new(img.texture_id(ctx), rect.size()).paint_at(ui, rect)
    }
}

enum BlinkPhase {
    Open,
    Closed,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum EyesExpression {
    Normal,
    Sad,
    Angry,
}
