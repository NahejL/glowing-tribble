
#[ path = "display_config => &app_root -> render_to_window_plugin" ]
mod render_to_window_plugin;

pub fn get( app_root: &PathBuf ) -> RenderingBundle< DefaultBackend > {

  RenderingBundle::< DefaultBackend >::new()
  .with_plugin( RenderUi::default() )
  .with_plugin( render_to_window_plugin.get( app_root ) )
  .with_plugin( RenderFlat2D::default() )
  // .with_plugin( RenderPbr3D::default() ) 
}