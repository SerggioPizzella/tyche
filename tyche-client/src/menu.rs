use bevy::{app::AppExit, prelude::*};
use reqwest::StatusCode;

use crate::{GameState, firebase::{self, FirebaseUser}};
pub struct MenuPlugin;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .insert_resource(User::default())
            .add_systems(OnEnter(GameState::Menu), spawn_ui)
            .add_systems(OnExit(GameState::Menu), delete_ui)
            .add_systems(Update, fetch_token.run_if(in_state(MenuState::LoggingIn)))
            .add_systems(Update, menu_action.run_if(in_state(MenuState::Main)));
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Main,
    LoggingIn,
    LoggedIn,
}

#[derive(Component)]
struct Title;

#[derive(Component)]
struct Menu;

#[derive(Component)]
enum ButtonAction {
    Login,
    Quit,
}

fn delete_ui(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    commands.entity(menu.single()).despawn_recursive();
}

fn spawn_ui(mut menu_state: ResMut<NextState<MenuState>>, mut commands: Commands) {
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
        .insert(Name::new("Main menu"))
        .insert(Menu)
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

    menu_state.set(MenuState::Main);
}

#[derive(Resource, Default)]
struct User {
    session: String,
}

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
    mut menu_state: ResMut<NextState<MenuState>>,
    mut user: ResMut<User>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            return;
        }

        match menu_button_action {
            ButtonAction::Quit => app_exit_events.send(AppExit),
            ButtonAction::Login => {
                let session = reqwest::blocking::get("http://localhost:3000/v1")
                    .unwrap()
                    .text()
                    .unwrap();

                let _ = open::that(format!("https://tyche-vtt.web.app/v1?session={}", session));
                user.session = session;
                menu_state.set(MenuState::LoggingIn);
            }
        }
    }
}

fn fetch_token(user: ResMut<User>, mut menu_state: ResMut<NextState<MenuState>>) {
    let request =
        reqwest::blocking::get(format!("http:/localhost:3000/v1/{}", user.session)).unwrap();

    println!("I made the request: {}", request.status());
    if request.status() == StatusCode::OK {
        let content = request.text().unwrap();
        println!("I made the request, got response: {}", &content);
        let fire_user: FirebaseUser = firebase::verify_id_token_with_project_id(&content).unwrap();
        println!("{}", fire_user.name.unwrap());
        menu_state.set(MenuState::LoggedIn);
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
