#[derive(Debug, Deserialize, Validate)]
pub struct Login {
    #[serde(deserialize_with = "deserialize_username")]
    #[validate(
        length(min = 1, message = "Username cannot be empty"),
        custom = "validate_username"
    )]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}
