use yew::prelude::*;

const BASE_URL: &str = "https://jredrupp.github.io";

struct Project {
    name: String,
    path: String,
}

#[function_component]
fn App() -> Html {
    let projects = vec![
        Project {
            name: "P5 Menger Sponge".to_string(),
            path: "./p5_menger_sponge_fractal/".to_string(),
        },
        Project {
            name: "Example Yew WebAssembly App".to_string(),
            path: "./yew-example/".to_string(),
        },
    ];

    html! {
        <body>
            {projects.iter().map(|project| {
                html! {
                    <div>
                        <a href={format!("{}{}", BASE_URL, project.path)}>{project.name.clone()}</a>
                    </div>
                }
            }).collect::<Html>()}
        </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
