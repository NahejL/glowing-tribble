use std::{slice::Iter, io::Read, ops::Deref};

use amethyst::{CoreApplication, GameData, StateEvent};

use deno_core::{JsRuntime, Op, json_op_sync};

use serde_json::Value;

pub fn run( _application: CoreApplication< 'static, GameData< 'static, 'static >, StateEvent >) -> Result< (), () > {

  let mut v8 = JsRuntime::new( Default::default() );

  runtime.register_op( 
    "op_print",
    | _state, zero_copy | {

      let mut out = std::io::stdout();

      for buffer in zero_copy {

        out.write_all( &buffer ).unwrap();

      }

      Op::Sync( Box::new([]) )
    }
  )

  runtime.register_op(
    "op_sum",
    json_op_sync( | _state, json: Vec<f64>, zero_copy | {

      if !zero_copy.is_empty() {
        
        log::info!( "Expected one argument" );
        Err()
      }
      else {

        let sum = json.iter().fold( 0.0, | a, v | a + v );
        Ok( Value::from( sum ) )
      }

    } )
  )

  Ok(())
}

