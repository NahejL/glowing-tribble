
use amethyst::{
  Logger,
  LoggerConfig,
  StdoutLog::Colored,
  LogLevelFilter::{
    Info,
    Warn
  }
};

/// Create and Consume Logger with (currently) default configuration
pub fn run(){

  let logger_config = LoggerConfig {
    stdout: Colored,
    level_filter: Info,
    log_file: None,
    allow_env_override: true,
    log_gfx_backend_level: Some( Warn ),
    log_gfx_rendy_level: Some( Warn ),
    module_levels: vec![]
  };

  Logger::from_config_formatter( 
    logger_config, 
    | out, message, record | {
  
      let output = format_args!(
        "[{level}][ {target} ] {message}",
        level = record.level(),
        target = record.target(),
        message = message
      );
  
      out.finish( output )
    
    }
  )
  .start();

}
