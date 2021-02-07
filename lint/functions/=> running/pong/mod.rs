#[ path = "../-> ( PathBuf, GameDataBuilder )/default.rs" ]
mod consumeRootDir;

use log::info;

pub fn run() {
  info!( "ok app setup start" );
  // PathBuf -> ( PathBuf, GameDataBuilder ) => ()
  let ( asset_dir, game_data ) = consumeRootDir::run();

  let mut game = Application::new( 
    asset_dir, 
    Game::default(), 
    game_data 
  )?; 

  game.run();
}