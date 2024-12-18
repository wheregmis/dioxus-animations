#[cfg(feature = "native_animation")]
mod desktop;
#[cfg(feature = "wasm_animation")]
mod web;

#[cfg(feature = "native_animation")]
pub use desktop::*;
#[cfg(feature = "wasm_animation")]
pub use web::*;

pub trait TimeProvider {
    fn now() -> instant::Instant;
    fn delay(duration: std::time::Duration) -> impl std::future::Future<Output = ()>;
}
