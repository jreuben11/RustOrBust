use leptos::*;
mod components;
use components::progressbar::ProgressBar;
use components::lists::{StaticList, DynamicList};
use components::control::{ControlledComponent, UncontrolledComponent};
use components::conditionals::Conditionals;
use components::errorhandling::ErrorHandling;
use components::parent_child_comms::ParentChildComms;
use components::child_components::ComponentChildren;
use components::use_effect::UseEffect;

// `main` function just mounts component to the <body> - defined it as `fn App`, use it in a template as <App/>
fn main() {
    console_error_panic_hook::set_once();
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    leptos::mount_to_body(|| view! { <App/> })
}


// #[component] macro marks a function as a reusable component
#[component]
fn App() -> impl IntoView {
    // create a reactive signal and get a (getter, setter) pair
    let (count, set_count) = create_signal(0);
    //"derived signal": a function that accesses other signals -  use to create reactive values that depend on values of one or more other signals
    let double_count = move || count() * 2;

    // `view` macro defines UI
    view! {
        <button
            // every event handler is defined as `on:{eventname}`
            // move `set_count` into the closure - signals are Copy and 'static
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            // the class: syntax reactively updates a single class
            class:red=move || count() % 2 == 1
        >
            // text nodes in RSX should be wrapped in quotes,like a normal Rust string
            "Click me"
        </button>
        
        <p>
            <strong>"Reactive: "</strong>
            // insert Rust expressions as values in the DOM by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: this will *not* be reactive - it simply gets the value of count once
            {count()}
        </p>

        // NOTE: self-closing tags like <br> need an explicit /
        <br/>
       
        <ProgressBar max=50 progress=count/>
        // use the default max value is 100
        <ProgressBar progress=count/>
        // Signal::derive creates a Signal wrapper from a derived signal
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
