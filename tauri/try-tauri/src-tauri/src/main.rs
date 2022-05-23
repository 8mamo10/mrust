#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use serde::{Deserialize, Serialize};
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.app_handle();
      std::thread::spawn(move || loop {
        app_handle
          .emit_all("back-to-front", "ping frontend".to_string())
          .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1))
      });
      app.listen_global("front-to-back", |event| {
        println!(
          "got front-to-back with paylot {:?}",
          event.payload().unwrap()
        )
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      simple_command,
      command_with_message,
      command_with_object,
      command_with_error,
      async_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn simple_command() {
  println!("simple_command");
  println!("I was invoked from JS!");
}

#[tauri::command]
fn command_with_message(message: String) -> String {
  println!("command_with_message");
  format!("hello {}", message)
}

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
  field_str: String,
  field_u32: u32,
}

#[tauri::command]
fn command_with_object(message: MyMessage) -> MyMessage {
  println!("command_with_object");
  let MyMessage {
    field_str,
    field_u32,
  } = message;
  MyMessage {
    field_str: format!("hello {}", field_str),
    field_u32: field_u32 + 1,
  }
}

#[tauri::command]
fn command_with_error(arg: u32) -> Result<String, String> {
  println!("command_with_error");
  if arg % 2 == 0 {
    Ok(format!("even value {}", arg))
  } else {
    Err(format!("odd value {}", arg))
  }
}

#[tauri::command]
async fn async_command(arg: u32) -> String {
  println!("async_command");
  "hello".into()
}
