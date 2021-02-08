
pub fn run() {
  
  amethyst::Logger::from_config_formatter( amethyst::LoggerConfig {
    stdout: amethyst::StdoutLog::Colored,
    level_filter: amethyst::LogLevelFilter::Info,
    log_file: Option::None,
    allow_env_override: true,
    log_gfx_backend_level: Option::Some( amethyst::LogLevelFilter::Warn ),
    log_gfx_rendy_level: Option::Some( amethyst::LogLevelFilter::Warn ),
    module_levels: vec![]
  }, | out, message, record | {

    out.finish( format_args!(
      "[{level}][{target}]{message}",
      level = record.level(),
      target = record.target(),
      message = message
    ) )

  } ).start();

}