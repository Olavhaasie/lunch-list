use yew_router::Switch;

#[derive(Debug, Clone, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/list/{id}"]
    List { id: usize },
    #[to = "/user"]
    User,
}
