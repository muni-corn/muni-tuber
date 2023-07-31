use std::time::Instant;

use eframe::{
    egui::{self, CentralPanel, Context, Vec2},
    Frame,
};
use egui_extras::RetainedImage;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::<MuniTuberApp>::default()),
    )
}

struct MuniTuberApp {
    start: Instant,
    quiet: RetainedImage,
}

impl Default for MuniTuberApp {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            quiet: RetainedImage::from_image_bytes(
                "quiet",
                include_bytes!("assets/png_muni_quiet.png"),
            )
            .unwrap(),
        }
    }
}

impl eframe::App for MuniTuberApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                let breath_value = self.start.elapsed().as_secs_f32().sin() / 75.0;
                let breath_scale_x = 1.0 - breath_value;
                let breath_scale_y = 1.0 + breath_value;
                self.quiet.show_size(
                    ui,
                    Vec2::new(200.0 * breath_scale_x, 200.0 * breath_scale_y),
                )
            });
        });
        ctx.request_repaint();
    }
}
