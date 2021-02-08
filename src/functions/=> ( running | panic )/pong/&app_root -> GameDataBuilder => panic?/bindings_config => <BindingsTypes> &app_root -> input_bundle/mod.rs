use amethyst::{
  input::{
    InputBundle,
    BindingTypes
  }
};
use std::path::PathBuf;
use log::warn;

#[ path = "app_root -> bindings_file.rs" ]
mod bindings_file;

#[ path = "InputBundle => panic.rs" ]
mod bindings;

pub fn get< BindingType: BindingTypes >( app_root: &PathBuf ) {

  let bundle = InputBundle::< BindingType >::new();

  let result = bundle.with_bindings_from_file( bindings_file::get( app_root ) );

  if let Err( error ) = result {
    warn!( "InputBundle with_bindings_from_file failed with {}", error );
    
    bindings::get( &bundle );

  }

  bundle
}

