use yew::{html, html::Component, ComponentLink, Html, ShouldRender};
use yew_router::router::Router;

use crate::{login::LoginComponent, routes::AppRoute};

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        log::info!("App is being created");
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{<LoginComponent/>},
                        AppRoute::Lists => html!{"switch"},
                        AppRoute::List { id } => html!{ id },
                        AppRoute::User => html!{"user"},
                    }
                })

                redirect = Router::redirect(|_| {
                    AppRoute::Login
                })
            />
        }
    }
}
