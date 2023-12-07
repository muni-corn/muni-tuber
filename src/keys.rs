use std::collections::HashMap;

use eframe::egui::{Context, Key};

use crate::ExpressionChange;

pub struct ExpressionHotkeyManager {
    pub force_blink_key: Key,
    pub expression_switches: HashMap<Key, ExpressionChange>,
    pub expression_holds: HashMap<Key, ExpressionChange>,
}

impl ExpressionHotkeyManager {
    pub fn should_force_blink(&self, ctx: &Context) -> bool {
        ctx.input(|i| i.key_down(self.force_blink_key))
    }

    /// Returns the expression to switch to if its key was pressed, or None if no key is pressed.
    pub fn get_expression(&self, ctx: &Context) -> Option<&ExpressionChange> {
        self.expression_switches
            .iter()
            .find_map(|(key, expression)| {
                if ctx.input(|i| i.key_pressed(*key)) {
                    Some(expression)
                } else {
                    None
                }
            })
    }

    /// Returns a temporary expression to use if its key is held down.
    pub fn get_temporary_expression(&self, ctx: &Context) -> Option<&ExpressionChange> {
        self.expression_holds.iter().find_map(|(key, expression)| {
            if ctx.input(|i| i.key_down(*key)) {
                Some(expression)
            } else {
                None
            }
        })
    }
}
