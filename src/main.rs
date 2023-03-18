use anyhow::{Error, Result};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _;
use log::*;
use std::{thread::sleep, time::Duration};

use esp32_c3_rust_std::modules::wifi::WifiDriver;
use esp32_c3_rust_std::server::http::HttpServer;

fn main() -> Result<(), Error> {
    esp_idf_sys::link_patches(); //Needed for esp32-rs

    info!("Starting ESP32 C3 setup");

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // wifi module as client
    let mut wifi = WifiDriver::new(peripherals, sys_loop, nvs);
    wifi.configure();
    wifi.start();

    // server example for accepting HTTP requests
    let mut server = HttpServer::new();
    server.set_handlers();

    info!("Server accepting connections");

    // Prevent program from exiting
    loop {
        info!(
            "IP info: {:?}",
            wifi.driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::new(2, 0));
    }
}
