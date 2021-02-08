use amethyst_rendy::RenderToWindow::{
  from_config_path,
  from_config,
};
use amethyst_window::DisplayConfig;

#[ path = "&app_root -> display_file.rs" ]
mod display_file;

pub fn get( app_root: &PathBuf ) -> RenderToWindow {

  let result = from_config_path( display_file.get( app_root ) );

  let window_plugin = match result {
    Ok( window_plugin ) => window_plugin,
    Err( error ) => {
      warn!( "RenderToWindow from_config_path failed with {}", error );
     
      from_config( DisplayConfig {
        title: "Glowing Tribble",
        dimensions: Some(( 500, 500 )),
        // icon: Some( PathBuf )
        resizable: true,
      } )
    },
  };

  window_plugin.with_clear([ 0.0, 0.0, 0.0, 1.0 ]);

  window_plugin
}

  