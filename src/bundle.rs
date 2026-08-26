use std::ffi::OsStr;
use std::path::Path;

pub fn running_inside_app_bundle(exe: &Path) -> bool {
    let macos = exe.parent();
    let contents = macos.and_then(Path::parent);
    let app = contents.and_then(Path::parent);
    matches!(
        (
            macos.and_then(|path| path.file_name()),
            contents.and_then(|path| path.file_name()),
            app.and_then(|path| path.file_stem()),
            app.and_then(|path| path.extension()),
        ),
        (Some(macos), Some(contents), Some(name), Some(ext))
            if macos == OsStr::new("MacOS")
                && contents == OsStr::new("Contents")
                && name == OsStr::new("Okmate")
                && ext == OsStr::new("app")
    )
}

pub fn is_empty_gui_launch(args: &[String]) -> bool {
    match args {
        [_] => true,
        [_, flag] if flag.starts_with("-psn_") => true,
        _ => false,
    }
}

pub fn argv_for_parse(args: Vec<String>, bundled: bool) -> Vec<String> {
    if !(bundled && is_empty_gui_launch(&args)) {
        return args;
    }
    let mut argv = args;
    if argv.len() == 2 {
        argv.truncate(1);
    }
    argv.push("view".into());
    argv
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_okmate_app_layout() {
        let exe = PathBuf::from("/Applications/Okmate.app/Contents/MacOS/okmate");
        assert!(running_inside_app_bundle(&exe));
    }

    #[test]
    fn ignores_plain_binaries() {
        assert!(!running_inside_app_bundle(Path::new(
            "/usr/local/bin/okmate"
        )));
        assert!(!running_inside_app_bundle(Path::new(
            "/Applications/Other.app/Contents/MacOS/okmate"
        )));
    }

    #[test]
    fn bundled_empty_launch_becomes_view() {
        let exe = "/tmp/Okmate.app/Contents/MacOS/okmate".to_string();
        assert_eq!(
            argv_for_parse(vec![exe.clone()], true),
            vec![exe.clone(), "view".into()]
        );
        assert_eq!(
            argv_for_parse(vec![exe.clone(), "-psn_0_123".into()], true),
            vec![exe, "view".into()]
        );
    }

    #[test]
    fn unpackaged_empty_launch_is_unchanged() {
        let exe = "/usr/local/bin/okmate".to_string();
        assert_eq!(argv_for_parse(vec![exe.clone()], false), vec![exe]);
    }

    #[test]
    fn explicit_args_are_unchanged() {
        let exe = "/tmp/Okmate.app/Contents/MacOS/okmate".to_string();
        assert_eq!(
            argv_for_parse(vec![exe.clone(), "check".into()], true),
            vec![exe, "check".into()]
        );
    }
}
