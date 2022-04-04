/// Returns CARGO_PKG_VERSION + APP_REVISION runtime environment variable if availabe, otherwise return CARGO_PKG_VERSION only
pub fn get_app_version_with_revision() -> String {
    let version = env!("CARGO_PKG_VERSION");
    match std::env::var("APP_REVISION") {
        Ok(env) => format!("{}-{}", version, env),
        _ => version.to_string(),
    }
}
