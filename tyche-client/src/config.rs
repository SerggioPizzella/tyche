pub fn auth_service() -> String {
    std::env::var("AUTH_SERVICE").unwrap()
}

pub fn character_service() -> String {
    std::env::var("CHARACTER_SERVICE").unwrap()
}
