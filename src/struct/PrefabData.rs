
use amethyst::{
  animation::{
      AnimationSetPrefab,
  },
  assets::{PrefabData, ProgressCounter},
  derive::PrefabData,
  ecs::{prelude::Entity},
  error::Error,
  renderer::{
      sprite::{prefab::SpriteScenePrefab, SpriteRender},
  },
};
use serde::Deserialize;

#[ path = "../enum/SpriteAnimationIds.rs" ]
mod sprite_animation_ids;
use sprite_animation_ids::AnimationId;

#[ derive( Debug, Clone, Deserialize, PrefabData )]
pub struct CustomPrefabData {
  
  sprite_scene: SpriteScenePrefab,

  animation_set: AnimationSetPrefab< AnimationId, SpriteRender >,

}