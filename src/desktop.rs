use anyhow::{Context, Result};

const PRODUCT_SCRIPT: &str = "window.__h35FindRoot = '#okmate-main';\n";

const APP_NAME: &str = "OKMate";

const APP_ICON: &[u8] = include_bytes!("../assets/brand/okmate-app-icon-macos.png");

#[cfg(target_os = "macos")]
fn set_macos_app_name(name: &str) {
    use objc2_foundation::{NSProcessInfo, NSString};
    NSProcessInfo::processInfo().setProcessName(&NSString::from_str(name));
}

pub fn run(options: crate::preview::ViewOptions) -> Result<()> {
    #[cfg(target_os = "macos")]
    set_macos_app_name(APP_NAME);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
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
    h35_desktop::preview(h35_desktop::HostOptions {
        title: APP_NAME.into(),
        identifier: "dev.okmate.preview".into(),
        icon_png: Some(APP_ICON),
        state_dir: crate::preview::state_dir(),
        url: ready.initial_url,
        home_url: Some(ready.home_url),
        live_reload: true,
        goto: false,
        find: true,
        extra_initialization_script: Some(PRODUCT_SCRIPT.into()),
        check_updates: std::env::current_exe()
            .ok()
            .is_some_and(|exe| crate::bundle::running_inside_app_bundle(&exe)),
        tab_shortcuts: true,
        ..h35_desktop::HostOptions::default()
    })
    .map_err(|error| anyhow::anyhow!("{error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_script_selects_find_root() {
        assert!(PRODUCT_SCRIPT.contains("window.__h35FindRoot = '#okmate-main'"));
    }

    #[test]
    fn desktop_host_sets_macos_app_name() {
        let source = include_str!("desktop.rs");
        assert!(source.contains("setProcessName"));
        assert!(source.contains("const APP_NAME: &str = \"OKMate\""));
    }

    #[test]
    fn desktop_host_enables_tab_shortcuts() {
        let source = include_str!("desktop.rs");
        assert!(source.contains("tab_shortcuts: true"));
    }

    #[test]
    fn desktop_host_embeds_the_okmate_icon() {
        assert!(APP_ICON.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert_eq!(
            APP_ICON[25], 6,
            "macOS dock icon must be RGBA so corners can be transparent"
        );
    }

    #[test]
    #[ignore = "opens a native window"]
    fn window_smoke() {}
}
