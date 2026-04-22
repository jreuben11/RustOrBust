use leptos::prelude::*;

/// Shows progress toward a goal.
#[component]
pub fn ProgressBar(
    #[prop(default = 100)]
    /// The maximum value of the progress bar.
    max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}
