use crate::math::Vec2f;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, SetCursorPos, ShowCursor};
use windows::Win32::Foundation::{HWND, POINT, };
use windows::Win32::UI::Input::KeyboardAndMouse::{SetCapture, ReleaseCapture};
use windows::Win32::Graphics::Gdi::ClientToScreen;
pub const VK_W: u32 = 0x57;
pub const VK_A: u32 = 0x41;
pub const VK_S: u32 = 0x53;
pub const VK_D: u32 = 0x44;
pub const VK_SPACE: u32 = 0x20;
pub const VK_LSHIFT: u32 = 0xA0;
pub const VK_ESCAPE: u32 = 0x1B;

pub struct InputManager {
    // Keyboard state - track what's currently pressed
    keys_pressed: [bool; 256],      // Win32 virtual key codes 0-255

    // Mouse state
    mouse_delta: Vec2f,             // Movement since last frame
    mouse_sensitivity: f32,
    mouse_captured: bool,
    window_handle: Option<HWND>,    // Need this for mouse capture

    // Timing
    last_frame_time: std::time::Instant,
    delta_time: f32,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 256],
            mouse_delta: Vec2f::zero(),
            mouse_sensitivity: 1.0,
            mouse_captured: false,
            window_handle: None,
            last_frame_time: std::time::Instant::now(),
            delta_time: 0.0,
        }
    }

    pub fn set_window_handle(&mut self, hwnd: HWND) {
        self.window_handle = Some(hwnd);
    }

    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }

    // Win32 message handlers - call these from window procedure
    pub fn on_key_down(&mut self, vk_code: u32) {
        if vk_code < 256 {
            self.keys_pressed[vk_code as usize] = true;
        }

        // Handle escape key for mouse capture toggle
        if vk_code == VK_ESCAPE {
            self.toggle_mouse_capture();
        }
    }

    pub fn on_key_up(&mut self, vk_code: u32) {
        if vk_code < 256 {
            self.keys_pressed[vk_code as usize] = false;
        }
    }

    pub fn on_mouse_move(&mut self, x_delta: i32, y_delta: i32) {
        if self.mouse_captured {
            // Accumulate mouse movement
            self.mouse_delta.x += x_delta as f32;
            self.mouse_delta.y += y_delta as f32;
        }
    }

    // Query methods for game logic
    pub fn is_key_pressed(&self, vk_code: u32) -> bool {
        if vk_code < 256 {
            self.keys_pressed[vk_code as usize]
        } else {
            false
        }
    }

    pub fn is_mouse_captured(&self) -> bool {
        self.mouse_captured
    }

    // Win32-specific mouse capture implementation
    pub fn toggle_mouse_capture(&mut self) {
        if let Some(hwnd) = self.window_handle {
            unsafe {
                if self.mouse_captured {
                    // Release mouse capture
                    ReleaseCapture();
                    ShowCursor(true);
                    self.mouse_captured = false;
                } else {
                    // Capture mouse
                    SetCapture(hwnd);
                    ShowCursor(false);
                    self.mouse_captured = true;

                    // Center cursor in window and reset delta
                    let mut rect = Default::default();
                    if GetClientRect(hwnd, &mut rect).is_ok() {
                        let center_x = rect.right / 2;
                        let center_y = rect.bottom / 2;

                        // Convert client coordinates to screen coordinates
                        let mut point = POINT {
                            x: center_x,
                            y: center_y
                        };
                        if ClientToScreen(hwnd, &mut point).as_bool() {
                            SetCursorPos(point.x, point.y);
                        }
                    }

                    // Reset mouse delta when starting capture
                    self.mouse_delta = Vec2f::zero();
                }
            }
        }
    }

    pub fn force_release_capture(&mut self) {
        if self.mouse_captured {
            unsafe {
                ReleaseCapture();
                ShowCursor(true);
            }
            self.mouse_captured = false;
        }
    }

    // These methods you'll implement with your own logic
    pub fn get_mouse_delta(&mut self) -> Vec2f {
        // TODO: Implement - should return mouse movement and reset internal delta
        Vec2f::zero()
    }

    pub fn get_delta_time(&self) -> f32 {
        // TODO: Implement - return time since last frame
        self.delta_time
    }

    pub fn update(&mut self) {
        // TODO: Implement - calculate delta time, handle any per-frame logic
    }
}