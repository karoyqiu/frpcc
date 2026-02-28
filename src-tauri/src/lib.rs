use std::{collections::VecDeque, fs, path::PathBuf, sync::Mutex};

use tauri::{AppHandle, Manager, State, command};
use tauri_plugin_shell::{ShellExt, process::{CommandChild, CommandEvent}};

const MAX_LOG_LINES: usize = 8192;

/// Shared application state that keeps the sidecar child handle and
/// in-memory line-oriented buffers for stdout/stderr.
///
/// We wrap this in an Arc and manage it via `app.manage(...)`. When the
/// state is dropped (on app shutdown), `Drop` is invoked to attempt a
/// graceful shutdown of frpc (via `frpc stop`) and ensure the child is
/// killed/waited on.
pub struct FrpcState {
  app: AppHandle,
  // 配置文件路径
  config: PathBuf,
  // store the plugin child as a type-erased boxed value so we can keep it in managed state
  child: Option<CommandChild>,
  // merged single log buffer (stdout+stderr merged)
  logs: VecDeque<String>,
}

impl FrpcState {
  pub fn new(app: AppHandle, config: PathBuf, child: CommandChild) -> Self {
    Self {
      app,
      config,
      child: Some(child),
      logs: VecDeque::with_capacity(MAX_LOG_LINES),
    }
  }

  fn push_log_line(&mut self, line: String) {
    if self.logs.len() == MAX_LOG_LINES {
      self.logs.pop_front();
    }

    self.logs.push_back(line);
  }

  /// Return up to `count` most-recent lines in chronological order.
  fn get_recent(&self, count: usize) -> Vec<String> {
    let total = self.logs.len();

    if total == 0 {
      return Vec::new();
    }

    let take = std::cmp::min(count, total);
    self
      .logs
      .iter()
      .skip(total - take)
      .cloned()
      .collect::<Vec<String>>()
  }

  fn stop(&mut self) -> Result<(), tauri_plugin_shell::Error> {
    let frpc = self.app.shell().sidecar("frpc")?;
    let cmd = frpc.args(["stop", "-c", &self.config.to_string_lossy()]);
    tauri::async_runtime::block_on(async move {
      let _ = cmd.status().await;
    });
    Ok(())
  }
}

impl Drop for FrpcState {
  fn drop(&mut self) {
    if let Some(child) = self.child.take() {
      let _ = child.kill();
    }
  }
}

#[command]
fn get_frpc_logs(state: State<'_, Mutex<FrpcState>>, count: Option<usize>) -> Vec<String> {
  let state = state.lock().unwrap();
  let c = count.unwrap_or(20);
  state.get_recent(c)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![get_frpc_logs])
    .setup(|app| {
      // Resolve config path in AppConfig
      let filename: PathBuf = app
        .path()
        .resolve("config.toml", tauri::path::BaseDirectory::AppConfig)?;

      if !filename.exists() {
        let default = include_str!("default.toml");
        if let Some(parent) = filename.parent() {
          let _ = std::fs::create_dir_all(parent);
        }
        fs::write(&filename, default)?;
      }

      // prepare shared state and manage it
      let app_handle = app.handle().clone();

      // Try to build the sidecar command `frpc -c <config>`
      let frpc = app.shell().sidecar("frpc")?;
      let cmd = frpc.args(["-c", &filename.to_string_lossy()]);
      let (mut rx, child) = cmd.spawn()?;

      // store the plugin child into managed state as a boxed Any so Drop or other code can access it
      app.manage(Mutex::new(FrpcState::new(
        app_handle.clone(),
        filename,
        child,
      )));

      // Spawn an asynchronous task to receive CommandEvent values (Stdout / Stderr / Terminated).
      // Use the tauri async runtime to avoid blocking the OS threads and integrate with app runtime.
      tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
          match event {
            // both stdout and stderr are merged into a single buffer
            CommandEvent::Stdout(bytes) | CommandEvent::Stderr(bytes) => {
              let state = app_handle.state::<Mutex<FrpcState>>();
              let mut state = state.lock().unwrap();
              let s = String::from_utf8_lossy(&bytes).to_string();

              for l in s.lines() {
                println!("frpc: {}", l);
                state.push_log_line(l.to_string());
              }
            }
            CommandEvent::Terminated(_code) => {
              // frpc terminated; continue draining events if any
            }
            _ => {}
          }
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
