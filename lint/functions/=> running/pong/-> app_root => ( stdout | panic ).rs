
use log::info;
use amethyst::app_root_dir::application_root_dir;

pub fn get() {

  let result: Result< PathBuf, Error > = application_root_dir()
  
  match result {
    Ok( app_root ) => {
      
      info!( "app_root = {}", app_root.display().to_string() );
    
      return app_root
    },
    Err( error ) => panic!( error ),
  }
  
}