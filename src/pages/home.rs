use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <body>
          <section class="text-gray-700 body-font">
          <div class="container mx-auto flex px-5 py-24 md:flex-row flex-col items-center">
            <div class="lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center">
              <h1 class="title-font sm:text-4xl text-3xl mb-4 font-medium text-gray-900">{"Hello!"}
              </h1>
              <p class="mb-8 leading-relaxed">{"My name is Jake, and I am a backend developer currently employed as a Software Engineer at "}<a class="text-gray-500 dark:text-gray-400" href="https://www.jpmorgan.com/global">{"@JPMorgan"}</a>{". When I’m not thinking about work or my personal projects I'm likely on a big cycle. Over the years I’ve taken an interest in Full Stack Engineering, Cyber Security and building out fun projects!"}</p>
              <div class="flex justify-center">
                <button class="inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg">{"About"}</button>
                <Link<Route> classes={classes!("ml-4", "inline-flex", "text-gray-700", "bg-gray-200", "border-0", "py-2", "px-6", "focus:outline-none", "hover:bg-gray-300", "rounded", "text-lg")} to={Route::Work}>
                { "Work" }
                </Link<Route>>
              </div>
            </div>
            <div class="lg:max-w-lg lg:w-full md:w-1/2 w-5/6">
              <img class="object-cover object-center rounded" alt="Jake!" src="public/jake.jpeg"/>
            </div>
          </div>
        </section>
          </body>
          }
    }
}
