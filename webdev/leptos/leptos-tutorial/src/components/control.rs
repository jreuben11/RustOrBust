use leptos::{ev::SubmitEvent, *};

#[component]
pub fn ControlledComponent() -> impl IntoView {
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
pub fn UncontrolledComponent() -> impl IntoView {
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

