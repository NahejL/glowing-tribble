
use amethyst::{
  prelude::*,
  renderer::{
    plugins::{ RenderFlat2D, RenderToWindow },
    types::DefaultBackend,
    RenderingBundle
  },
  core::transform::TransformBundle,
  error::Error,
  input::{ InputBundle, StringBindings },
  ui::{ RenderUi, UiBundle },
  
};


pub fn run(){

  log::info!( "ok app setup start" );

  let app_root = match amethyst::utils::application_root_dir() {
    Err( error ) => panic!( "Failed at application_root_dir with: {}", error ),
    Ok( app_root ) => {
    
      log::info!( "app_root = {}", app_root.display().to_string() );
    
      app_root
    },
  };

  let asset_dir = app_root.join( "assets" );
  let config_dir = app_root.join( "config" );

  let game_data = (|| -> GameDataBuilder {

    type BINDINGS = StringBindings;

    let rendering_bundle = (|| -> RenderingBundle< DefaultBackend > {
  
      let window_plugin = match RenderToWindow::from_config_path( app_root.join( "config" ).join( "display.ron" ) ) {
        Ok( plugin ) => plugin,
        Err( error ) => {

          log::warn!( "Failed RenderToWindow from_config_path with: {}", error );

          RenderToWindow::from_config( amethyst::window::DisplayConfig::default() )
        },
      }.with_clear([ 0.0, 0.0, 0.0, 1.0 ]);
      
      RenderingBundle::< DefaultBackend >::new()
      .with_plugin( RenderUi::default() )
      .with_plugin( window_plugin )
      .with_plugin( RenderFlat2D::default() )
    })();

    let input_bundle = match InputBundle::< BINDINGS >::new().with_bindings_from_file( config_dir.join( "bindings.ron" ) ) {
      Ok( bundle ) => bundle,
      Err( error ) => panic!( "Failed InputBundle with_bindings_from_file with: {}", error ),
    };

    let game_data_builder = match (|| -> Result< GameDataBuilder, Error > {
      GameDataBuilder::default()
      .with_bundle( TransformBundle::new() )?
      .with_bundle( input_bundle )?
      .with_bundle( UiBundle::< BINDINGS >::new() )?
      .with_bundle( rendering_bundle )
    })() {
      Ok( game_data_builder ) => game_data_builder,
      Err( error ) => panic!( "Failed GameDataBuilder with_bundle with: {}", error ),
    }; 

    game_data_builder.with( super::systems::PaddleSystem, "paddle_system", &[ "input_system" ] )
    .with( super::systems::BallSystem, "ball_system", &[] )
    .with( super::systems::CollisionSystem, "collision_system", &[ "paddle_system", "ball_system"]) 
    .with( super::systems::WinningSystem, "winning_system", &[ "ball_system" ])
  })();

  log::info!( "ok" );

  match Application::new( 
    asset_dir, 
    super::game::Game::default(), 
    game_data 
  ) {
    Err( error ) => panic!( "Application::new failed with: {}", error ),
    Ok( mut application ) => application.run(),
  }; 

} 