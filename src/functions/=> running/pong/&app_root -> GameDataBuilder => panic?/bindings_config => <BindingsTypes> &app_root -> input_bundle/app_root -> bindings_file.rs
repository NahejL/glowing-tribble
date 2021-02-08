use std::path::PathBuf;

const CONFIG: &str = "config";
const BINDINGS: &str = "bindings.ron";

pub fn get( app_root: &PathBuf ) {

  app_root
  .join( CONFIG )
  .join( BINDINGS )
}