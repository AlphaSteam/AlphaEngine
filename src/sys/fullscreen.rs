use glutin::{monitor::MonitorHandle, window::Fullscreen as glutinFullscreen};

#[derive(Debug)]
pub enum Fullscreen {
    Borderless,
    Exclusive,
    False,
}

impl Fullscreen {
    pub fn value(&self, monitor_handle: Option<MonitorHandle>) -> Option<glutinFullscreen> {
        match *self {
            Fullscreen::Borderless => Some(glutinFullscreen::Borderless(None)),
            Fullscreen::Exclusive => {
                let video_modes = MonitorHandle::video_modes(&monitor_handle.unwrap());
                for _video_mode in video_modes {
                    //println!("{:?}", video_mode);
                }
                println!("Exclusive fullscreen not yet implemented, defaulting to borderless fullscreen.");
                Some(glutinFullscreen::Borderless(None))
                //Some(glutinFullscreen::Exclusive(VideoMode {video_mode:
            }
            Fullscreen::False => None,
        }
    }
}
