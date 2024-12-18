use dioxus_hooks::{use_coroutine, use_signal, Coroutine};
use dioxus_signals::{Readable, Signal, Writable};
use easer::functions::{Easing, Linear};
use futures_util::StreamExt;

use uuid::Uuid;

mod platform;
use platform::TimeProvider;

// Single conditional import block for Time
#[cfg(not(feature = "wasm_animation"))]
use {platform::DesktopTime as Time, std::time::Duration};

#[cfg(feature = "wasm_animation")]
use {instant::Duration, platform::WebTime as Time};

#[derive(Debug, Clone, PartialEq)]
enum AnimationState {
    Idle,
    Running,
    Completed,
}

#[derive(Debug, Clone, Copy)]
pub struct Motion {
    id: Uuid,
    initial: f32,
    target: f32,
    duration: Duration,
    easing: fn(f32, f32, f32, f32) -> f32,
    on_complete: Option<fn()>,
}

impl Motion {
    pub fn new(initial: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            initial,
            target: initial,
            duration: Duration::from_millis(300),
            easing: Linear::ease_in_out,
            on_complete: None,
        }
    }

    pub fn to(mut self, target: f32) -> Self {
        self.target = target;
        self
    }

    pub fn animate(self, target: f32) -> Self {
        self.to(target)
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn on_complete(mut self, f: fn()) -> Self {
        self.on_complete = Some(f);
        self
    }
}

#[derive(Clone, Copy)]
pub struct UseMotion {
    id: Uuid,
    value: Signal<f32>,
    running_state: Signal<bool>,
    completion_state: Signal<AnimationState>,
    config: Motion,
    channel: Coroutine<()>,
}

impl UseMotion {
    pub fn value(&self) -> f32 {
        *self.value.read()
    }

    pub fn start(&mut self) {
        *self.running_state.write() = true;
        self.channel.send(());
    }
}

pub fn use_motion(config: Motion) -> UseMotion {
    let id = Uuid::new_v4();
    let mut value = use_signal(|| config.initial);
    let mut running_state = use_signal(|| false);
    let mut completion_state = use_signal(|| AnimationState::Idle);

    let channel = use_coroutine(move |mut rx| async move {
        while rx.next().await.is_some() {
            let start_time = Time::now();
            let start_value = *value.read();
            let end_value = config.target;

            running_state.set(true);

            while *running_state.read() {
                let elapsed = Time::now().duration_since(start_time);
                if elapsed >= config.duration {
                    break;
                }

                let progress = elapsed.as_secs_f32() / config.duration.as_secs_f32();
                let current = (config.easing)(progress, start_value, end_value - start_value, 1.0);

                value.set(current);

                Time::delay(Duration::from_millis(16)).await;
            }

            value.set(end_value);
            running_state.set(false);
            completion_state.set(AnimationState::Completed);

            if let Some(ref f) = config.on_complete {
                f();
            }
        }
    });

    UseMotion {
        id,
        value,
        running_state,
        completion_state,
        config,
        channel,
    }
}
