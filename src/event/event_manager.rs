use glutin::dpi::PhysicalPosition;
use glutin::event::DeviceId;
use glutin::event::ElementState;
use glutin::event::KeyboardInput;
use glutin::event::MouseButton;
use glutin::event::MouseScrollDelta;
#[derive(Debug)]
pub struct EventManager {
    key: fn(key: KeyboardInput, device_id: DeviceId),
    device_added: fn(device_id: DeviceId),
    device_removed: fn(device_id: DeviceId),
    mouse_motion: fn(delta: (f64, f64), device_id: DeviceId),
    mouse_wheel: fn(delta: MouseScrollDelta, device_id: DeviceId),
    motion: fn(axis: u32, value: f64, device_id: DeviceId),
    button: fn(button: u32, state: ElementState, device_id: DeviceId),
    text: fn(codepoint: char, device_id: DeviceId),
    mouse_input: fn(state: ElementState, button: MouseButton, device_id: DeviceId),
    cursor_moved: fn(position: PhysicalPosition<f64>, device_id: DeviceId),
    axis_motion: fn(axis: u32, value: f64, device_id: DeviceId),
}
impl EventManager {
    pub fn new() -> Self {
        fn key(_key: KeyboardInput, _device_id: DeviceId) {}
        fn device_added(_device_id: DeviceId) {}
        fn device_removed(_device_id: DeviceId) {}
        fn mouse_motion(_delta: (f64, f64), _device_id: DeviceId) {}
        fn mouse_wheel(_delta: MouseScrollDelta, _device_id: DeviceId) {}
        fn motion(_axis: u32, _value: f64, _device_id: DeviceId) {}
        fn button(_button: u32, _state: ElementState, _device_id: DeviceId) {}
        fn text(_codepoint: char, _device_id: DeviceId) {}
        fn mouse_input(_state: ElementState, _button: MouseButton, _device_id: DeviceId) {}
        fn cursor_moved(_position: PhysicalPosition<f64>, _device_id: DeviceId) {}
        fn axis_motion(_axis: u32, _value: f64, _device_id: DeviceId) {}
        EventManager {
            key,
            device_added,
            device_removed,
            mouse_motion,
            mouse_wheel,
            motion,
            button,
            text,
            mouse_input,
            cursor_moved,
            axis_motion,
        }
    }
    pub fn set_key_callback(&mut self, callback: fn(key: KeyboardInput, device_id: DeviceId)) {
        self.key = callback;
    }
    pub fn run_key_callback(&self, key: KeyboardInput, device_id: DeviceId) {
        (self.key)(key, device_id);
    }
    pub fn set_device_added_callback(&mut self, callback: fn(device_id: DeviceId)) {
        self.device_added = callback;
    }
    pub fn run_device_added_callback(&self, device_id: DeviceId) {
        (self.device_added)(device_id);
    }
    pub fn set_device_removed_callback(&mut self, callback: fn(device_id: DeviceId)) {
        self.device_removed = callback;
    }
    pub fn run_device_removed_callback(&self, device_id: DeviceId) {
        (self.device_removed)(device_id);
    }

    pub fn set_mouse_motion_callback(
        &mut self,
        callback: fn(delta: (f64, f64), device_id: DeviceId),
    ) {
        self.mouse_motion = callback;
    }
    pub fn run_mouse_motion_callback(&self, delta: (f64, f64), device_id: DeviceId) {
        (self.mouse_motion)(delta, device_id);
    }

    pub fn set_mouse_wheel_callback(
        &mut self,
        callback: fn(delta: MouseScrollDelta, device_id: DeviceId),
    ) {
        self.mouse_wheel = callback;
    }
    pub fn run_mouse_wheel_callback(&self, delta: MouseScrollDelta, device_id: DeviceId) {
        (self.mouse_wheel)(delta, device_id);
    }

    pub fn set_motion_callback(
        &mut self,
        callback: fn(axis: u32, value: f64, device_id: DeviceId),
    ) {
        self.motion = callback;
    }
    pub fn run_motion_callback(&self, axis: u32, value: f64, device_id: DeviceId) {
        (self.motion)(axis, value, device_id);
    }

    pub fn set_button_callback(
        &mut self,
        callback: fn(button: u32, state: ElementState, device_id: DeviceId),
    ) {
        self.button = callback;
    }
    pub fn run_button_callback(&self, button: u32, state: ElementState, device_id: DeviceId) {
        (self.button)(button, state, device_id);
    }
    pub fn set_text_callback(&mut self, callback: fn(codepoint: char, device_id: DeviceId)) {
        self.text = callback;
    }
    pub fn run_text_callback(&self, codepoint: char, device_id: DeviceId) {
        (self.text)(codepoint, device_id);
    }
    pub fn set_mouse_input_callback(
        &mut self,
        callback: fn(state: ElementState, button: MouseButton, device_id: DeviceId),
    ) {
        self.mouse_input = callback;
    }
    pub fn run_mouse_input_callback(
        &self,
        state: ElementState,
        button: MouseButton,
        device_id: DeviceId,
    ) {
        (self.mouse_input)(state, button, device_id);
    }

    pub fn set_cursor_moved_callback(
        &mut self,
        callback: fn(position: PhysicalPosition<f64>, device_id: DeviceId),
    ) {
        self.cursor_moved = callback;
    }
    pub fn run_cursor_moved_callback(&self, position: PhysicalPosition<f64>, device_id: DeviceId) {
        (self.cursor_moved)(position, device_id);
    }

    pub fn set_axis_motion_callback(
        &mut self,
        callback: fn(axis: u32, value: f64, device_id: DeviceId),
    ) {
        self.axis_motion = callback;
    }
    pub fn run_axis_motion_callback(&self, axis: u32, value: f64, device_id: DeviceId) {
        (self.axis_motion)(axis, value, device_id);
    }
}
