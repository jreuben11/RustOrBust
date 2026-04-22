use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

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

#[component]
pub fn UncontrolledComponent() -> impl IntoView {
    use leptos::html::Input;

    let (name, set_name) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().expect("<input> to exist").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
