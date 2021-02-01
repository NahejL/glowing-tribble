use amethyst::{
  //assets::{ AssetStorage, Loader, Handle },
  //core::transform::Transform,
  ecs::{ Component, DenseVecStorage },
  //prelude::*,
  //renderer::{ Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture }
};


pub const VELOCITY_X : f32 = 75.0;
pub const VELOCITY_Y : f32 = 50.0;
pub const RADIUS : f32 = 2.0;

pub struct Ball {
  pub velocity: [ f32; 2 ],
  pub radius: f32,
}

impl Component for Ball {
  type Storage = DenseVecStorage< Self >;
}

