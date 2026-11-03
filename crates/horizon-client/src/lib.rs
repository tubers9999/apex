//! Gameplay and startup, above the reusable client library.
// No `#![recursion_limit]` or other inner attribute horizon-mixins doesn't know here: the
// launcher's horizon-mixins then rejects every native mod targeting this crate (#8169).
#![cfg_attr(not(target_os = "android"), forbid(unsafe_code))]
#![cfg_attr(target_os = "android", allow(unsafe_code))]
extern crate self as horizon_client;
pub use horizon_client_lib::*;
pub mod entry;
pub mod game;
pub mod mod_api;

#[cfg(target_os = "android")]
#[global_allocator]
static ALLOCATOR: std::alloc::System = std::alloc::System;

#[cfg(target_os = "android")]
pub static ANDROID_APP: std::sync::OnceLock<android_activity::AndroidApp> = std::sync::OnceLock::new();

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: android_activity::AndroidApp) {
    let _ = ANDROID_APP.set(app.clone());
    let internal_path = app
        .internal_data_path()
        .unwrap_or_else(|| std::path::PathBuf::from("/data/data/dev.redglitchx.horizon/files"));
    std::fs::create_dir_all(&internal_path).ok();
    logging::set_game_home(internal_path);
    entry::main();
}
