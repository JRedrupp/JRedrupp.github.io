use yew::prelude::*;

const BASE_URL: &str = "https://jredrupp.github.io";

struct Project {
    title: String,
    path: String,
    description: String,
}

pub struct Work;
impl Component for Work {
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
            Project {
                title: "Example Yew WebAssembly App".to_string(),
                path: "./yew-example/".to_string(),
                description: "This is an example Yew WebAssembly app".to_string(),
            },
            Project {
                title: "Example Yew WebAssembly App".to_string(),
                path: "./yew-example/".to_string(),
                description: "This is an example Yew WebAssembly app".to_string(),
            },
        ];

        html! {
                        <body>
                            <div class={classes!("flex", "flex-col", "w-full")}>
                            <h1>{"Projects"}</h1>
                            // Columns of projects that will reduce in comumns if not enough size
                            <div class={classes!("flex", "flex-row", "flex-wrap", "justify-center", "gap-3")}>
                                {for projects.iter().map(|project| html! {
                                    <div class={classes!("max-w-sm", "p-6", "bg-white","border", "border-gray-200", "rounded-lg", "shadow", "dark:bg-gray-800", "dark:border-gray-700")}>
                                        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{project.title.clone()}</h5>
                                        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{project.description.clone()}</p>
                                        <div class="flex space-x-4 w-16">
                                        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"></path>
                </svg>

                <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"></path>
        </svg></div>


                                    </div>
                                })} </div> </div>

                                /*
                                <div class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
                                    <a href="#">
                                        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Noteworthy technology acquisitions 2021</h5>
                                    </a>
                                    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.</p>
                                    <a href="#" class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                        Read more
                                        <svg aria-hidden="true" class="w-4 h-4 ml-2 -mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                                    </a>
                                </div>

                                 */
                        </body>
                        }
    }
}
