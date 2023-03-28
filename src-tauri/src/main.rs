// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{distributions::{Alphanumeric, DistString}, Rng};

#[tauri::command]
fn create_window(app: tauri::AppHandle) {
    let mut rng = rand::thread_rng();
    let random_code = Alphanumeric.sample_string(&mut rng, 32);
    let random_x = rng.gen_range(0.0..1000.0);
    let random_y = rng.gen_range(0.0..1400.0);

    let window = tauri::WindowBuilder::new(&app, random_code, tauri::WindowUrl::App("index.html".into()))
        .title("Pomorodo Timer")
        .always_on_top(true)
        .max_inner_size(200.0, 150.0)
        .position(random_x, random_y)
        .build()
        .unwrap();
    window.show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_window])
        .setup(|app| {
            let window = tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                .title("Pomorodo Timer")
                .always_on_top(true)
                .max_inner_size(200.0, 150.0)
                .position(-10.0, 100.0)
                .build()
                .unwrap();
            window.show().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
