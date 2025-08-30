use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::{GetStockObject, ValidateRect, HBRUSH, WHITE_BRUSH},
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};
use Rust_3D_Rasterizer::lighting::Light;
use Rust_3D_Rasterizer::math::Vec3f;
use Rust_3D_Rasterizer::renderer::Renderer;
use Rust_3D_Rasterizer::scene::Scene;

struct WindowData {
    renderer: Renderer,
    scene: Scene,
}

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

        // Main directional light (key light)
        scene.add_light(Light::directional(
            Vec3f::new(-0.5, -1.0, -0.5), // Coming from upper left
            Vec3f::new(1.0, 0.9, 0.8),    // Warm white
            0.8
        ));

        // Fill light (softer, cooler)
        scene.add_light(Light::directional(
            Vec3f::new(0.5, 0.0, -1.0),   // Coming from right
            Vec3f::new(0.6, 0.7, 1.0),    // Cool blue
            0.4
        ));

        // Point light for additional interest
        scene.add_light(Light::point(
            Vec3f::new(0.0, 4.0, 2.0),    // Above and behind
            Vec3f::new(1.0, 0.5, 0.2),    // Orange
            2.0,                          // Bright
            10.0                          // Range
        ));

        // Spot light for dramatic shadows
        scene.add_light(Light::spot(
            Vec3f::new(-4.0, 3.0, 4.0),   // Position
            Vec3f::new(1.0, -0.5, -1.0).normalize(), // Direction
            Vec3f::new(0.9, 0.2, 0.9),    // Purple
            3.0,                          // Intensity
            15.0,                         // Range
            std::f32::consts::PI / 6.0,   // Inner angle (30°)
            std::f32::consts::PI / 4.0    // Outer angle (45°)
        ));

        let window_data = Box::new(WindowData {
                renderer,
                scene,
        });

        SetWindowLongPtrA(hwnd, GWLP_USERDATA, Box::into_raw(window_data) as isize);

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
            WM_PAINT => {
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    let window_data = &mut *window_data_ptr;

                    // Update scene (this will rotate the cubes)
                    window_data.scene.update(0.016); // Assume ~60 FPS

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
                let window_data_ptr = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowData;
                if !window_data_ptr.is_null() {
                    let _ = Box::from_raw(window_data_ptr); // This will drop and clean up
                }
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}