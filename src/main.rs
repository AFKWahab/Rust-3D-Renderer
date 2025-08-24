use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::{ValidateRect, GetStockObject, WHITE_BRUSH, HBRUSH},
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS};
use Rust_3D_Rasterizer::renderer::Renderer;

struct WindowData {
    renderer: Renderer
}
static mut RENDERER: Option<Renderer> = None;
fn main() -> Result<()> {
    unsafe {

        // Initialize renderer
        RENDERER = Some(Renderer::new(800,600));
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
            s!("This is a sample window"),
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

        let window_data = Box::new(WindowData {
            renderer: Renderer::new(800, 600),
        });

        SetWindowLongPtrA(hwnd, GWLP_USERDATA, Box::into_raw(window_data) as isize);
        // Message loop
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
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
                    window_data.renderer.render_frame();
                    // Create bitmap information for displaying the framebuffer
                    let (width, height) = window_data.renderer.get_dimension();
                    let mut bitmap_info_header = BITMAPINFOHEADER {
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
                        hdc, // "Where to draw" the windows device context
                        0, 0, // Where in the window to start drawing (0,0) = top left corner
                        width, height, // How big of an area to draw
                        0, 0, // Where in the source image to start reading (0,0) = start from top left of pixel array
                        0, height, // Which rows of pixels to use, StartScan: 0 = start from row 0, cLines: height = use all height rows
                        window_data.renderer.get_framebuffer().as_ptr() as *const _,
                        &bitmap_info,
                        DIB_RGB_COLORS,
                    );
                    ReleaseDC(Option::from(window), hdc);
                }
                ValidateRect(Option::from(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
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