pub struct Config {
    pub tick_rate_ps:u32,
    pub window_title:String
}
impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ps:20,
            window_title:String::from("Blueprint")
        }
    }
}