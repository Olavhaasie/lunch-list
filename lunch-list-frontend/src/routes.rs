use yew_router::{switch::Permissive, Switch};

#[derive(Debug, Clone, Switch)]
pub enum AppRoute {
    #[to = "/login!"]
    Login,
    #[to = "/dashboard!"]
    Dashboard,
    #[to = "/list/{id}!"]
    List { id: usize },
    #[to = "/user!"]
    User,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}
