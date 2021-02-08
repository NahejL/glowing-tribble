
#[ path = "../structures/GameState.rs" ]
mod Game;


impl SimpleState for Game {

  fn on_start( &mut self, data: StateData< '_, GameData< '_, '_ > > ) {

    self.ball_spawn_timer.replace( 1.0 );
    self.sprite_sheet_handle.replace( (||{        
        
      let loader = data.world.read_resource::< Loader >();
      let texture_handle = {

        let texture_storage = data.world.read_resource::< AssetStorage< Texture > >();

        loader.load(
          "textures/pong_spritesheet.png",
          ImageFormat::default(),
          (),
          &texture_storage,
        )
      };
      let sprite_sheet_store = data.world.read_resource::< AssetStorage< SpriteSheet > >();

      return loader.load(
        "textures/pong_spritesheet.ron",
        SpriteSheetFormat( texture_handle ),
        (),
        &sprite_sheet_store,
      );
    })() );

    // #region camera init
    {
      let mut transform = Transform::default();

      transform.set_translation_xyz( 
        ARENA_WIDTH * 0.5, 
        ARENA_HEIGHT * 0.5, 
        1.0 
      );

      data.world.create_entity()
        .with( Camera::standard_2d( ARENA_WIDTH, ARENA_HEIGHT ) )
        .with( transform )
        .build();
    }
    // #endregion
    // #region paddles init
    (| sprite_handle: Handle<SpriteSheet> |{

      let sprite_render = SpriteRender::new( 
        sprite_handle, 
        0 
      );
      let y = ARENA_HEIGHT / 2.0;

      let mut init_paddle = | side: Side, x: f32 | -> () {

        let mut transform = Transform::default();

        transform.set_translation_xyz(
          x, 
          y, 
          0.0
        );

        data.world.create_entity()
          .with( Paddle::new( side ))
          .with( transform )
          .with( sprite_render.clone() )
          .build(); 

      };

      init_paddle( 
        Side::Right, 
        ARENA_WIDTH - SOME_WIDTH 
      );
      init_paddle( 
        Side::Left, 
        SOME_WIDTH * 0.5, 
      );

    })( self.sprite_sheet_handle.clone().unwrap() );
    // #endregion
    // #region scoreboard init
    {
      let font = data.world.read_resource::< Loader >().load(
        "fonts/square.ttf", 
        TtfFormat,
        (),
        &(data.world).read_resource()
      );

      let mut create_player = | string: &str, a: f32, b: f32 | -> Entity {

        let transform = UiTransform::new(
          string.to_string(), 
          Anchor::TopMiddle,
          Anchor::TopMiddle,
          a, -50.0, -1.0, 200.0, b
        );

        let ui = UiText::new(
          font.clone(),
          "0".to_string(),
          [ 1., 1., 1., 1. ],
          50.,
          LineMode::Single,
          Anchor::Middle
        );

        return data.world.create_entity()
        .with( transform )
        .with( ui )
        .build();

      };

      let left_score = create_player(
        "P!", 
        -50.0, 
        50.0,
      );
      
      let right_score = create_player(
        "P?",
        50.0, 
        50.0,
      );

      data.world.insert( ScoreText { left_score, right_score } );
      
    }
    // #endregion

  }

  fn update( &mut self, data: &mut StateData< '_, GameData< '_, '_ > > ) -> SimpleTrans {

    if let Some( mut timer ) = self.ball_spawn_timer.take() {

      {
        let time = data.world.fetch::< Time >();
        timer -= time.delta_seconds();
      }

      if timer <= 0.0 {
        // #region ball init
        (| sprite_handle: Handle<SpriteSheet> | {
          let mut local_transform = Transform::default();
          local_transform.set_translation_xyz(
            ARENA_WIDTH / 2.0, 
            ARENA_HEIGHT / 2.0,
            0.0
          );

          let sprite_render = SpriteRender::new( 
            sprite_handle, 
            1 
          );

          data.world.create_entity()
          .with( sprite_render )
          .with( Ball {
            radius: RADIUS,
            velocity: [ VELOCITY_X, VELOCITY_Y ]
          } )
          .with( local_transform )
          .build();
        })( self.sprite_sheet_handle.clone().unwrap() ); //.clone()
        // #endregion
      }
      else {
        self.ball_spawn_timer.replace( timer );
      }

    }

    Trans::None
  }

}
