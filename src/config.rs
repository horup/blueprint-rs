#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Debug {
    pub show_fps:bool,
    pub show_sprite_bounds:bool,
    pub show_mouse_state:bool
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            show_fps:true,
            show_sprite_bounds:false,
            show_mouse_state:true
        }
    }
}

pub struct Config {
    pub tick_rate_ps:u32,
    pub window_title:String,
    pub width:f32,
    pub height:f32,
    pub debug:Debug
}
impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ps:20,
            window_title:String::from("Blueprint"),
            width:0.0,
            height:0.0,
            debug:Debug::default()
        }
    }
}