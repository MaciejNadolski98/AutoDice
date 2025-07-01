use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};
use bevy_defer::AccessError;

use crate::{constants::{dice_texture::{FONT_SIZE, OFFSET, PIPS_POSITION, SCALING_FACTOR, TARGET_SIZE}, DICE_FACES_LAYER}, dice::{action::{resolve, ResolutionContext}, dice_template::FacePrototype, DiceID}, loading_screen::AssetStore};

pub struct FacePlugin;

impl Plugin for FacePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<FaceCounter>()
      .add_systems(First, deactivate_face_cameras)
      .add_systems(Update, activate_face_cameras);
  }
}

#[derive(Component, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Face {
  pub prototype: FacePrototype,
  pub image: Handle<Image>,
}

impl Face {
  pub fn spawn(self, commands: &mut RelatedSpawnerCommands<ChildOf>) {
    let entity = commands.spawn(self).id();
    commands.commands().run_system_cached_with(initialize_face, entity);
  }

  pub fn from_prototype(prototype: FacePrototype, images: &mut Assets<Image>) -> Self {
    Self { prototype, image: build_face_image(images) }
  }

  pub fn from_other(other: &Self, images: &mut Assets<Image>) -> Self {
    Self::from_prototype(other.prototype, images)
  }

  pub async fn resolve(self, dice_id: DiceID) -> Result<(), AccessError> {
    resolve(ResolutionContext { face: self.prototype, dice_id }).await
  }
}

#[derive(Component)]
#[relationship(relationship_target = FaceSourceOf)]
pub struct FaceSource {
  source: Entity,
}

impl FaceSource {
  pub fn new(source: Entity) -> Self {
    Self {
      source
    }
  }

  pub fn source(&self) -> Entity {
    self.source
  }
}

#[derive(Component)]
#[relationship_target(relationship = FaceSource)]
pub struct FaceSourceOf {
  entity: Entity,
}

#[derive(Component)]
#[relationship(relationship_target = FaceCamera)]
struct FaceCameraOf {
  face: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = FaceCameraOf, linked_spawn)]
struct FaceCamera {
  camera: Entity,
}

#[derive(Component)]
#[relationship(relationship_target = FaceRoot)]
struct FaceRootOf {
  face: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = FaceRootOf, linked_spawn)]
struct FaceRoot {
  root: Entity,
}

pub trait Gridable: Component {
  fn grid(&self) -> Vec<(i16, i16)>;
}

pub fn build_face_image(images: &mut Assets<Image>) -> Handle<Image> {
  let size = Extent3d {
    width: TARGET_SIZE as u32,
    height: TARGET_SIZE as u32,
    depth_or_array_layers: 1,
  };
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };
  image.resize(size);
  images.add(image)
}

#[derive(Resource, Default, Deref)]
struct FaceCounter(u32);

fn initialize_face(
  face_entity: In<Entity>,
  faces: Query<&Face>,
  mut face_counter: ResMut<FaceCounter>,
  mut commands: Commands,
) {
  let face = faces.get(*face_entity).unwrap();
  let position = Vec3::X * TARGET_SIZE * (**face_counter as f32);
  face_counter.0 += 1;

  commands
    .entity(*face_entity)
    .with_related::<FaceRootOf>((
        Name::new("Dice face root"),
        Visibility::Visible,
        Transform::from_translation(position),
        DICE_FACES_LAYER,
    ))
    .with_related::<FaceCameraOf>((
        Name::new("Dice texture camera"),
        Camera2d,
        Camera {
          order: -1,
          target: face.image.clone().into(),
          ..default()
        },
        Transform::from_translation(position + Vec3::Z).looking_at(position, Vec3::Y),
        DICE_FACES_LAYER,
    ));
}


fn deactivate_face_cameras(
  cameras: Query<&mut Camera, (Changed<Camera>, With<FaceCameraOf>)>,
) {
  for mut camera in cameras {
    camera.is_active = false;
  }
}

fn activate_face_cameras(
  faces: Query<(&FaceCamera, &FaceRoot, &Face), Changed<Face>>,
  mut cameras: Query<&mut Camera>,
  mut commands: Commands,
  asset_store: Res<AssetStore>,
) {
  for (
    FaceCamera { camera },
    FaceRoot { root },
    Face { prototype: FacePrototype { action, pips, background }, .. }
  ) in faces {
    commands
      .entity(*root)
      .despawn_related::<Children>()
      .with_children(|commands| {
        commands.spawn((
          Name::new("Background"),
          Sprite::from_color(
            Color::linear_rgb(0.2, 0.2, 0.2),
            Vec2::splat(TARGET_SIZE)
          ),
          DICE_FACES_LAYER,
        )).with_children(|commands| {
          commands.spawn((
            Name::new("Foreground"),
            Sprite::from(*background),
            DICE_FACES_LAYER,
          )).with_children(|commands| {
            commands.spawn((
              Name::new("Face icon"),
              Sprite::from_image(asset_store.get((*action).into())),
              Transform::default()
                .with_scale(Vec3::splat(SCALING_FACTOR))
                .with_translation((OFFSET, 0.0).into()),
              DICE_FACES_LAYER,
            ));

            if let Some(pips) = pips {
              commands.spawn((
                Name::new("Pips"),
                Text2d(format!("{}", pips)),
                TextFont {
                  font_size: FONT_SIZE,
                  ..default()
                },
                TextColor::BLACK,
                Transform::from_translation((PIPS_POSITION, 1.0).into()),
                DICE_FACES_LAYER,
              ));
            }
          });
        });
      });
    cameras.get_mut(*camera).unwrap().is_active = true;
  }
}

#[derive(Default, Component)]
struct FaceTooltip;
