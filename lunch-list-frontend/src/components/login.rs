use anyhow::Error;
use log::{error, info};
use yew::{
    agent::{Dispatched, Dispatcher},
    format::Json,
    html,
    html::NodeRef,
    services::fetch::{FetchService, FetchTask, Request, Response},
    web_sys::{Event, HtmlInputElement},
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::{agent::RouteRequest, prelude::*};

use crate::{
    api::AuthApi,
    models::{LoginRequest, LoginResponse},
    routes::AppRoute,
    TokenAgent, TokenRequest,
};

pub struct LoginComponent {
    link: ComponentLink<Self>,
    router: Dispatcher<RouteAgent>,
    fetch: FetchService,
    fetch_task: Option<FetchTask>,
    token: Dispatcher<TokenAgent>,
    name_input: NodeRef,
    password_input: NodeRef,
}

pub enum Msg {
    LoginTask,
    LoginReady(Result<LoginResponse, Error>),
    LoginFailed,
    ServerError,
}

impl LoginComponent {
    fn fetch_login(&mut self, req: LoginRequest) -> FetchTask {
        let callback =
            self.link
                .callback(move |res: Response<Json<Result<LoginResponse, Error>>>| {
                    let (meta, Json(data)) = res.into_parts();
                    info!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::LoginReady(data)
                    } else if meta.status.is_client_error() {
                        Msg::LoginFailed
                    } else {
                        Msg::ServerError
                    }
                });
        let request = Request::post(AuthApi::Login.to_string())
            .header("content-type", "application/json")
            .body(Json(&req))
            .unwrap();
        self.fetch.fetch(request, callback).unwrap()
    }
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            router: RouteAgent::dispatcher(),
            fetch: FetchService::new(),
            fetch_task: Default::default(),
            token: TokenAgent::dispatcher(),
            name_input: Default::default(),
            password_input: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoginTask => {
                log::info!("logging in");
                let username = self.name_input.cast::<HtmlInputElement>().unwrap();
                let password = self.password_input.cast::<HtmlInputElement>().unwrap();
                let task = self.fetch_login(LoginRequest {
                    username: username.value(),
                    password: password.value(),
                });
                self.fetch_task = Some(task);
            }
            Msg::LoginReady(result) => {
                match result {
                    Ok(data) => self.token.send(TokenRequest::UpdateToken(data.token)),
                    Err(e) => error!("Error when logging in: {}", e),
                }
                let route = Route::from(AppRoute::Dashboard);
                self.router.send(RouteRequest::ChangeRoute(route));
                return true;
            }
            Msg::LoginFailed => {
                error!("Login failed");
            }
            Msg::ServerError => error!("Server error"),
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="login">
                <form onsubmit=self.link.callback(|e: Event| { e.prevent_default(); Msg::LoginTask })>
                    <label class="input-label" for="un">{ "Username" }</label>
                    <br></br>
                    <input class="input" ref=self.name_input.clone() type="text" id ="un" name="username" pattern="[a-zA-Z0-9][a-zA-Z0-9 ]*" autofocus=true required=true/>
                    <br></br>
                    <label class="input-label" for="pw">{ "Password" }</label>
                    <br></br>
                    <input class="input" ref=self.password_input.clone() type="password" id="pw" name="password" required=true/>
                    <br></br>
                    <input class="login-button" type="submit" value="Login"/>
                </form>
            </div>
        }
    }
}
