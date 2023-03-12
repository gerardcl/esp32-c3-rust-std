use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _;
use std::{thread::sleep, time::Duration};

use esp32_c3_rust_std::modules::wifi::WifiDriver;

fn main() {
    esp_idf_sys::link_patches(); //Needed for esp32-rs

    println!("Starting ESP32 setup");

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = WifiDriver::new(peripherals, sys_loop, nvs);
    wifi.configure();
    wifi.start();

    loop {
        println!(
            "IP info: {:?}",
            wifi.driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::new(10, 0));
    }
}
