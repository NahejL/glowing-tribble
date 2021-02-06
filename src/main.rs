mod game;
use game::Game;

mod systems;

use amethyst::{
  prelude::*,
  renderer::{
    plugins::{ RenderFlat2D, RenderToWindow },
    types::DefaultBackend,
    RenderingBundle
  },
  utils::application_root_dir,
  core::transform::TransformBundle,
  config::ConfigError,
  error::Error,
  input::{ InputBundle, StringBindings },
  ui::{ RenderUi, UiBundle },
  
};

use log::{ info, warn };

fn main() -> amethyst::Result<()> {
  // #region init logger
  amethyst::Logger::from_config_formatter( amethyst::LoggerConfig {
    stdout: amethyst::StdoutLog::Colored,
    level_filter: amethyst::LogLevelFilter::Info,
    log_file: Option::None,
    allow_env_override: true,
    log_gfx_backend_level: Option::Some( amethyst::LogLevelFilter::Warn ),
    log_gfx_rendy_level: Option::Some( amethyst::LogLevelFilter::Warn ),
    module_levels: vec![]
  }, | out, message, record | {

    out.finish( format_args!(
      "[{level}][{target}]{message}",
      level = record.level(),
      target = record.target(),
      message = message
    ) )

  } ).start();
  // #endregion 
  // #region application setup
  { 
    info!( "ok app setup start" );

    let app_root = application_root_dir()?; // scope to asset_dir & game_data

    info!( "{}", app_root.display().to_string() );

    let asset_dir = app_root.join( "assets" );
    let game_data = (|| -> Result< GameDataBuilder, Error > {

      let rendering_bundle = (|| -> Result< RenderingBundle< DefaultBackend >, ConfigError > {
    
        let window_plugin = RenderToWindow::from_config_path( app_root.join( "config" ).join( "display.ron" ) )?
        .with_clear([ 0.0, 0.0, 0.0, 1.0 ]) ;

        let flat_plugin = RenderFlat2D::default();

        return Ok( RenderingBundle::< DefaultBackend >::new()
        .with_plugin( RenderUi::default() )
        .with_plugin( window_plugin )
        .with_plugin( flat_plugin ) );
      })()?;

      let transform_bundle = TransformBundle::new();

      let ( input_bundle, ui_bundle ) = {

        type BINDINGS = StringBindings;
        
        let input_bundle = {
      
          let bindings_path = app_root.join( "config" ).join( "bindings.ron" );
        
          InputBundle::< BINDINGS >::new()
          .with_bindings_from_file( bindings_path )?
        };

        let ui_bundle = UiBundle::< BINDINGS >::new();

        ( input_bundle, ui_bundle )
      };

      return Ok( 
        GameDataBuilder::default()
        .with_bundle( rendering_bundle )?
        .with_bundle( transform_bundle )?
        .with_bundle( input_bundle )?
        .with_bundle( ui_bundle )?
        .with( systems::PaddleSystem, "paddle_system", &[ "input_system" ] )
        .with( systems::BallSystem, "ball_system", &[] )
        .with( systems::CollisionSystem, "collision_system", &[ "paddle_system", "ball_system"]) 
        .with( systems::WinningSystem, "winning_system", &[ "ball_system" ])
      );
    })()?;

    let mut game = Application::new( 
      asset_dir, 
      Game::default(), 
      game_data 
    )?; 

    game.run();
  } 
  // #endregion
  Ok(())
}

// triggersadfsadfasdasdadsaasasasdasasdasdasdasdasdasd