use yew::prelude::*;

const BASE_URL: &str = "https://jredrupp.github.io";

struct Project {
    title: String,
    path: String,
    description: String,
}

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let projects = vec![
            Project {
                title: "P5 Menger Sponge".to_string(),
                path: "./p5_menger_sponge_fractal/".to_string(),
                description: "This is an example P5.js app".to_string(),
            },
            Project {
                title: "Example Yew WebAssembly App".to_string(),
                path: "./yew-example/".to_string(),
                description: "This is an example Yew WebAssembly app".to_string(),
            },
        ];

        html! {
        <body>
            <div class={classes!("flex", "justify-center")}>
            <div class={classes!("w-96")}>
            <h1>{"Projects"}</h1>
                {projects.iter().map(|project| {
                    html! {
                        <div>
                        <a href={format!("{}{}", BASE_URL, project.path)} class="flex flex-col items-center bg-white border border-gray-200 rounded-lg shadow md:flex-row md:max-w-xl hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
                        <div class="flex flex-col justify-between p-4 leading-normal">
                        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{project.title.clone()}</h5>
                        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{project.description.clone()}</p>
                        </div>
                        </a>
                        </div>
                    }
                }).collect::<Html>()}
            </div></div>
        </body>
        }
    }
}
