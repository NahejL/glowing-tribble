use amethyst::{
  //assets::{ AssetStorage, Loader, Handle },
  //core::transform::Transform,
  ecs::{ Component, DenseVecStorage },
  //prelude::*,
  //renderer::{ Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture }
};

pub const SOME_HEIGHT: f32 = 16.0;
pub const SOME_WIDTH: f32 = 4.0;

#[derive(PartialEq)]
pub enum Side {
  Left,
  Right
}

pub struct Paddle {
  pub side: Side,
  pub width: f32,
  pub height: f32,
}

impl Paddle {

  pub fn new( side: Side ) -> Paddle {
    Paddle {
      side,
      width: SOME_WIDTH,
      height: SOME_HEIGHT,
    }
  }

}

impl Component for Paddle {

  type Storage = DenseVecStorage< Self >;

}