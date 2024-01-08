use std::env;

use bevy::{app::AppExit, prelude::*};
use reqwest::StatusCode;

use crate::{
    auth_service,
    firebase::{self, FirebaseUser},
    user::User,
    GameState,
};

use super::Page;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct LoginPage;

impl Plugin for LoginPage {
    fn build(&self, app: &mut App) {
        app.add_state::<LoginState>()
            .insert_resource(Session::default())
            .add_systems(OnEnter(Page::Login), spawn_ui)
            .add_systems(OnExit(GameState::Menu), delete_ui)
            .add_systems(Update, menu_action.run_if(in_state(LoginState::Main)))
            .add_systems(Update, fetch_token.run_if(in_state(LoginState::LoggingIn)))
            .add_systems(OnEnter(LoginState::LoggedIn), login_complete);
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum LoginState {
    #[default]
    Main,
    LoggingIn,
    LoggedIn,
}

#[derive(Component)]
struct Title;

#[derive(Component)]
enum ButtonAction {
    Login,
    Quit,
}
#[derive(Component)]
struct Root;

fn delete_ui(mut commands: Commands, menu: Query<Entity, With<Root>>) {
    commands.entity(menu.single()).despawn_recursive();
}

fn login_complete(mut menu_state: ResMut<NextState<GameState>>) {
    menu_state.set(GameState::Main)
}

fn spawn_ui(mut menu_state: ResMut<NextState<LoginState>>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(Root)
        .insert(Name::new("Login Page"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        TextBundle::from_section(
                            "Bevy Game Menu UI",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                        Title,
                    ));

                    spawn_button(parent, ButtonAction::Login, "Login");
                    spawn_button(parent, ButtonAction::Quit, "Quit");
                });
        });

    menu_state.set(LoginState::Main);
}

#[derive(Resource, Default)]
struct Session(String);

fn spawn_button(parent: &mut ChildBuilder, menu_action: ButtonAction, text: impl Into<String>) {
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    parent
        .spawn(MenuButtonBundle {
            menu_action,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(text, button_text_style.clone()));
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<LoginState>>,
    mut session: ResMut<Session>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            return;
        }

        match menu_button_action {
            ButtonAction::Quit => app_exit_events.send(AppExit),
            ButtonAction::Login => {
                let content = reqwest::blocking::get(auth_service!())
                    .unwrap()
                    .text()
                    .unwrap();

                session.0 = content;
                let _ = open::that(format!(
                    "https://tyche-vtt.web.app/?session={}&mode=local",
                    session.0
                ));
                menu_state.set(LoginState::LoggingIn);
            }
        }
    }
}

fn fetch_token(
    session: ResMut<Session>,
    mut user: ResMut<User>,
    mut menu_state: ResMut<NextState<LoginState>>,
) {
    let request = reqwest::blocking::get(format!("{}/{}", auth_service!(), session.0)).unwrap();

    if request.status() == StatusCode::OK {
        let content = request.text().unwrap();
        let fire_user = firebase::verify_id_token_with_project_id(&content).unwrap();
        menu_state.set(LoginState::LoggedIn);
        user.name = fire_user.name.unwrap();
        user.token = content;
    }
}

#[derive(Bundle)]
struct MenuButtonBundle {
    button_bundle: ButtonBundle,
    menu_action: ButtonAction,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        Self {
            button_bundle: ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            menu_action: ButtonAction::Quit,
        }
    }
}
