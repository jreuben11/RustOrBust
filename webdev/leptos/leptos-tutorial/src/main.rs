use leptos::{ev::MouseEvent, ev::SubmitEvent, *};

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

    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    // Marks this as an optional prop. It will default to the default value of its type, i.e., 0.
    #[prop(default = 100)]
    /// The maximum value of the progress bar.
    max: u16,
    // Will run `.into()` on the value passed into the prop.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}

/// without ability to add or remove items.
#[component]
fn StaticList(
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    // create counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| create_signal(idx));

    // manipulate StaticList using ordinary Rust iterators, collect it into a Vec<_> to insert it into DOM
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

    // Note: if `counter_buttons` were a reactive list and its value changed, would be very inefficient:
    // it would rerender every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// allows you to add or remove items.
#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // <For/> is a keyed list. - each row has a defined key. If the key does not change, row will not be re-rendered. 
    // When list changes, only minimum number of changes will be made to the DOM.

    // `next_counter_id` -> generate unique IDs  by simply incrementing  each time we create a counter
    let mut next_counter_id = initial_length;

    // generate initial list as in <StaticList/>, but this time include ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // store initial list in a signal -> modify the list over time, adding / removing counters;  it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add counter to list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T` -> use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // <For/> component allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator - a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator and returns a view
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


#[component]
fn ControlledComponent() -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // functions the same way as event.target.value in JavaScript, but smooths out some of the typecasting
                set_name(event_target_value(&ev));
            }

            // `prop:` syntax - update a DOM property,rather than an attribute.
            //
            // IMPORTANT: `value` *attribute* only sets the initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; 
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // NodeRef stores a reference to the input element -  filled when element is created
    let input_element: NodeRef<Input> = create_node_ref();

    // fires when on form `submit` event - use to store value of the <input> in the signal
    let on_submit = move |ev: SubmitEvent| {
        // stop page from reloading
        ev.prevent_default();

        // extract value from the input
        let value = input_element()
            // event handlers can only fire after the view is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type ->
            // can call`HtmlInputElement::value()` to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                // use the `value` *attribute* to set only the initial value; browser maintains the state after that
                value=name

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Conditionals() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>"Control Flow"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        <h2><code>"Option<T>"</code></h2>
        // For any `T` that implements `IntoView`,  so does `Option<T>`

        <p>{odd_text}</p>
        // -> can use `Option` methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // can do dynamic conditional if-then-else logic in several ways
        //
        // a. An "if" expression in a function
        //    will simply re-render every time the value changes: good for lightweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling a class
        //    good for an element that's going to toggled often, as it doesn't destroy it in between states
        //    (you can find the `hidden` class in `index.html`)
        <p class:hidden=is_odd>"Appears if even."</p>

        // c. The <Show/> component
        //    only renders the fallback and the child once, lazily, and toggles between them when needed. 
        //    more efficient than a {move || if ...} block
        <Show when=is_odd
            fallback=|| view! { <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to  `Option`, can use it to create a show/hide toggled
        {move || is_odd().then(|| view! { <p>"Oddity!"</p> })}

        <h2>"Converting between Types"</h2>
        // e. Note: if branches return different types,can convert between them with:
        //    `.into_any()` (for different HTML element types)
        //    or `.into_view()` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => {
                // <pre> returns HtmlElement<Pre>
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
                // <p> returns HtmlElement<P>
                // so we convert into a more generic type
                view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }}
    }
}

#[component]
fn ErrorHandling() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
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
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn ParentChildComms() -> impl IntoView {
    // just some signals to toggle three classes on our <p>
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smallcaps) = create_signal(false);

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonC
    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
            <p
                // class: attributes take F: Fn() => bool, and these signals all implement Fn()
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

            // Button B: use a regular event listener
            // setting an event listener on a component like this applies it
            // to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // Button D gets its setter from context rather than props
            <ButtonD/>
        </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
    /// Signal that will be toggled when the button is clicked.
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

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
    /// Callback that will be invoked when the button is clicked.
    on_click: F,
) -> impl IntoView
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

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic
    // the component macro actually expands to define a
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever had named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle Italics"
        </button>
    }
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
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

// Often, you want to pass some kind of child view to another
// component. There are two basic patterns for doing this:
// - "render props": creating a component prop that takes a function
//   that creates a view
// - the `children` prop: a special property that contains content
//   passed as the children of a component in your view, not as a
//   property

#[component]
pub fn ComponentChildren() -> impl IntoView {
    let (items, set_items) = create_signal(vec![0, 1, 2]);
    let render_prop = move || {
        // items.with(...) reacts to the value without cloning
        // by applying a function. Here, we pass the `len` method
        // on a `Vec<_>` directly
        let len = move || items.with(Vec::len);
        view! {
            <p>"Length: " {len}</p>
        }
    };

    view! {
        // This component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand
            // `render_prop=render_prop` => `render_prop`
            // (this doesn't work for HTML element attributes)
            render_prop
        >
            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        // This component actually iterates over and wraps the children
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    /// this is an alias for `Box<dyn FnOnce() -> Fragment>`
    /// ... aren't you glad we named it `Children` instead?
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

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}