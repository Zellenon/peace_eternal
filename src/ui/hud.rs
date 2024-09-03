use bevy::{
    core::Name,
    prelude::{Entity, Query, With},
    reflect::Reflect,
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};

use crate::gameplay::{
    inventory::{components::Inventory, swapping::HoldingInventoryItem},
    levels_setup::IsPlayer,
    Arm,
};

#[derive(Debug, Reflect, Clone, Copy)]
enum HudSlotState {
    EmptyInactive,
    EmptyActive,
    FullInactive,
    FullActive,
}

pub(crate) fn hotbar_ui(
    mut root: EguiContexts,
    player: Query<(Entity, &Inventory), With<IsPlayer>>,
    names: Query<&Name>,
    arms: Query<(&HoldingInventoryItem, &Arm)>,
) {
    let (player_entity, player_inventory) = player.get_single().unwrap();
    let (player_held_item, _) = arms
        .iter()
        .filter(|(_held, arm)| arm.parent == player_entity)
        .next()
        .unwrap();
    let names = player_inventory.slots.iter().map(|w| match w.contents {
        Some(item_entity) => names
            .get(item_entity)
            .ok()
            .map(|w| w.into())
            .unwrap_or(" ?? ".to_owned()),
        None => " [ ] ".to_owned(),
    });
    let slot_states = player_inventory.slots.iter().enumerate().map(|(i, w)| {
        match (
            player_held_item.held_slot.map(|w| w == i).unwrap_or(false),
            w.contents,
        ) {
            (true, None) => HudSlotState::EmptyActive,
            (true, Some(_)) => HudSlotState::FullActive,
            (false, None) => HudSlotState::EmptyInactive,
            (false, Some(_)) => HudSlotState::FullInactive,
        }
    });
    egui::Window::new("HUD1")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(50., 50.))
        .show(root.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Items:");
                for (name, state) in names.into_iter().zip(slot_states.into_iter()) {
                    ui.visuals_mut().selection = egui::style::Selection {
                        bg_fill: egui::Color32::RED,
                        ..Default::default()
                    };
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        ui.visuals_mut().widgets.noninteractive.fg_stroke.color = match state {
                            HudSlotState::EmptyInactive => Color32::BLACK,
                            HudSlotState::EmptyActive => Color32::WHITE,
                            HudSlotState::FullInactive => Color32::GRAY,
                            HudSlotState::FullActive => Color32::GREEN,
                        };
                        ui.label(name);
                    });
                }
            })
        });
}

pub(crate) fn hud_ui(mut root: EguiContexts) {
    egui::Window::new("HUD2")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Health:");
                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "000000XX");
            })
        });
}

pub(crate) fn third_gui(mut root: EguiContexts) {
    egui::Window::new("HUD3")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::RIGHT_CENTER, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                let temp = vec!["Faster", "AoE", "Dubler", "Eraser"];
                for item in temp.into_iter() {
                    egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                        ui.label(item);
                    });
                }
            })
        });
}
