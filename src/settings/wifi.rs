#[toml_cfg::toml_config]
pub struct WifiConfig {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}
