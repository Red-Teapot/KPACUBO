#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;

fn main() {
    let mut app = App::new();
    kpacubo::run(&mut app);
}
