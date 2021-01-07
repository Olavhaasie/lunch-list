use std::time::Duration;

use log::{error, info};
use yew::{
    agent::{Bridge, Bridged},
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchService, FetchTask, Request},
        interval::{IntervalService, IntervalTask},
    },
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
    router::Router,
    switch::Permissive,
};

use crate::{
    api::{AuthApi, Response},
    components::{ListComponent, ListsComponent, LoginComponent},
    models::LoginResponse,
    routes::AppRoute,
    TokenAgent, TokenRequest,
};

const REFRESH_INTERVAL_SECS: u64 = 8 * 60;

pub struct App {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
    token: Box<dyn Bridge<TokenAgent>>,
    #[allow(dead_code)]
    interval_task: IntervalTask,
}

pub enum Msg {
    Fetch(Response<LoginResponse>),
    RefreshToken,
    NoOp,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::NoOp));
        let token = TokenAgent::bridge(link.callback(|_| Msg::NoOp));

        let callback = link.callback(Msg::Fetch);
        let request = Request::get(AuthApi::Refresh.to_string())
            .body(Nothing)
            .unwrap();
        let fetch_task = FetchService::fetch(request, callback).ok();

        let callback = link.callback(|_| Msg::RefreshToken);
        let duration = Duration::from_secs(REFRESH_INTERVAL_SECS);
        let interval_task = IntervalService::spawn(duration, callback);

        Self {
            link,
            router,
            fetch_task,
            token,
            interval_task,
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
                            self.token.send(TokenRequest::UpdateToken(data.token));
                        }
                        Err(e) => error!("Error when refreshing: {}", e),
                    }
                } else if meta.status.is_client_error() {
                    self.router
                        .send(RouteRequest::ChangeRoute(AppRoute::Login.into()));
                    return true;
                } else {
                    error!("Got server error");
                }
            }
            Msg::RefreshToken => {
                let callback = self.link.callback(Msg::Fetch);
                let request = Request::get(AuthApi::Refresh.to_string())
                    .body(Nothing)
                    .unwrap();
                self.fetch_task = FetchService::fetch(request, callback).ok();
            }
            Msg::NoOp => (),
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="content">
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{<LoginComponent/>},
                        AppRoute::Dashboard => html!{<ListsComponent/>},
                        AppRoute::List { id } => html!{<ListComponent id=id/>},
                        AppRoute::User => html!{"user"},
                        AppRoute::NotFound(Permissive(r)) => html!{format!("Page not found {}", r.unwrap_or_default())},
                        _ => html!{"loading..."},
                    }
                })

                redirect = Router::redirect(|r: Route| {
                    AppRoute::NotFound(Permissive(Some(r.route)))
                })
            />
            </div>
            <footer class="footer">
                <p>{ format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) }</p>
                <a href=env!("CARGO_PKG_REPOSITORY") target="_blank">
                    <img src="/github-mark.png"/>
                </a>
            </footer>
            </>
        }
    }
}
