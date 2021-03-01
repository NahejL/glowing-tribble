use amethyst::{GameDataBuilder, StateEvent, assets::{HotReloadBundle, HotReloadStrategy}, audio::AudioBundle, input::InputBundle, renderer::{RenderBase3D, RenderDebugLines, RenderSkybox, bundle::Target, palette::Srgb, pass::FlatPassDef}, ui::{RenderUi, UiBundle}, utils::fps_counter::FpsCounterBundle, window::DisplayConfig};
#[ allow(unused_imports)]
use amethyst::{
  animation::{
      get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
      AnimationSetPrefab, EndControl,
  },
  assets::{PrefabData, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter, RonFormat},
  core::transform::{Transform, TransformBundle},
  derive::PrefabData,
  ecs::{prelude::{Entity, DispatcherBuilder}, Entities, Join, ReadStorage, WriteStorage},
  error::Error,
  prelude::{Builder, World, WorldExt},
  renderer::{
      camera::Camera,
      plugins::{RenderFlat2D, RenderToWindow},
      rendy::hal::command::ClearColor,
      sprite::{prefab::SpriteScenePrefab, SpriteRender},
      types::DefaultBackend,
      RenderingBundle,
  },
  utils::application_root_dir,
  window::ScreenDimensions,
  Application, GameData, SimpleState, SimpleTrans, StateData, Trans,
};

#[ path = "impl/states/combat.rs" ]
mod state_mod;
use state_mod::CombatState as initState;

#[ path = "impl/bindingTypes/input.rs" ]
mod input;

#[ path = "struct/PrefabData.rs" ]
mod custom_prefab_data;
use custom_prefab_data::CustomPrefabData;

pub fn run() -> amethyst::CoreApplication< 'static, GameData<'static, 'static>, StateEvent > {

  log::info!( "ok app setup start" );

  let app_root = match amethyst::utils::application_root_dir() {
    Err( error ) => panic!( "Failed at application_root_dir with: {}", error ),
    Ok( app_root ) => {
    
      log::info!( "app_root = {}", app_root.display().to_string() );
    
      app_root
    },
  };

  let asset_dir = app_root.join( "assets" );
  // let config_dir = app_root.join( "config" );

  let game_data = {

    match (|| -> Result< GameDataBuilder, amethyst::Error > {
  
      
      GameDataBuilder::default()
      .with_system_desc(
        PrefabLoaderSystemDesc::<CustomPrefabData>::default(),
        "scene_loader",
        &[]
      )
      // https://docs-src.amethyst.rs/master/amethyst_core/transform/index.html
      .with_bundle( TransformBundle::new() )?
      // https://docs.amethyst.rs/stable/amethyst_animation/struct.AnimationBundle.html
      .with_bundle( AnimationBundle::< (), Transform >::new(
        "animation_control_system",
        "sampler_interpolation_system",) )?
      // https://docs.amethyst.rs/stable/amethyst_assets/index.html
      .with_bundle( HotReloadBundle::new( HotReloadStrategy::every( 2 ) ) )?
      // https://docs.amethyst.rs/stable/amethyst_audio/struct.AudioBundle.html
      .with_bundle( AudioBundle::default() )?
      // https://docs.amethyst.rs/stable/amethyst_input/index.html
      .with_bundle( InputBundle::< input::InputBindingTypes >::new() )?
      // https://docs.amethyst.rs/stable/amethyst_rendy/index.html
      .with_bundle( { RenderingBundle::< DefaultBackend >::new()
        .with_plugin( { RenderDebugLines::default()
          .with_target( Target::Main ) } )
        .with_plugin( { RenderUi::default()
          .with_target( Target::Main ) } )
        .with_plugin( { RenderFlat2D::default()
          .with_target( Target::Main ) } )
        .with_plugin( { RenderSkybox::with_colors( 
          Srgb::new( 1.0, 1.0, 1.0 ), 
          Srgb::new( 0.0, 0.0, 0.0) ) 
          .with_target( Target::Main ) } )
        //https://docs.amethyst.rs/stable/amethyst_window/index.html
        .with_plugin( { RenderToWindow::from_config( DisplayConfig {
          title: String::from("Glowing Tribble"),
          fullscreen: None,
          dimensions: None,
          min_dimensions: None,
          max_dimensions: None,
          visibility: true,
          icon: None,
          always_on_top: false,
          decorations: true,
          maximized: true,
          multitouch: false,
          resizable: true,
          transparent: false,
          loaded_icon: None,
          } )
          .with_target( Target::Main )
          .with_clear([ 1.0, 1.0, 1.0, 1.0 ])
        } )
        .with_plugin( { RenderBase3D::<FlatPassDef>::default() 
          .with_target( Target::Main ) } ) } )?
      // https://docs.amethyst.rs/stable/amethyst_ui/struct.UiBundle.html
      .with_bundle( UiBundle::< input::InputBindingTypes >::new() )? // Will fail with error 'No resource with the given id' if the InputBundle is not added.
      // https://docs.amethyst.rs/stable/amethyst_utils/fps_counter/index.html
      .with_bundle( FpsCounterBundle )
    })() {
      Ok( game_data_builder ) => game_data_builder,
      Err( error ) => panic!( "Failed GameDataBuilder with_bundle with: {}", error ),
    }
  };

  match amethyst::Application::new( // ::<input::InputBindingTypes> ?
    asset_dir, 
    initState::default(), 
    game_data 
  ) {
    Err( error ) => panic!( "Application::new failed with: {}", error ),
    Ok( mut application ) => { 

      application.run();

      application
    },
  } 

} 