use std::time::Duration;

use dioxus::prelude::*;
use dioxus_animations::{use_motion, Motion};

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut motion = use_motion(
        Motion::new(0.0)
            .animate(100.0)
            .duration(Duration::from_millis(1000))
            .on_complete(|| println!("Animation complete!")),
    );

    rsx!(
        button { onclick: move |_| motion.start(), "Animate!" }
        div {
            width: "{motion.value()}%",
            height: "100px",
            background: "linear-gradient(90deg, #ff0000, #00ff00)",
        }
    )
}
