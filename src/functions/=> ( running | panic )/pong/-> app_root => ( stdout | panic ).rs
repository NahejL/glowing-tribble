
use log::info;
use amethyst::utils::app_root_dir::application_root_dir;
use std::{
  path::PathBuf,
  io::Error
};

pub fn get() -> PathBuf {

  let result: Result< PathBuf, Error > = application_root_dir();
  
  match result {
    Ok( app_root ) => {
      
      info!( "app_root = {}", app_root.display().to_string() );
    
      return app_root
    },
    Err( error ) => panic!( error ),
  }
  
}