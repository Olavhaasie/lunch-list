use anyhow::Error;
use log::{error, info};
use web_sys::HtmlInputElement;
use yew::{
    agent::{Bridge, Bridged},
    format::Json,
    html,
    html::NodeRef,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{Area, StorageService},
    },
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::{agent::RouteRequest, prelude::*};

use crate::models::{LoginRequest, LoginResponse};
use crate::routes::AppRoute;

pub struct LoginComponent {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    storage: StorageService,
    fetch: FetchService,
    fetch_task: Option<FetchTask>,
    name_input: NodeRef,
    password_input: NodeRef,
}

pub enum Msg {
    LoginTask,
    LoginReady(Result<LoginResponse, Error>),
    LoginFailed,
    ServerError,
    NoOp,
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
        let request = Request::post("http://localhost:8080/api/auth/login")
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
        let callback = link.callback(|_| Msg::NoOp);
        let router = RouteAgent::bridge(callback);
        Self {
            link,
            router,
            storage: StorageService::new(Area::Local).unwrap(),
            fetch: FetchService::new(),
            fetch_task: Default::default(),
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
                    Ok(res) => self.storage.store("token", Ok(res.token)),
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
            Msg::NoOp => (),
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="login">
                <label class="login-label" for="username">{ "Username" }</label>
                <br></br>
                <input class="input" ref=self.name_input.clone() type="text" name="username"/>
                <br></br>
                <label class="login-label" for="password">{ "Password" }</label>
                <br></br>
                <input class="input" ref=self.password_input.clone() type="password" name="password"/>
                <br></br>
                <button class ="login-button" onclick=self.link.callback(|_| Msg::LoginTask)>
                    { "Login" }
                </button>
            </div>
        }
    }
}
