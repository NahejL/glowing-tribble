
// fragment syntaxt: [consumes] -> [produces] => [affects]
// eg: () -> () => logging
// reduced: => logging

#[ path = "functions/=> logging/default.rs" ]
mod logging;

#[ path = "functions/=> ( running | panic )/pong/mod.rs" ]
mod running;

fn main() -> amethyst::Result<()> {

  logging::run();

  running::run();

  Ok( () )
}