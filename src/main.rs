use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1>{ "RustConf Explorer"}</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <p>{"Jogn Doe: Building and breadking things"}</p>
            <p>{"Jane Smith: The development process"}</p>
            <p>{"Tom Jerry: Mouseless development"}</p>
        </div>
        <div>
            <h3>{"John Doe : Building and breadking things"}</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
