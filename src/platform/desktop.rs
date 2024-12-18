use super::TimeProvider;

pub struct DesktopTime;

impl TimeProvider for DesktopTime {
    fn now() -> instant::Instant {
        instant::Instant::now()
    }

    fn delay(duration: std::time::Duration) -> impl std::future::Future<Output = ()> {
        futures_timer::Delay::new(duration)
    }
}
