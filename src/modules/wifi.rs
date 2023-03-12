use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    nvs::{EspNvsPartition, NvsDefault},
    wifi::EspWifi,
};

use crate::settings::wifi::CONFIG as wifi_config;

pub struct WifiDriver<'a> {
    pub driver: EspWifi<'a>,
}

impl WifiDriver<'_> {
    pub fn new(
        peripherals: Peripherals,
        sys_loop: EspEventLoop<System>,
        nvs: EspNvsPartition<NvsDefault>,
    ) -> WifiDriver<'static> {
        WifiDriver {
            driver: EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap(),
        }
    }

    pub fn configure(&mut self) {
        self.driver
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid: wifi_config.wifi_ssid.into(),
                password: wifi_config.wifi_psk.into(),
                ..Default::default()
            }))
            .unwrap();
    }

    pub fn start(&mut self) {
        self.driver.start().unwrap();
        self.driver.connect().unwrap();
        while !self.driver.is_connected().unwrap() {
            let config = self.driver.get_configuration().unwrap();
            println!(
                "Waiting for station SSID: {} on Channel #{:?}, with AuthMethod: {}, on BSSID: {:?}",
                config.as_client_conf_ref().unwrap().ssid,
                config.as_client_conf_ref().unwrap().channel,
                config.as_client_conf_ref().unwrap().auth_method,
                config.as_client_conf_ref().unwrap().bssid
            )
        }
        println!("Should be connected now");
    }
}
