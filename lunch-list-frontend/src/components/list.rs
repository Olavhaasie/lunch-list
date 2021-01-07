use log::{error, info};
use yew::{
    agent::{Bridge, Bridged},
    format::{Json, Nothing},
    html,
    macros::Properties,
    services::fetch::{FetchService, FetchTask, Request},
    Component, ComponentLink, Html, ShouldRender,
};

use crate::{
    api::{ListApi, Response},
    models::ListResponse,
    TokenAgent, TokenRequest,
};

pub struct ListComponent {
    props: Props,
    list: Option<ListResponse>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    #[allow(dead_code)]
    token_agent: Box<dyn Bridge<TokenAgent>>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub id: usize,
}

pub enum Msg {
    Fetch(Response<ListResponse>),
    UpdateToken(String),
}

impl Component for ListComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut token_agent = TokenAgent::bridge(link.callback(Msg::UpdateToken));
        token_agent.send(TokenRequest::GetToken);
        Self {
            props,
            list: Default::default(),
            link,
            fetch_task: Default::default(),
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
                        Ok(list) => {
                            self.list = Some(list);
                            return true;
                        }
                        Err(e) => error!("Error when fetching lists: {}", e),
                    }
                } else {
                    error!("Error while fetching lists");
                }
            }
            Msg::UpdateToken(token) => {
                let callback = self.link.callback(Msg::Fetch);
                let request = Request::get(ListApi::Get(self.props.id).to_string())
                    .header("Authorization", format!("Bearer {}", &token))
                    .body(Nothing)
                    .unwrap();
                self.fetch_task = FetchService::fetch(request, callback).ok();
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(list) = &self.list {
            html! {
                <>
                <div class="list-header">
                    <h1>{ &list.list_type }</h1>
                    <h2>{ &list.date.format("%A, %-d %B, %C%y") }</h2>
                    <div class="attendance-content">{ list.users.len() }</div>
                </div>
                <div class="list-container">
                    { self.view_user_list(&list.users) }
                </div>
                </>
            }
        } else {
            html! {
                <div class="list-header">
                    <h1>{ "loading..." }</h1>
                </div>
            }
        }
    }
}

impl ListComponent {
    fn view_user_list(&self, users: &[String]) -> Html {
        if users.is_empty() {
            html! {
                <div class="empty-message">
                    { "It seems that no one has signed up yet" }
                </div>
            }
        } else {
            html! {
                <ul class="user-list">
                    { for users.iter().map(|u| self.view_user(u)) }
                </ul>
            }
        }
    }

    fn view_user(&self, name: &str) -> Html {
        html! {
            <li class="user-item">
                { name }
            </li>
        }
    }
}
