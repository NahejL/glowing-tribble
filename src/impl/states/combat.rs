

use amethyst::{core::{Transform, math::{ Vector3}}, renderer::ActiveCamera};
#[ allow(unused) ]
use amethyst::{GameData, State, StateData, StateEvent, Trans, animation::{AnimationCommand, AnimationControlSet, AnimationSet, EndControl, get_animation_set}, assets::{AssetStorage, Loader, PrefabLoader, RonFormat}, ecs::{Read, Entities, Join, ReadStorage, WriteStorage, World}, input::{ VirtualKeyCode, is_close_requested, is_key_down}, prelude::{Builder, WorldExt}, renderer::{Camera, SpriteRender, Sprite, SpriteSheet, Texture, loaders::load_from_srgba, palette::Srgba}, window::ScreenDimensions};

#[ path = "../../struct/CombatState.rs" ]
mod state;
pub use state::CombatState;

#[ path = "../bindingTypes/input.rs" ]
mod input;

impl State<GameData<'static, 'static>, StateEvent> for CombatState{ // 

  /// Executed when the game state begins.
  fn on_start( &mut self, _data: StateData<'_, GameData > ) {
    
    #[allow(unused)]
    let StateData { world, data } = _data;

    log::info!( "CombatState on_start" );
    
    // setup sprite
    {
    
      let handle = {

        let mut loader = world.write_resource::<Loader>();

        loader.set_hot_reload( true );
        
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let sprites = vec![
          Sprite::from_pixel_values(
            1, // image_w: Width of the full sprite sheet.
            1, // image_h: Height of the full sprite sheet.
            1, // sprite_w: Width of the sprite.
            1, // sprite_h: Height of the sprite.
            0,// pixel_left: Pixel X coordinate of the left side of the sprite.
            0,// pixel_top: Pixel Y coordinate of the top of the sprite.
            [ 0., 0. ],// offsets: Number of pixels to shift the sprite to the left and down relative to the entity.
            false, false,
          ),
        ];
        let texture = {
        
          let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        
          loader.load_from_data(
            load_from_srgba( Srgba::new( 0.1, 0.5, 0.3, 1.0 ) ).into(),
            (),
            & texture_storage
          )
        };

        loader.load_from_data(
          SpriteSheet { 
            texture,
            sprites,
          },// data: A::Data,
          (), // mut progress: P,
          &sprite_sheet_storage// storage: &AssetStorage<A>,
        )
      };

      world.create_entity()
      .with( SpriteRender::new(
        handle,  // sprite_sheet: Handle<SpriteSheet>, 
        0  // sprite_number: usize
      ) )
      .with({ 
        let mut transform = Transform::default();

        transform.set_translation_xyz( -5., -5., -5. );
        transform.set_scale( Vector3::new( 5., 5., 5. ) );

        transform
      })
      .build();
    
    };

    // setup camera 
    {

      let entity = {

        let mut transform = Transform::default();
        
        transform.set_translation_xyz( 1., 1., 1. );
        transform.face_towards(
           Vector3::new(0.0, 0.0, 0.0),
           Vector3::new(0.0, 0.0, 1.0)
        );
                  
        world.create_entity()
        .with( transform )
        // .with( Camera::orthographic( 
        //     0.0,
        //     1.0,
        //     0.0,
        //     1.0,
        //     0.1,
        //     2000.0,
        //  ) )
        .with( Camera::perspective( 1., std::f32::consts::FRAC_PI_3, 0.1 ) ) 
        .build()
      };

      let mut camera = world.write_resource::<ActiveCamera>();

      camera.entity = Some( entity );

    }

  }

  /// Executed when the game state exits.
  fn on_stop( &mut self, _data: StateData<'_, GameData< '_, '_ > >) { }

  /// Executed when a different game state is pushed onto the stack.
  fn on_pause( &mut self, _data: StateData<'_, GameData< '_, '_ > >) { }

  /// Executed when the application returns to this game state once again.
  fn on_resume( &mut self, _data: StateData<'_, GameData< '_, '_ > >) { }

  /// Executed on every frame before updating, for use in reacting to events.
  fn handle_event( &mut self, _data: StateData<'_, GameData< '_, '_ > >, _event: StateEvent ) -> Trans< GameData< 'static, 'static >, StateEvent > {
    // log::info!( "CombatState handle_event {:?}", _event );
    #[allow(unused)]
    let StateData { world, .. } = _data;

    match &_event {
      StateEvent::Window( _event ) => {
        // log::info!( "CombatState handle_event Window {:?}", _event );
        
        if is_close_requested( & _event ) || is_key_down( &_event, VirtualKeyCode::Escape ){ 
          // log::info!( "CombatState handle_event Window is_close_requested" );
          return Trans::Quit; 
        }
      
      },
      StateEvent::Ui( _event) => {

        log::info!( "Ui event {:?}", _event );
      
      },
      StateEvent::Input( _event ) => {

        log::info!( "Input event {:?}", _event );

      },
    };

    Trans::None
  }

  /// Executed repeatedly at stable, predictable intervals (1/60th of a second by default).
  fn fixed_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) -> Trans< GameData< 'static, 'static >, StateEvent > { Trans::None }

  /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit).
  fn update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) -> Trans< GameData< 'static, 'static >, StateEvent > {

    #[allow(unused)]
    let StateData { world, data } = _data;


    data.update( &world );
    Trans::None
  
  }

  /// Executed repeatedly at stable, predictable intervals (1/60th of a second
  /// by default),
  /// even when this is not the active state,
  /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
  fn shadow_fixed_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) { }

  /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit),
  /// even when this is not the active state,
  /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
  fn shadow_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) { }

}