use leptos::prelude::*;

#[component]
pub fn ComponentChildren() -> impl IntoView {
    let (items, _set_items) = signal(vec![0, 1, 2]);
    let render_prop = move || {
        let len = move || items.with(Vec::len);
        view! {
            <p>"Length: " {len}</p>
        }
    };

    view! {
        <TakesChildren render_prop>
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

/// Displays a `render_prop` and some children within markup.
#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
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

/// Wraps children in a `<ul>`.
#[component]
fn WrapsChildren(children: Children) -> impl IntoView {
    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        <ul>{children()}</ul>
    }
}
