use std::path::PathBuf;


const PATH: &str = "assets" ;

/// Doesnt need to be segmented ?
pub fn get( app_root: &PathBuf ) -> PathBuf {

  app_root.join( PATH )
}
