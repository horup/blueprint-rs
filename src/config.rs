pub struct Config {
    pub tick_rate_ps:u32,
    pub window_title:String,
    pub width:f32,
    pub height:f32
}
impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ps:20,
            window_title:String::from("Blueprint"),
            width:0.0,
            height:0.0
        }
    }
}