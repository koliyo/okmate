use anyhow::{Context, Result};

const PICK_FOLDER_ALIAS: &str = r#"
window.addEventListener("h35-pick-folder", function (event) {
  window.dispatchEvent(new CustomEvent("okmate-pick-folder", { detail: event.detail }));
});
"#;

pub fn run(options: crate::preview::ViewOptions) -> Result<()> {
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
        title: "Okmate".into(),
        identifier: "dev.okmate.preview".into(),
        state_dir: crate::preview::state_dir(),
        url: ready.initial_url,
        home_url: Some(ready.home_url),
        live_reload: true,
        goto: false,
        find: true,
        extra_initialization_script: Some(PICK_FOLDER_ALIAS.into()),
        ..h35_desktop::HostOptions::default()
    })
    .map_err(|error| anyhow::anyhow!("{error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_folder_alias_forwards_h35_event() {
        assert!(PICK_FOLDER_ALIAS.contains("h35-pick-folder"));
        assert!(PICK_FOLDER_ALIAS.contains("okmate-pick-folder"));
    }

    #[test]
    #[ignore = "opens a native window"]
    fn window_smoke() {}
}
