use super::TimeProvider;
use instant::Duration;
use wasm_bindgen::prelude::*;
use web_sys::Performance;

pub struct WebTime;

impl TimeProvider for WebTime {
    fn now() -> instant::Instant {
        instant::Instant::now()
    }

    fn delay(duration: Duration) -> impl std::future::Future<Output = ()> {
        futures_timer::Delay::new(duration)
    }
}
