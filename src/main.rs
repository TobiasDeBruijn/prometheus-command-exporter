mod config;
mod metrics;
mod error;

use actix_web::{App, HttpServer};
use log::{error, info, trace};

#[cfg(not(unix))]
compile_error!("This program only works on UNIX platforms");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }

    let matches = clap_app().get_matches();
    let config = matches.value_of("config");
    let port = matches.value_of("port").expect("Missing 'port'");
    if matches.is_present("verbose") {
        use std::env::set_var;
        // 0: Default
        // 1: Warn
        // 2: Info
        // 3: Trace
        let occurrence = matches.occurrences_of("verbose");
        match occurrence {
            0 => unreachable!(),
            1 => set_var("RUST_LOG", "warn"),
            2 => set_var("RUST_LOG", "info"),
            3 | _ => set_var("RUST_LOG", "trace")
        }
    }

    env_logger::try_init().expect("Failed to initialize logger");
    info!("Starting application");

    info!("Reading configuration");
    let config = match if let Some(config) = config {
        let path = std::path::Path::new(config);
        if !path.exists() {
            error!("Provided path {} does not exist.", config);
            std::process::exit(1);
        }

        config::Config::read_from_path(path)
    } else {
        config::Config::read()
    } {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read configuration: {:?}", e);
            std::process::exit(1);
        }
    };
    trace!("Configuration read successfully.");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .data(config.clone())
            .service(metrics::metrics)

    })
        .bind(format!("[::]:{}", port))?
        .run()
        .await
}

fn clap_app() -> clap::App<'static, 'static> {
    use clap::Arg;
    use const_format::formatcp;

    clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("config")
            .takes_value(true)
            .short("c")
            .long("config")
            .help(formatcp!("The configuration path to use. Defaults to {}{}", config::DEFAULT_PATH, config::DEFAULT_CONFIG_FILE)))
        .arg(Arg::with_name("port")
            .takes_value(true)
            .short("p")
            .long("port")
            .help("The TCP port to bind to")
            .default_value("10405"))
        .arg(Arg::with_name("verbose")
            .takes_value(false)
            .short("v")
            .multiple(true)
            .help("Determines the level of verbosity. Verbosity can also be set with the RUST_LOG environmental variable. Note that using this option overrides the environmental variable."))
}