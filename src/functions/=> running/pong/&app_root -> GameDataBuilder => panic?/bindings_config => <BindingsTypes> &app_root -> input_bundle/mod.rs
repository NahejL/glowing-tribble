use amethyst_input::{
  InputBundle,
  BindingsTypes
};

#[ path = "app_root -> bindings_file.rs" ]
mod bindings_file;

#[ path = "Bindings => panic.rs" ]
mod bindings;

pub fn get< T: BindingsTypes >( app_root: &PathBuf ) {

  let bundle = InputBundle::< BINDINGS >::new();
  let result = bundle.with_bindings_from_file( bindings_file.get( app_root ) );

  if let Err( error ) = result {
    warn!( "InputBundle with_bindings_from_file failed with {}", error );
    
    bindings::get( &bundle );

  }

  bundle
}

