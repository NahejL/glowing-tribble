use std::path::PathBuf;

const CONFIG: &str = "config"
const DISPLAY: &str = "display.ron"

pub fn get( app_root: &PathBuf ) {

  app_root
  .join( CONFIG )
  .join( DISPLAY )
}