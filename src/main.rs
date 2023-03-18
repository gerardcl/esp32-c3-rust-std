use anyhow::{Error, Result};
use embedded_svc::{
    http::{Headers, Method},
    io::Write,
};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
    nvs::EspDefaultNvsPartition,
};
use esp_idf_sys as _;
use std::{thread::sleep, time::Duration};
use log::*;

use esp32_c3_rust_std::modules::wifi::WifiDriver;

fn main() -> Result<(), Error> {
    esp_idf_sys::link_patches(); //Needed for esp32-rs

    info!("Starting ESP32 setup");

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = WifiDriver::new(peripherals, sys_loop, nvs);
    wifi.configure();
    wifi.start();

    // Set the HTTP server
    let mut server = EspHttpServer::new(&Configuration::default())?;

    server.fn_handler("/", Method::Get, move |request| {
        let html = render_html("world".into());
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/host", Method::Get, move |request| {
        let html = render_html(request.host().unwrap_or_default().to_string());
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    })?;

    info!("Server awaiting connection");

    // Prevent program from exiting
    loop {
        info!(
            "IP info: {:?}",
            wifi.driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::new(2, 0));
    }
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn render_html(content: String) -> String {
    templated(format!("Hello {content}!"))
}
