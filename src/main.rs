mod logging;
mod running;
mod testing;

fn main() -> amethyst::Result<()> {

  logging::run();
  
  let application = running::run();

  testing::run( application );
  
  Ok(())
}

// triggersadfsadfasdasdadsaasasasdasasdasdasdasdasdasdasdaasdasdasdasdasdasdasdasdasdasd