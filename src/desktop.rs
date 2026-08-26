use std::thread;

use anyhow::{Context, Result};
use muda::{
    Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    accelerator::{Accelerator, CMD_OR_CTRL, Code},
};
use tao::event::{ElementState, Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::keyboard::{KeyCode, ModifiersState};
use tao::platform::run_return::EventLoopExtRunReturn;
use tao::{dpi::LogicalSize, window::WindowBuilder};
use wry::{WebContext, WebViewBuilder, http::Request};

const QUIT_ID: &str = "app.quit";
const CLOSE_WINDOW_ID: &str = "file.close-window";
const CLOSE_KEYS_SCRIPT: &str = r#"
window.addEventListener("keydown", (event) => {
  if (!(event.metaKey || event.ctrlKey) || event.altKey || event.shiftKey) {
    return;
  }
  const key = event.key.toLowerCase();
  if (key !== "w" && key !== "q") {
    return;
  }
  event.preventDefault();
  event.stopPropagation();
  if (window.ipc && window.ipc.postMessage) {
    window.ipc.postMessage(key === "q" ? "quit" : "close");
  }
}, true);
"#;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IpcMessage {
    PickFolder,
    Home,
    Close,
    Quit,
}

impl IpcMessage {
    pub fn parse(message: &str) -> Option<Self> {
        match message.trim() {
            "pick-folder" => Some(Self::PickFolder),
            "home" => Some(Self::Home),
            "close" => Some(Self::Close),
            "quit" => Some(Self::Quit),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
enum DesktopEvent {
    PickFolder,
    PickFolderResult(Option<String>),
    Home,
    Quit,
}

pub fn pick_folder_result_script(path: Option<&str>) -> String {
    let detail = match path {
        Some(path) => format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(path).unwrap_or_else(|_| "null".into())
        ),
        None => r#"{"path":null}"#.to_string(),
    };
    format!(r#"window.dispatchEvent(new CustomEvent("okmate-pick-folder",{{detail:{detail}}}));"#)
}

pub fn home_url(bound: impl std::fmt::Display) -> String {
    crate::preview::home_url(bound)
}

pub fn run(options: crate::preview::ViewOptions) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = tx.send(Err(anyhow::anyhow!(error)));
                return;
            }
        };
        let result = runtime.block_on(crate::preview::serve_ready(options, tx.clone()));
        if let Err(error) = result {
            let _ = tx.send(Err(error));
        }
    });
    let ready = rx
        .recv()
        .context("preview server thread exited before binding")?;
    let ready = ready?;
    open_window(&ready.initial_url, &ready.home_url)
}

struct Live {
    webview: wry::WebView,
    _context: WebContext,
}

fn open_window(initial_url: &str, home_url: &str) -> Result<()> {
    let mut event_loop = EventLoopBuilder::<DesktopEvent>::with_user_event().build();
    #[cfg(target_os = "macos")]
    {
        use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
        event_loop.set_activation_policy(ActivationPolicy::Regular);
    }
    let proxy = event_loop.create_proxy();
    let mut window = WindowBuilder::new()
        .with_title("Okmate")
        .with_inner_size(LogicalSize::new(1200.0, 800.0));
    #[cfg(target_os = "macos")]
    {
        use tao::platform::macos::WindowBuilderExtMacOS;
        window = window.with_automatic_window_tabbing(false);
    }
    let window = window
        .build(&event_loop)
        .context("failed to create Okmate window")?;

    let menu = install_menu(proxy.clone())?;
    let mut context = WebContext::new(None);
    let ipc_proxy = proxy.clone();
    let webview = WebViewBuilder::new_with_web_context(&mut context)
        .with_url(initial_url)
        .with_initialization_script(CLOSE_KEYS_SCRIPT)
        .with_ipc_handler(
            move |request: Request<String>| match IpcMessage::parse(request.body()) {
                Some(IpcMessage::PickFolder) => {
                    let _ = ipc_proxy.send_event(DesktopEvent::PickFolder);
                }
                Some(IpcMessage::Home) => {
                    let _ = ipc_proxy.send_event(DesktopEvent::Home);
                }
                Some(IpcMessage::Close | IpcMessage::Quit) => {
                    let _ = ipc_proxy.send_event(DesktopEvent::Quit);
                }
                None => {}
            },
        )
        .build(&window)
        .context("failed to create Okmate webview")?;

    let live = Live {
        webview,
        _context: context,
    };
    let home = home_url.to_string();
    let pick_proxy = proxy;
    let mut modifiers = ModifiersState::empty();

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let _keep = &menu;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::UserEvent(DesktopEvent::Quit) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::ModifiersChanged(next),
                ..
            } => modifiers = next,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } if is_close_key_event(&event, modifiers) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(DesktopEvent::PickFolder) => {
                start_pick_folder(pick_proxy.clone());
            }
            Event::UserEvent(DesktopEvent::PickFolderResult(path)) => {
                let _ = live
                    .webview
                    .evaluate_script(&pick_folder_result_script(path.as_deref()));
            }
            Event::UserEvent(DesktopEvent::Home) => {
                let _ = live.webview.load_url(&home);
            }
            _ => {}
        }
    });
    Ok(())
}

