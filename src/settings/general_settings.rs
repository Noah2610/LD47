// resources/settings/general.ron

#[derive(Deserialize, Clone)]
pub struct GeneralSettings {
    pub text_scroll_delay_ms: u64,
}
