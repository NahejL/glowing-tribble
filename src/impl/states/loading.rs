use amethyst::{GameData, State, StateData, StateEvent, Trans, input::{VirtualKeyCode, is_close_requested, is_key_down}, prelude::WorldExt, ui::UiCreator};


#[ path = "../../struct/LoadingState.rs" ]
mod state;
pub use state::LoadingState;

impl State<GameData<'static, 'static>, StateEvent> for LoadingState {

  /// Executed when the game state begins.
  fn on_start( &mut self, _data: StateData<'_, GameData< '_, '_ > > ) {
    log::info!( "LoadingState on_start" );
    
    #[allow(unused)]
    let StateData { world, data } = _data;

    self.ui_handle = Some( world.exec( |mut creator: UiCreator<'_>| creator.create( "ui/loading.ron", () ) ) );

  }

  /// Executed when the game state exits.
  fn on_stop( &mut self, _data: StateData<'_, GameData< '_, '_ > >) {
  
    #[allow(unused)]
    let StateData { world, data } = _data;

    if let Some( root_entity ) = self.ui_handle {
      world.delete_entity( root_entity ).expect( "Failed to remove Loading Ui");
    }

    self.ui_handle = None;
    
  }

  /// Executed when a different game state is pushed onto the stack.
  fn on_pause( &mut self, _data: StateData<'_, GameData< '_, '_ > >) {

    
  }

  /// Executed when the application returns to this game state once again.
  fn on_resume( &mut self, _data: StateData<'_, GameData< '_, '_ > >) {

    
  }

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
  fn fixed_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) -> Trans< GameData< 'static, 'static >, StateEvent > {

    Trans::None
  }

  /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit).
  fn update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) -> Trans< GameData< 'static, 'static >, StateEvent > {

    _data.data.update( &_data.world );

    Trans::None
  }

  /// Executed repeatedly at stable, predictable intervals (1/60th of a second
  /// by default),
  /// even when this is not the active state,
  /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
  fn shadow_fixed_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) {
      
    
  }

  /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit),
  /// even when this is not the active state,
  /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
  fn shadow_update( &mut self, _data: StateData<'_, GameData< '_, '_ > >) {

    
  }

  

}