fn start_pick_folder(proxy: EventLoopProxy<DesktopEvent>) {
    thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(_) => {
                let _ = proxy.send_event(DesktopEvent::PickFolderResult(None));
                return;
            }
        };
        let picked = runtime.block_on(async {
            rfd::AsyncFileDialog::new()
                .set_title("Choose knowledge folder")
                .pick_folder()
                .await
        });
        let path = picked.map(|handle| handle.path().to_string_lossy().into_owned());
        let _ = proxy.send_event(DesktopEvent::PickFolderResult(path));
    });
}

fn install_menu(proxy: EventLoopProxy<DesktopEvent>) -> Result<Menu> {
    MenuEvent::set_event_handler(Some(move |event| {
        if is_menu_id(&event, QUIT_ID) || is_menu_id(&event, CLOSE_WINDOW_ID) {
            let _ = proxy.send_event(DesktopEvent::Quit);
        }
    }));

    let menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        let app = Submenu::new("Okmate", true);
        app.append_items(&[
            &PredefinedMenuItem::about(Some("About Okmate"), None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(Some("Hide Okmate")),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(Some("Quit Okmate")),
        ])
        .context("failed to add Okmate menu items")?;
        menu.append(&app).context("failed to add Okmate menu")?;
    }
    let file = Submenu::new("File", true);
    file.append(&MenuItem::with_id(
        CLOSE_WINDOW_ID,
        "Close Window",
        true,
        Some(Accelerator::new(Some(CMD_OR_CTRL), Code::KeyW)),
    ))
    .context("failed to add Close Window menu item")?;
    menu.append(&file).context("failed to add File menu")?;
    #[cfg(target_os = "macos")]
    menu.init_for_nsapp();
    Ok(menu)
}

fn is_menu_id(event: &MenuEvent, expected: &str) -> bool {
    event.id() == &MenuId::new(expected)
}

fn is_close_shortcut(key: KeyCode, modifiers: ModifiersState) -> bool {
    matches!(key, KeyCode::KeyQ | KeyCode::KeyW) && close_modifier(modifiers)
}

fn is_close_key_event(event: &tao::event::KeyEvent, modifiers: ModifiersState) -> bool {
    event.state == ElementState::Pressed
        && !event.repeat
        && is_close_shortcut(event.physical_key, modifiers)
}

fn close_modifier(modifiers: ModifiersState) -> bool {
    #[cfg(target_os = "macos")]
    {
        modifiers.super_key()
            && !modifiers.control_key()
            && !modifiers.alt_key()
            && !modifiers.shift_key()
    }
    #[cfg(not(target_os = "macos"))]
    {
        modifiers.control_key()
            && !modifiers.super_key()
            && !modifiers.alt_key()
            && !modifiers.shift_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ipc_pick_folder_and_home() {
        assert_eq!(
            IpcMessage::parse("pick-folder"),
            Some(IpcMessage::PickFolder)
        );
        assert_eq!(IpcMessage::parse("  home  "), Some(IpcMessage::Home));
        assert_eq!(IpcMessage::parse("close"), Some(IpcMessage::Close));
        assert_eq!(IpcMessage::parse("quit"), Some(IpcMessage::Quit));
        assert_eq!(IpcMessage::parse("osascript"), None);
        assert_eq!(IpcMessage::parse("pick-folder-http"), None);
    }

    #[test]
    fn pick_folder_script_json_escapes_paths() {
        let script = pick_folder_result_script(Some(r#"C:\tmp\"quotes""#));
        assert!(script.contains("okmate-pick-folder"), "{script}");
        assert!(script.contains(r#"C:\\tmp\\\"quotes\""#), "{script}");
        let cancelled = pick_folder_result_script(None);
        assert!(cancelled.contains(r#""path":null"#), "{cancelled}");
    }

    #[test]
    fn home_url_is_origin_root() {
        assert_eq!(home_url("127.0.0.1:8000"), "http://127.0.0.1:8000/");
    }

    #[test]
    fn close_shortcuts_use_platform_modifier() {
        #[cfg(target_os = "macos")]
        {
            assert!(is_close_shortcut(KeyCode::KeyW, ModifiersState::SUPER));
            assert!(is_close_shortcut(KeyCode::KeyQ, ModifiersState::SUPER));
            assert!(!is_close_shortcut(KeyCode::KeyW, ModifiersState::CONTROL));
            assert!(!is_close_shortcut(
                KeyCode::KeyQ,
                ModifiersState::SUPER | ModifiersState::SHIFT
            ));
        }
        #[cfg(not(target_os = "macos"))]
        {
            assert!(is_close_shortcut(KeyCode::KeyW, ModifiersState::CONTROL));
            assert!(is_close_shortcut(KeyCode::KeyQ, ModifiersState::CONTROL));
            assert!(!is_close_shortcut(KeyCode::KeyW, ModifiersState::SUPER));
        }
    }

    #[test]
    #[ignore = "opens a native window"]
    fn window_smoke() {}
}
