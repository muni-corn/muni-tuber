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
    last_expression_name: String,

    /// The duration until the next blink.
    next_blink_time: Duration,

    /// The current phase of the blink animation.
    blink_phase: BlinkPhase,

    /// Images to fallback to when an expression is not found.
    default_expression: EyesExpression,

    /// Expression images to use when the character's eyes are open.
    expressions: HashMap<String, EyesExpression>,
}

impl Default for Eyes {
    fn default() -> Self {
        let now = Instant::now();
        let next_blink_time = Self::random_blink_delay();

        Self {
            last_blink: now,
            last_expression_name: String::new(),
            next_blink_time,
            blink_phase: BlinkPhase::Open,
            default_expression: EyesExpression {
                idle: RetainedImage::from_image_bytes(
                    "eyes_default_open",
                    include_bytes!("assets/eyes_normal_open.png"),
                )
                .unwrap(),
                blink: Some(
                    RetainedImage::from_image_bytes(
                        "eyes_default_closed",
                        include_bytes!("assets/eyes_normal_closed.png"),
                    )
                    .unwrap(),
                ),
            },
            expressions: HashMap::from([
                (
                    "sad".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_sad_open",
                            include_bytes!("assets/eyes_sad_open.png"),
                        )
                        .unwrap(),
                        blink: Some(
                            RetainedImage::from_image_bytes(
                                "eyes_sad_closed",
                                include_bytes!("assets/eyes_sad_closed.png"),
                            )
                            .unwrap(),
                        ),
                    },
                ),
                (
                    "angry".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_angry_open",
                            include_bytes!("assets/eyes_angry_open.png"),
                        )
                        .unwrap(),
                        blink: Some(
                            RetainedImage::from_image_bytes(
                                "eyes_angry_closed",
                                include_bytes!("assets/eyes_angry_closed.png"),
                            )
                            .unwrap(),
                        ),
                    },
                ),
                (
                    "wide".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_wide_open",
                            include_bytes!("assets/eyes_wide.png"),
                        )
                        .unwrap(),
                        blink: Some(
                            RetainedImage::from_image_bytes(
                                "eyes_tight_shut",
                                include_bytes!("assets/eyes_tight.png"),
                            )
                            .unwrap(),
                        ),
                    },
                ),
                (
                    "dreamy".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_dreamy_open",
                            include_bytes!("assets/eyes_dreamy_open.png"),
                        )
                        .unwrap(),
                        blink: Some(
                            RetainedImage::from_image_bytes(
                                "eyes_dreamy_closed",
                                include_bytes!("assets/eyes_dreamy_closed.png"),
                            )
                            .unwrap(),
                        ),
                    },
                ),
                (
                    "happy".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_smiling",
                            include_bytes!("assets/eyes_happy.png"),
                        )
                        .unwrap(),
                        blink: None,
                    },
                ),
                (
                    "tight".to_string(),
                    EyesExpression {
                        idle: RetainedImage::from_image_bytes(
                            "eyes_tight",
                            include_bytes!("assets/eyes_tight.png"),
                        )
                        .unwrap(),
                        blink: None,
                    },
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
        expression_name: &str,
        force_shut: bool,
    ) {
        // blink now if our expression has changed
        if expression_name != self.last_expression_name {
            self.last_blink = Instant::now();
            self.last_expression_name = expression_name.to_string();
        } else {
            self.update();
        }

        // get the expression to use, or fallback to default
        let expression = self
            .expressions
            .get(expression_name)
            .unwrap_or(&self.default_expression);

        // decide which image to use
        let img = if matches!(self.blink_phase, BlinkPhase::Closed) || force_shut {
            expression.blink.as_ref().unwrap_or(&expression.idle)
        } else {
            &expression.idle
        };

        // paint the image over the given rectangle
        Image::new(img.texture_id(ctx), rect.size()).paint_at(ui, rect)
    }
}

enum BlinkPhase {
    Open,
    Closed,
}

pub struct EyesExpression {
    pub idle: RetainedImage,
    pub blink: Option<RetainedImage>,
}
