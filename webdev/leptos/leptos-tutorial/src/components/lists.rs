use leptos::*;


/// without ability to add or remove items.
#[component]
pub fn StaticList(
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
pub fn DynamicList(
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