use self::login_page::LoginPage;
use bevy::{
    app::{App, Plugin},
    ecs::schedule::States,
};

mod login_page;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Page>().add_plugins(LoginPage);
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum Page {
    #[default]
    Login,
}
