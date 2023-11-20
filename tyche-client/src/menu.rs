use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

// All actions that can be triggered from a button click
#[derive(Component)]
enum ButtonAction {
    Login,
    Quit,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(Startup, start_menu)
            .add_systems(Update, menu_action.run_if(in_state(MenuState::Main)))
            .add_systems(OnEnter(MenuState::Main), spawn_ui);
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Disabled,
    Main,
}

fn start_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn spawn_ui(mut commands: Commands) {
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
                    parent.spawn(
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
                    );

                    spawn_button(parent, ButtonAction::Login, "Login");
                    spawn_button(parent, ButtonAction::Quit, "Quit");
                });
        });
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
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            return;
        }

        match menu_button_action {
            ButtonAction::Quit => app_exit_events.send(AppExit),
            ButtonAction::Login => menu_state.set(MenuState::Disabled),
        }
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
