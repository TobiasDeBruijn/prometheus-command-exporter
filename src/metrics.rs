use std::io::Read;
use actix_web::{web, get, HttpResponse};
use crate::error::HttpResult;
use crate::config::{Config, PrometheusType};
use log::trace;
use prometheus::{Counter, Encoder, Gauge, Registry, TextEncoder};
use std::process::{Command, Stdio};

#[get("/metrics")]
pub async fn metrics(data: web::Data<Config>) -> HttpResult {
    let reg = Registry::new();

    for export in &data.exports {
        trace!("Executing export command: {}", &export.command);
        let mut metric = Command::new("sh")
            .args(&["-c", &export.command])
            .stdout(Stdio::piped())
            .spawn()?;

        trace!("Awaiting command completion");
        metric.wait()?;

        trace!("Reading response");
        let mut read_f = metric.stdout.expect("Missing stdout");
        let mut output = String::new();
        read_f.read_to_string(&mut output)?;

        let output = output.trim().replace("\n", "");
        trace!("Got value {}", output);

        let floating_output: f64 = output.parse()?;

        trace!("Creating Prometheus metrics and setting them");
        match export.r#type {
            PrometheusType::Counter => {
                let counter = Counter::new(&export.name, &export.description)?;
                counter.inc_by(floating_output);
                reg.register(Box::new(counter))?;
            },
            PrometheusType::Gauge => {
                let gauge = Gauge::new(&export.name, &export.description)?;
                gauge.set(floating_output);
                reg.register(Box::new(gauge))?;
            }
        }
    }

    trace!("Collecting metrics");
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = reg.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer)?;
    Ok(HttpResponse::Ok().body(body))
}