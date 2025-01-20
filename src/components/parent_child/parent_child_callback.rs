use leptos::*;

#[component]
pub fn ParentCallBack() -> impl IntoView {
    
    
    let (counter, set_counter) = create_signal::<i16>(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);    
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);    

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px">
                <h3>"Parent Callback"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>
                        "Parent Increment"
                    </button>
                    <button type="button" on:click=decrement_counter>
                        "Parent Decrement"
                    </button>
                </div>
            </div>
        </div>
    }
}   