use bevy::{prelude::*, utils::Uuid};
use bevy_renet::{client_connected, renet::{RenetClient, DefaultChannel}};
pub use tyche_host::*;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnToken>()
            .insert_resource(MyToken::default())
            .add_systems(Update, handle_spawn_token.run_if(client_connected()));
    }
}

#[derive(Default, Resource)]
pub struct MyToken(pub Option<Uuid>);

#[derive(Bundle)]
pub struct TokenBundle {
    pub name: Name,
    pub token: Token,
    pub sprite: SpriteBundle,
}

fn handle_spawn_token(
    mut commands: Commands,
    mut server: ResMut<RenetClient>,
    mut ev_spawn_token: EventReader<SpawnToken>,
) {
    for event in ev_spawn_token.read() {
        spawn_token(&mut commands, &event.0);
        let spawn_token = SpawnToken(event.0.clone());
        let message = bincode::serialize(&spawn_token).unwrap();
        server.send_message(DefaultChannel::ReliableOrdered, message);
    }
}

pub fn spawn_token(commands: &mut Commands, token: &Token) {
    commands
        .spawn(TokenBundle {
            token: token.clone(),
            name: token.name.clone().into(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: token.color,
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..Default::default()
                },
                ..Default::default()
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
