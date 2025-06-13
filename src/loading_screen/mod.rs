use std::collections::HashMap;

use bevy::prelude::*;

use crate::{constants::{loading_screen::{BAR_SIZE, INNER_BAR_SIZE}, HEIGHT, WIDTH}, dice::Action, states::GameState};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<AssetStore>()
      .add_systems(OnEnter(GameState::Loading), (populate_asset_store, spawn_loading_screen))
      .add_systems(Update, update_progress.run_if(in_state(GameState::Loading)))
      .add_systems(OnExit(GameState::Loading), despawn_loading_screen);
  }
}

#[derive(Resource, Default)]
pub struct AssetStore {
  assets: HashMap<&'static str, UntypedHandle>,
  assets_to_load: Vec<UntypedHandle>,
  already_loaded: usize,
}

impl AssetStore {
  pub fn get<A: Asset>(&self, name: &'static str) -> Handle<A> {
    self.assets.get(name).unwrap().clone().typed_unchecked::<A>()
  }

  fn load(&mut self, name: &'static str, handle: UntypedHandle) {
    self.assets.insert(name, handle.clone());
    self.assets_to_load.push(handle);
  }

  fn progress(&mut self, server: AssetServer) -> f32 {
    self.assets_to_load.retain(|handle| {
      let (retain, drop) = (true, false);
      if server.is_loaded_with_dependencies(handle) {
        self.already_loaded += 1;
        drop
      } else {
        retain
      }
    });

    (self.already_loaded as f32) / (self.assets_to_load.len() as f32 + self.already_loaded as f32)
  }

  fn is_done(&self) -> bool {
    self.assets_to_load.is_empty()
  }
}

fn populate_asset_store(
  asset_server: Res<AssetServer>,
  mut asset_store: ResMut<AssetStore>,
) {
  for name in [
    Action::Attack,
    Action::Defend,
    Action::Regenerate,
    Action::Fire,
  ] {
    let handle = asset_server.load::<Image>(Into::<&'static str>::into(name));
    asset_store.load(name.into(), handle.into());
  }
  {
    let name = "autodicetable.gltf";
    let handle = asset_server.load::<Scene>(GltfAssetLabel::Scene(0).from_asset(name));
    asset_store.load(name, handle.into());
  }
}

fn update_progress(
  mut asset_store: ResMut<AssetStore>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  bar_indicator: Single<&mut Transform, With<BarIndicator>>,
) {
  let progress = asset_store.progress(asset_server.clone());

  let mut transform = bar_indicator.into_inner();
  transform.scale = Vec3::new(1.0 - progress, 1.0, 1.0);
  transform.translation = Vec3::new(0.5 * progress * INNER_BAR_SIZE.x, 0.0, 0.0);

  if asset_store.is_done() {
    commands.set_state(GameState::Menu);
  }
}

#[derive(Component)]
struct LoadingScreen;

#[derive(Component)]
struct BarIndicator;

fn spawn_loading_screen(
  mut commands: Commands,
) {
  commands.spawn((
    LoadingScreen,
    Sprite::from_color(Color::BLACK, Vec2::new(WIDTH, HEIGHT)),
  )).with_children(|commands| {
    commands.spawn((
      Sprite::from_color(Color::WHITE, BAR_SIZE),
    ))
    .with_child((
      BarIndicator,
      Sprite::from_color(Color::BLACK, INNER_BAR_SIZE),
    ));
  });
}

fn despawn_loading_screen(
  loading_screen: Single<Entity, With<LoadingScreen>>,
  mut commands: Commands,
) {
  commands
    .entity(loading_screen.into_inner())
    .despawn();
}
