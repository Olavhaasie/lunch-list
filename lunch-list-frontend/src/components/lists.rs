use log::{error, info};
use yew::{
    agent::{Bridge, Bridged},
    format::{Json, Nothing},
    html,
    services::fetch::{FetchService, FetchTask, Request},
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::components::RouterAnchor;

use crate::{
    api::{ListApi, Response},
    models::{List, ListsResponse},
    routes::AppRoute,
    TokenAgent, TokenRequest,
};

pub struct ListsComponent {
    lists: Vec<List>,
    link: ComponentLink<Self>,
    fetch: FetchService,
    fetch_task: Option<FetchTask>,
    token: String,
    #[allow(dead_code)]
    token_agent: Box<dyn Bridge<TokenAgent>>,
}

pub enum Msg {
    Fetch(Response<ListsResponse>),
    UpdateToken(String),
}

impl Component for ListsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut token_agent = TokenAgent::bridge(link.callback(Msg::UpdateToken));
        token_agent.send(TokenRequest::GetToken);
        Self {
            lists: Default::default(),
            link,
            fetch: FetchService::new(),
            fetch_task: Default::default(),
            token: "".to_string(),
            token_agent,
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
                            self.lists = data.lists;
                            return true;
                        }
                        Err(e) => error!("Error when fetching lists: {}", e),
                    }
                } else {
                    error!("Error while fetching lists");
                }
            }
            Msg::UpdateToken(token) => {
                self.token = token;
                let callback = self.link.callback(Msg::Fetch);
                let request = Request::get(ListApi::GetAll.to_string())
                    .header("Authorization", format!("Bearer {}", &self.token))
                    .body(Nothing)
                    .unwrap();
                self.fetch_task = self.fetch.fetch(request, callback).ok();
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="list-header">
                <h1>{ "lunch-list" }</h1>
            </div>
            <div class="list-container">
                <ul class="list-list">
                    { for self.lists.iter().map(|l| self.view_list(l)) }
                </ul>
            </div>
            </>
        }
    }
}

impl ListsComponent {
    fn view_list(&self, list: &List) -> Html {
        let class = match list.list_type.as_str() {
            "lunch" => "lunch-item",
            "dinner" => "dinner-item",
            _ => "",
        };
        html! {
            <RouterAnchor<AppRoute> classes="list-anchor" route=AppRoute::List { id: list.id }>
                <li class=("list-item", class)>
                    <div class="date-content">
                        { &list.date }
                    </div>
                    <div class="attendance-content">
                        { 0 }
                    </div>
                </li>
            </RouterAnchor<AppRoute>>
        }
    }
}
