use web_sys::HtmlInputElement;
use yew::services::ConsoleService;
use yew::{html, html::NodeRef, Component, ComponentLink, Html, ShouldRender};

pub struct LoginComponent {
    link: ComponentLink<Self>,
    console: ConsoleService,
    name_input: NodeRef,
    password_input: NodeRef,
}

pub enum Msg {
    Login,
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            console: ConsoleService::new(),
            name_input: Default::default(),
            password_input: Default::default(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(input) = self.name_input.cast::<HtmlInputElement>() {
            input.focus().expect("Failed to focus input");
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                self.console.log("logging in");
                let name = self.name_input.cast::<HtmlInputElement>().unwrap();
                let password = self.password_input.cast::<HtmlInputElement>().unwrap();
                self.console.log(&name.value());
                self.console.log(&password.value());
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label for="username">{ "Username" }</label>
                <br></br>
                <input ref=self.name_input.clone() type="text" name="username"/>
                <br></br>
                <label for="password">{ "Password" }</label>
                <br></br>
                <input ref=self.password_input.clone() type="password" name="password"/>
                <br></br>
                <button onclick=self.link.callback(|_| Msg::Login)>
                    { "➡️" }
                </button>
            </div>
        }
    }
}
