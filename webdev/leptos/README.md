# leptos client tutorial

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
cargo install leptosfmt
cargo install cargo-generate
# cargo generate --git https://github.com/leptos-community/start-csr
trunk serve --port 3000 --open 
```

- [Cargo.toml](leptos-tutorial/Cargo.toml)
- [main.rs](leptos-tutorial/src/main.rs)

## Basic Component
```rust
use leptos::*;

// `main` function just mounts component to the <body> - defined it as `fn App`, use it in a template as <App/>
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

## view: Dynamic Classes, Styles and Attributes
```rust
leptos::mount_to_body(|| view! { <App/> })

...
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button 
            on:click=move |_| { set_count.update(|n| *n += 1); } 
            class:red=move || count() % 2 == 1 
        >
            "Click me"
        </button>
        <p>
            <strong>"Reactive: "</strong>
            {move || count()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <br/>
    }
}
```

## components and props
```rust
let double_count = move || count() * 2;
<ProgressBar max=50 progress=count/>
<ProgressBar progress=count/>
<ProgressBar max=50 progress=Signal::derive(double_count)/>
<p>"Count: " {count}</p>
<p>"Double Count: " {double_count}</p>

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max={max} value=progress />
        <br/>
    }
}
```

## Static Iteration
```rust
<StaticList length=5/>

#[component]
fn StaticList( length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <ul>{counter_buttons}</ul>
    }
}
```

## Dynamic Iteration
```rust
<DynamicList initial_length=5/>

#[component]
fn DynamicList(initial_length: usize,) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(initial_counters);
    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
```

## controlled component
```rust
#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}
```

## uncontrolled component
```rust
use leptos::{ev::SubmitEvent, *};

#[component]
fn UncontrolledComponent() -> impl IntoView {
    use leptos::html::Input;
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
```

## conditionals
```rust
#[component]
fn Conditionals() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>"Control Flow"</h1>
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>
        <hr/>

        <h2><code>"Option<T>"</code></h2>
        <p>{odd_text}</p>
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // a. An "if" expression in a function
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>
        // b. Toggling a class
        <p class:hidden=is_odd>"Appears if even."</p>
        // c. The <Show/> component
        <Show when=is_odd
            fallback=|| view! { <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>
        // d. `bool::then()` 
        {move || is_odd().then(|| view! { <p>"Oddity!"</p> })}
        <h2>"Converting between Types"</h2>
        // e. convert branches that return different return types
        {move || match is_odd() {
            true if value() == 1 => {
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
                view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }}
    }
}
```

## error handling
```rust
#[component]
fn ErrorHandling() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}
```

## parent-child communication
```rust
#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn ParentChildComms() -> impl IntoView {
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smallcaps) = create_signal(false);

    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
            <p
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>
            // Button A: pass the signal setter
            <ButtonA setter=set_red/>
            // Button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>
            // Button C: use a regular event listener - applies it to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>
            // Button D gets its setter from context rather than props
            <ButtonD/>
        </main>
    }
}

#[component]
pub fn ButtonA(
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Red"
        </button>
    }
}

#[component]
pub fn ButtonB<F>( on_click: F,) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            on:click=on_click
        >
            "Toggle Right"
        </button>
    }
}

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle Italics"
        </button>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Small Caps"
        </button>
    }
}
```
## Child Components
```rust
#[component]
pub fn ComponentChildren() -> impl IntoView {
    let (items, set_items) = create_signal(vec![0, 1, 2]);
    let render_prop = move || {
        let len = move || items.with(Vec::len);
        view! {
            <p>"Length: " {len}</p>
        }
    };

    view! {
        <TakesChildren
            render_prop
        >
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    render_prop: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        <ul>{children}</ul>
    }
}

```