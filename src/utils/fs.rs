use std::env;
use std::path::PathBuf;

/// Use this instead of application_root_dir, because of Windows problems.
/// See https://github.com/rust-lang/rust/issues/42869
pub fn root() -> PathBuf {
    env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or(())
        .or_else(|_| {
            let mut p = env::current_exe()?;
            assert!(p.pop());
            Ok(p)
        })
        .and_then(dunce::canonicalize)
        .expect("Failed to get root dir")
}
