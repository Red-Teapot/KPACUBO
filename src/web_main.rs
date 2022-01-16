use bevy::prelude::*;
use std::cell::Cell;
use wasm_bindgen::prelude::*;

// Wasm is single-threaded anyways
thread_local! {
    static RESIZE_HINT: Cell<Option<(f32, f32)>> = Cell::new(None);
}

#[wasm_bindgen]
pub fn web_main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_system(resize_system);

    crate::run(&mut app);
}

// FIXME Replace this check called each frame with something proper
fn resize_system(mut windows: ResMut<Windows>) {
    RESIZE_HINT.with(|hint| {
        if let Some((width, height)) = hint.get() {
            windows.get_primary_mut().map(|windows| {
                windows.set_resolution(width as f32, height as f32);
                hint.set(None);
            });
        }
    });
}

#[wasm_bindgen]
pub fn handle_resize(width: f32, height: f32) {
    RESIZE_HINT.with(|hint| {
        hint.set(Some((width, height)));
    });
}
