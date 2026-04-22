use leptos::prelude::*;
mod components;
use components::child_components::ComponentChildren;
use components::conditionals::Conditionals;
use components::control::{ControlledComponent, UncontrolledComponent};
use components::errorhandling::ErrorHandling;
use components::lists::{DynamicList, StaticList};
use components::parent_child_comms::ParentChildComms;
use components::progressbar::ProgressBar;
use components::use_effect::UseEffect;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me"
        </button>

        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count.get_untracked()}
        </p>

        <br/>

        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>

        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>

        <h1>"controlled / uncontrolled components"</h1>
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>

        <h1>"conditionals"</h1>
        <Conditionals/>

        <h1>"Error Handling"</h1>
        <ErrorHandling />

        <ParentChildComms />
        <ComponentChildren />
        <UseEffect />
    }
}
