use amethyst::Application;

#[ path = "-> app_root => ( stdout | panic ).rs" ]
mod app_root;

#[ path = "&app_root -> asset_dir.rs" ]
mod asset_dir;

#[ path = "&app_root -> GameDataBuilder => panic?/mod.rs" ]
mod game_data_builder;

#[ path = "../../../structures/GameState.rs" ]
mod GameState;
#[ path = "../../../implementations/State for GameState.rs" ]
mod GameStateDefault;

use log::info;

pub fn run() {
  info!( "ok app setup start" );
  
  let app_root = app_root::get();
  let asset_dir = asset_dir::get( &app_root );
  let game_data_builder = game_data_builder::get( &app_root );

  // should dispose of app_root

  let result = Application::new( 
    asset_dir, 
    GameState::GameState::default(), 
    game_data_builder, 
  ); 

  match result {
    Ok( application ) => application.run(),
    Err( error ) => panic!( error ),
  };

}