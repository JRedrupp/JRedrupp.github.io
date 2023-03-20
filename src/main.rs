use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::home::Home;
use pages::work::Work;
use yew::html::Scope;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/work")]
    Work,
    // #[not_found]
    // #[at("/404")]
    // Home,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "© Jake C Redrupp" }
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}
impl App {
    // https://github.com/yewstack/yew/blob/master/examples/router/src/main.rs
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active {
            "hidden"
        } else {
            "is-active"
        };

        html! {

        <nav class="bg-gray-100">
          <div class="max-w-6xl mx-auto px-4">
            <div class="flex justify-between">

              <div class="flex space-x-4">
                <div>
                  <a href="#" class="flex items-center py-5 px-2 text-gray-700 hover:text-gray-900">
                    <svg class="h-6 w-6 mr-1 text-blue-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                    </svg>
                    <Link<Route> classes={classes!("font-bold")} to={Route::Home}>
                        { "Jake Redrupp" }
                    </Link<Route>>
                  </a>
                </div>
                <div class="hidden md:flex items-center space-x-1">
                    <Link<Route> classes={classes!("py-5", "px-3", "text-gray-700", "hover:text-gray-900")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("py-5", "px-3", "text-gray-700", "hover:text-gray-900")} to={Route::Work}>
                    { "Work" }
                    </Link<Route>>
                </div>
              </div>

              <div class="md:hidden flex items-center">
                <button class="mobile-menu-button" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                  <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                  </svg>
                </button>
              </div>

            </div>
          </div>

          <div classes={classes!("mobile-menu", active_class )}>
                <div>
                <Link<Route> classes={classes!("block", "py-2", "px-4", "text-sm", "hover:bg-gray-200", active_class)} to={Route::Home}>
                    { "Home" }
                </Link<Route>>
                </div>
                <div>
                <Link<Route> classes={classes!("block", "py-2", "px-4", "text-sm", "hover:bg-gray-200", active_class)} to={Route::Work}>
                    { "Work" }
                </Link<Route>>
                </div>
          </div>
        </nav>
                }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Work => {
            html! { <Work /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
