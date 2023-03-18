use embedded_svc::{
    http::{Headers, Method},
    io::Write,
};
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use log::info;

use super::template::render_html;

pub struct HttpServer {
    server: EspHttpServer,
}

impl HttpServer {
    // Set the HTTP server
    pub fn new() -> HttpServer {
        info!("Server creating");

        HttpServer {
            server: EspHttpServer::new(&Configuration::default()).unwrap(),
        }
    }

    pub fn set_handlers(&mut self) {
        info!("Server handlers setup");

        self.server
            .fn_handler("/", Method::Get, move |request| {
                let html = render_html("world");
                let mut response = request.into_ok_response()?;
                response.write_all(html.as_bytes())?;
                Ok(())
            })
            .unwrap();

        self.server
            .fn_handler("/host", Method::Get, move |request| {
                let html = render_html(request.host().unwrap_or_default());
                let mut response = request.into_ok_response()?;
                response.write_all(html.as_bytes())?;
                Ok(())
            })
            .unwrap();
    }
}
