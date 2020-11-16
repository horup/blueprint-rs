pub struct Config {
    pub tick_rate_ps:u32
}
impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ps:20
        }
    }
}