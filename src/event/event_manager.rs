use glutin::event::DeviceEvent;
use glutin::event::KeyboardInput;
#[derive(Debug)]
pub struct EventManager {
    key_callback: fn(key: KeyboardInput, event: DeviceEvent),
    device_added: fn(event: DeviceEvent),
}
impl EventManager {
    pub fn new() -> Self {
        fn key_callback(_key: KeyboardInput, event: DeviceEvent) {}
        fn device_added(_event: DeviceEvent) {}
        EventManager {
            key_callback,
            device_added,
        }
    }
    pub fn set_key_callback(&mut self, callback: fn(key: KeyboardInput, event: DeviceEvent)) {
        self.key_callback = callback;
    }
    pub fn run_key_callback(&self, key: KeyboardInput, event: DeviceEvent) {
        (self.key_callback)(key, event);
    }
    pub fn run_device_added_callback(&self, event: DeviceEvent) {
        (self.device_added)(event);
    }
}
