use dioxus::prelude::*;
use dioxus_animations::{use_motion, Motion};
use std::time::Duration;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut motion = use_motion(
        Motion::new(0.0)
            .to(100.0)
            .duration(Duration::from_millis(1000))
            .on_complete(|| println!("Animation complete!")),
    );

    let mut opacity_motion = use_motion(
        Motion::new(0.0)
            .to(1.0)
            .duration(Duration::from_millis(800))
            .on_complete(|| println!("Opacity animation complete!")),
    );

    let mut y_motion = use_motion(
        Motion::new(100.0)
            .to(0.0)
            .duration(Duration::from_millis(600)),
    );

    use_effect(move || {
        opacity_motion.start();
        y_motion.start();
    });

    rsx! {
        div {
            class: "container",
            style: "display: flex; flex-direction: column; gap: 20px; padding: 20px;",
            // Width animation
            div { style: "text-align: center",
                button {
                    onclick: move |_| motion.start(),
                    style: "padding: 10px 20px; border-radius: 4px; background: #4CAF50; color: white; border: none;",
                    "Animate Width!"
                }
            }
            // Width animation div
            div {
                width: "{motion.value()}%",
                height: "50px",
                background: "linear-gradient(90deg, #ff0000, #00ff00)",
                border_radius: "8px",
            }
            // Opacity animation div
            div {
                height: "50px",
                opacity: "{opacity_motion.value()}",
                background: "linear-gradient(90deg, #2196F3, #9C27B0)",
                border_radius: "8px",
            }
            // Transform animation div
            div {
                height: "50px",
                transform: "translateY({y_motion.value()}px)",
                background: "linear-gradient(90deg, #FF9800, #F44336)",
                border_radius: "8px",
            }
        }
    }
}
