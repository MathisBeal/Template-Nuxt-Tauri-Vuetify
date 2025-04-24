use std::sync::Mutex;
use tauri::State;

struct Counter {
    value: Mutex<i32>,
}

#[tauri::command]
fn increment_counter(counter: State<Counter>) -> i32 {
    let mut value = counter.value.lock().unwrap();
    *value += 1;
    *value
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Counter {
            value: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![increment_counter])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
