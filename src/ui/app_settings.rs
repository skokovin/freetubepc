use std::fmt::format;
use std::sync::atomic::{AtomicUsize, Ordering};
use eframe::egui::{self, pos2, vec2, Button, Ui, Vec2};
use egui::Pos2;
use log::warn;
use crate::ui::keypad::{Cmd, Keypad};
pub static STRIGHT_SPEED: AtomicUsize = AtomicUsize::new(100);
pub static ROTATE_SPEED: AtomicUsize = AtomicUsize::new(10);
pub static ANGLE_SPEED: AtomicUsize = AtomicUsize::new(10);
pub struct AppSettings {
    start_pos: Pos2,
    pub is_open: bool,
    id: egui::Id,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            start_pos: pos2(300.0, 100.0),
            is_open: false,
            id: egui::Id::new("AppSettings"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, keypad: &mut Keypad) {
        let (rect, pixels_per_point) = ctx.input(|i: &egui::InputState| (i.screen_rect(), i.pixels_per_point()));

        egui::Window::new("Settings").open(&mut self.is_open).resizable(true).min_size([rect.size().x * 0.75, rect.size().y * 0.75]).show(ctx, |ui| {
            if ui.label(format!("STRIGHT_SPEED={:?} mm/sec", STRIGHT_SPEED.load(Ordering::Relaxed))).clicked() {
                keypad.set_open_close(ctx, Cmd::StrightSpeedCmd(STRIGHT_SPEED.load(Ordering::Relaxed)));
            };

            if ui.label(format!("ROTATE_SPEED={:?} mm/sec", ROTATE_SPEED.load(Ordering::Relaxed))).clicked() {
                keypad.set_open_close(ctx, Cmd::RotateSpeedCmd(ROTATE_SPEED.load(Ordering::Relaxed)));
            };

            if ui.label(format!("ANGLE_SPEED={:?} mm/sec", ANGLE_SPEED.load(Ordering::Relaxed))).clicked() {
                keypad.set_open_close(ctx, Cmd::AngleSpeedCmd(ANGLE_SPEED.load(Ordering::Relaxed)));
            };

            ui.label("This is a settings window.");
            ui.label("You can put any controls here.");
            ui.label("For instance, here is a button:");
            if ui.button("Click me").clicked() {
                keypad.set_open_close(ctx, Cmd::Dismiss);
            }
            ui.allocate_space(ui.available_size());
        });
    }
}