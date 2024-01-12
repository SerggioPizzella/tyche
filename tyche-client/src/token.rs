use bevy::prelude::*;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnToken>()
            .add_systems(Update, handle_spawn_token);
    }
}

#[derive(Component, Clone)]
pub struct Token {
    pub name: Name,
}

impl Token {
    pub fn new(name: Name) -> Self {
        Self { name }
    }
}

#[derive(Bundle)]
struct TokenBundle {
    name: Name,
    token: Token,
    button: ButtonBundle,
}

#[derive(Event)]
pub struct SpawnToken(pub Token);

fn handle_spawn_token(
    mut ev_spawn_token: EventReader<SpawnToken>,
    tokens: Query<Entity, With<Token>>,
    mut commands: Commands,
) {
    for event in ev_spawn_token.read() {
        for token in &tokens {
            commands.entity(token).despawn_recursive();
        }

        spawn_token(&mut commands, &event.0);
    }
}

fn spawn_token(commands: &mut Commands, token: &Token) {
    commands
        .spawn(TokenBundle {
            token: token.clone(),
            name: token.name.clone(),
            button: ButtonBundle {
                background_color: Color::rgb(0.8, 0.15, 0.15).into(),
                style: Style {
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            },
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &token.name,
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        });
}
