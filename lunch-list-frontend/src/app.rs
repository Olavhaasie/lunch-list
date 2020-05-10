use log::{error, info};
use yew::{
    agent::{Bridge, Bridged},
    format::{Json, Nothing},
    html,
    services::{
        fetch,
        fetch::{FetchService, FetchTask, Request},
    },
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::{agent::RouteRequest, prelude::*, switch::Permissive};

use crate::{api::AuthApi, login::LoginComponent, models::LoginResponse, routes::AppRoute};

type Response<T> = fetch::Response<Json<anyhow::Result<T>>>;

pub struct App {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
    token: Option<String>,
}

pub enum Msg {
    Fetch(Response<LoginResponse>),
    UpdateToken(String),
    NoOp,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::NoOp));

        let callback = link.callback(Msg::Fetch);
        let request = Request::get(AuthApi::Refresh.to_string())
            .body(Nothing)
            .unwrap();
        let fetch_task = FetchService::new().fetch(request, callback).ok();

        Self {
            link,
            router,
            fetch_task,
            token: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fetch(res) => {
                self.fetch_task = None;
                let (meta, Json(data)) = res.into_parts();
                info!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    match data {
                        Ok(data) => {
                            self.token = Some(data.token);
                        }
                        Err(e) => error!("Error when refreshing: {}", e),
                    }
                } else if meta.status.is_client_error() {
                    self.token = None;
                    self.router
                        .send(RouteRequest::ChangeRoute(AppRoute::Login.into()));
                    return true;
                } else {
                    error!("Got server error");
                }
            }
            Msg::UpdateToken(token) => self.token = Some(token),
            Msg::NoOp => (),
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(Msg::UpdateToken);
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{<LoginComponent login_callback=callback.clone()/>},
                        AppRoute::Dashboard => html!{"dashboard"},
                        AppRoute::List { id } => html!{ id },
                        AppRoute::User => html!{"user"},
                        AppRoute::NotFound(Permissive(r)) => html!{format!("Page not found {}", r.unwrap_or_default())},
                    }
                })

                redirect = Router::redirect(|r: Route| {
                    AppRoute::NotFound(Permissive(Some(r.route)))
                })
            />
        }
    }
}
