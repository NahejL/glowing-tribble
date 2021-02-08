
use log::info;

#[ path = "&app_root -> rendering_bundle" ]
mod rendering_bundle;

#[ path = "bindings_config => <BindingsTypes> &app_root -> input_bundle" ]
mod input_bundle;

pub fn get( app_root: &PathBuf ) -> GameDataBuilder {

  type BINDINGS = StringBindings;

  let game_data_builder = GameDataBuilder::default();
  let bundles = vec![
    rendering_bundle.get( app_root ),
    TransformBundle::new(),
    input_bundle.get< BINDINGS >( app_root ),
    UiBundle::< BINDINGS >::new()
  ];

  for bundle in bundles.iter() {

    let result = game_data_builder.with_bundle( bundle )

    if let Err( error ) = result {

      panic!( "GameDataBuilder with_bundle failed with: {}", error );

    }

  }

  game_data_builder
  .with( systems::PaddleSystem, "paddle_system", &[ "input_system" ] )
  .with( systems::BallSystem, "ball_system", &[] )
  .with( systems::CollisionSystem, "collision_system", &[ "paddle_system", "ball_system"]) 
  .with( systems::WinningSystem, "winning_system", &[ "ball_system" ])
}
