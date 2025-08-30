use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, InvalidateRect};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::{GetStockObject, ValidateRect, HBRUSH, WHITE_BRUSH},
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use Rust_3D_Rasterizer::lighting::Light;
use Rust_3D_Rasterizer::math::Vec3f;
use Rust_3D_Rasterizer::renderer::Renderer;
use Rust_3D_Rasterizer::scene::Scene;
use Rust_3D_Rasterizer::input::{InputManager, VK_W, VK_A, VK_S, VK_D, VK_SPACE, VK_LSHIFT};

struct WindowData {
    renderer: Renderer,
    scene: Scene,
    input: InputManager,
}

// tiny helpers to extract x/y from LPARAM (avoids missing GET_X/Y_LPARAM)
#[inline]
fn lparam_get_x(lp: LPARAM) -> i32 {
    (lp.0 as u32 & 0xFFFF) as i16 as i32
}
#[inline]
fn lparam_get_y(lp: LPARAM) -> i32 {
    ((lp.0 as u32 >> 16) & 0xFFFF) as i16 as i32
}

// frame timer constants
const FRAME_TIMER_ID: usize = 1;
const FRAME_TIMER_MS: u32 = 1;

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        if atom == 0 {
            return Err(Error::from_win32());
        }

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Adam Game Engine"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Some(instance.into()),
            None,
        )?;

        if hwnd.0.is_null() {
            return Err(Error::from_win32())
        }

        // Create renderer and scene
        let renderer = Renderer::new(800, 600);
        let mut scene = Scene::new();

        // Add multiple cubes with different positions
        scene.add_cube_at(Vec3f::new(-2.0, 0.0, 0.0));
        scene.add_cube_at(Vec3f::new(2.0, 0.0, 0.0));
        scene.add_cube_at(Vec3f::new(0.0, 2.0, -2.0));

        // Add multiple lights for dramatic effect
        scene.add_light(Light::directional(
            Vec3f::new(-0.5, -1.0, -0.5),
            Vec3f::new(1.0, 0.9, 0.8),
            0.8
        ));
        scene.add_light(Light::directional(
            Vec3f::new(0.5, 0.0, -1.0),
            Vec3f::new(0.6, 0.7, 1.0),
            0.4
        ));
        scene.add_light(Light::point(
            Vec3f::new(0.0, 4.0, 2.0),
            Vec3f::new(1.0, 0.5, 0.2),
            2.0,
            10.0
        ));
        scene.add_light(Light::spot(
            Vec3f::new(-4.0, 3.0, 4.0),
            Vec3f::new(1.0, -0.5, -1.0).normalize(),
            Vec3f::new(0.9, 0.2, 0.9),
            3.0,
            15.0,
            std::f32::consts::PI / 6.0,
            std::f32::consts::PI / 4.0
        ));

        // set up input (attach window handle + sensitivity)
        let mut input = InputManager::new();
        input.set_window_handle(hwnd);
        input.set_mouse_sensitivity(1.0);

        let window_data = Box::new(WindowData {
            renderer,
            scene,
            input,
        });

        SetWindowLongPtrA(hwnd, GWLP_USERDATA, Box::into_raw(window_data) as isize);

        // start a fast frame timer to drive updates + repaints
        SetTimer(Option::from(hwnd), FRAME_TIMER_ID, FRAME_TIMER_MS, None);

        // Message loop
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            // key events → InputManager
            WM_KEYDOWN => {
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    (*window_data_ptr).input.on_key_down(wparam.0 as u32);
                }
                LRESULT(0)
            }
            WM_KEYUP => {
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    (*window_data_ptr).input.on_key_up(wparam.0 as u32);
                }
                LRESULT(0)
            }

            // relative mouse movement + recenter when captured
            WM_MOUSEMOVE => {
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    let wd = &mut *window_data_ptr;
                    if wd.input.is_mouse_captured() {
                        let x = lparam_get_x(lparam);
                        let y = lparam_get_y(lparam);

                        let mut rect = RECT::default();
                        if GetClientRect(window, &mut rect).is_ok() {
                            let cx = (rect.right - rect.left) / 2;
                            let cy = (rect.bottom - rect.top) / 2;

                            let dx = x - cx;
                            let dy = y - cy;

                            wd.input.on_mouse_move(dx, dy);

                            // warp cursor back to center (client -> screen)
                            let mut p = POINT { x: cx, y: cy };
                            if ClientToScreen(window, &mut p).as_bool() {
                                SetCursorPos(p.x, p.y);
                            }
                        }
                    }
                }
                LRESULT(0)
            }

            // frame tick — update input, move camera, update scene, then repaint
            WM_TIMER => {
                if wparam.0 == FRAME_TIMER_ID {
                    let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                    if !window_data_ptr.is_null() {
                        let wd = &mut *window_data_ptr;

                        // compute delta time
                        wd.input.update();
                        let dt = wd.input.get_delta_time();

                        // WASD + up/down (units/second)
                        let speed = 3.5_f32;
                        if wd.input.is_key_pressed(VK_W) {
                            wd.scene.camera.move_forward(speed * dt);
                        }
                        if wd.input.is_key_pressed(VK_S) {
                            wd.scene.camera.move_forward(-speed * dt);
                        }
                        if wd.input.is_key_pressed(VK_A) {
                            wd.scene.camera.move_right(-speed * dt);
                        }
                        if wd.input.is_key_pressed(VK_D) {
                            wd.scene.camera.move_right(speed * dt);
                        }
                        if wd.input.is_key_pressed(VK_SPACE) {
                            wd.scene.camera.move_up(speed * dt);
                        }
                        if wd.input.is_key_pressed(VK_LSHIFT) {
                            wd.scene.camera.move_up(-speed * dt);
                        }

                        // mouse-look (in radians), using your camera API
                        if wd.input.is_mouse_captured() {
                            let md = wd.input.get_mouse_delta(); // scaled by sensitivity
                            let yaw_delta = md.x * 0.002;
                            let pitch_delta = -md.y * 0.002;

                            let fwd = wd.scene.camera.get_forward_vector();
                            let dist = 1.0;
                            let mut yaw = fwd.z.atan2(fwd.x);
                            let mut pitch = (fwd.y / fwd.length()).asin();

                            let half_pi = std::f32::consts::FRAC_PI_2;
                            yaw += yaw_delta;
                            pitch = (pitch + pitch_delta).clamp(-half_pi + 0.001, half_pi - 0.001);

                            let new_dir = Vec3f::new(
                                dist * pitch.cos() * yaw.cos(),
                                dist * pitch.sin(),
                                dist * pitch.cos() * yaw.sin(),
                            );
                            wd.scene.camera.look_in_direction(new_dir);
                        }

                        // animate scene (rotations etc.)
                        wd.scene.update(dt);

                        // request repaint
                        InvalidateRect(Some(window), None, false);
                    }
                }
                LRESULT(0)
            }

            WM_PAINT => {
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    let window_data = &mut *window_data_ptr;

                    // Render the scene
                    window_data.scene.render(&mut window_data.renderer);

                    // Display the framebuffer
                    let (width, height) = window_data.renderer.get_dimension();
                    let bitmap_info_header = BITMAPINFOHEADER {
                        biSize: size_of::<BITMAPINFOHEADER>() as u32,
                        biWidth: width as i32,
                        biHeight: -(height as i32),
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: 0,
                        ..Default::default()
                    };
                    let bitmap_info = BITMAPINFO {
                        bmiHeader: bitmap_info_header,
                        ..Default::default()
                    };

                    let hdc = GetDC(Option::from(window));
                    SetDIBitsToDevice(
                        hdc,
                        0, 0,
                        width, height,
                        0, 0,
                        0, height,
                        window_data.renderer.get_framebuffer().as_ptr() as *const _,
                        &bitmap_info,
                        DIB_RGB_COLORS,
                    );
                    ReleaseDC(Option::from(window), hdc);
                }
                let _ = ValidateRect(Option::from(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                // stop timer
                KillTimer(Option::from(window), FRAME_TIMER_ID);

                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    let _ = Box::from_raw(window_data_ptr); // drop & clean up
                }
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
