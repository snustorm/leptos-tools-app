use ev::MouseEvent;
use leptos::*;

#[component]
pub fn Child(
    counter: ReadSignal<i16>,
    #[prop(into)] on_increment: Callback<MouseEvent>,
    #[prop(into)] on_decrement: Callback<MouseEvent>,
) -> impl IntoView {


    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px">
                <h3>"Child Callback"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=on_increment>
                        "Child Increment"
                    </button>
                    <button type="button" on:click=on_decrement>
                        "Child Decrement"
                    </button>
                </div>
            </div>
        </div>
    }

}


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
            <Child counter=counter on_increment=increment_counter on_decrement=decrement_counter />
            
        </div>
    }
}   