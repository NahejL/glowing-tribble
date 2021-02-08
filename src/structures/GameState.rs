#[derive( Default )]
pub struct GameState {
  ball_spawn_timer: Option< f32 >,
  sprite_sheet_handle: Option< Handle < SpriteSheet > >,
}
