
impl<'s> System<'s> for WinningSystem {

  type SystemData = (
    WriteStorage< 's, Ball >,
    WriteStorage< 's, Transform >,
    WriteStorage< 's, UiText >,
    Write< 's, ScoreBoard >,
    ReadExpect< 's, ScoreText >,
  );

  fn run( &mut self, ( mut balls, mut locals, mut ui_texts, mut scores, score_texts ): Self::SystemData ) {

    for ( ball, transform ) in ( &mut balls, &mut locals ).join() {

      let ball_x = transform.translation().x;
      
      let mut update_text = | board: &mut i32, entity: Entity | {

        *board = ( *board + 1 ).min( 999 );

        if let Some( text ) = ui_texts.get_mut( entity ) {

          text.text = board.to_string();

        }
              
      };

      if  
        if ball_x <= ball.radius {

          update_text( &mut scores.score_right, score_texts.right_score );
          
          true
        }
        else if ball_x >= ARENA_WIDTH - ball.radius {

          update_text( &mut scores.score_left, score_texts.left_score );

          true
        }
        else { false } {

        ball.velocity[0] = -ball.velocity[0];
        transform.set_translation_x( ARENA_WIDTH / 2.0 );
        transform.set_translation_y( ARENA_HEIGHT / 2.0 );

      }

    }

  }

}
