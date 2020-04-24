use yew_router::Switch;

#[derive(Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/lists"]
    Lists,
    #[to = "/list/{id}"]
    List { id: usize },
    #[to = "/user"]
    User,
}
