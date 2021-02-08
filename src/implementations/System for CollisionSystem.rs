
impl<'s> System<'s> for CollisionSystem {

  type SystemData = (
    WriteStorage< 's, Ball >,
    ReadStorage< 's, Paddle >,
    ReadStorage< 's, Transform >,
  );

  fn run ( &mut self, ( mut balls, paddles, transforms ): Self::SystemData ) {
    // iterate balls
    for( ball, transform ) in ( &mut balls, &transforms ).join() {

      let ball_x = transform.translation().x;
      let ball_y = transform.translation().y;
      // Collision top/bottom
      if ( ball_y <= ball.radius && ball.velocity[1] < 0.0 ) || ( ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0 ) {

        ball.velocity[1] = - ball.velocity[1];

      }
      // iterate paddles
      for( paddle, paddle_transform ) in ( &paddles, &transforms ).join() {

        let paddle_x = paddle_transform.translation().x - ( paddle.width * 0.5 );
        let paddle_y = paddle_transform.translation().y - ( paddle.height * 0.5 );
        // collision paddles
        if ( // point in rect  
          ball_x >= ( paddle_x - ball.radius ) && 
          ball_x <= ( paddle_x + paddle.width + ball.radius ) &&
          ball_y >= ( paddle_y - ball.radius ) &&
          ball_y <= ( paddle_y + paddle.height + ball.radius ) 
        )  && 
        ( 
          ( // moving in left
            paddle.side == Side::Left && 
            ball.velocity[0] < 0.0 
          ) || 
          ( // moving in right
            paddle.side == Side::Right && 
            ball.velocity[0] > 0.0 
          ) 
        ){

          ball.velocity[0] = -ball.velocity[0];

        }

      }

    }

  }

}
