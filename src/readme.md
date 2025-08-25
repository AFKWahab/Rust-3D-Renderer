# Wín32 API
- Every application has a WinMain entry-point function
- Every application must create at least one window
- Every window belongs to one window class
- Every window has one unique window handle
- Every window has one GUI thread
- Every GUI thread has one message queue
- Every window class contains one window procedure

Extra information: Windows is a message-based system, where:

- Every message sent or posted to a window is processed by its window procedure
- Every message posted to a window is placed in its message queue
- Every window must remove and process messages posted to its message queue



## Documentation:

General information about the RUST API in 
https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html#what-are-windows
https://learn.microsoft.com/en-us/windows/win32/learnwin32/your-first-windows-program

Cursor information:

https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors  

Window procedure & Windows procedure messages:

https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc
https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues#system-defined-messages


# Pixel Rendering Documentation

# TODO (Short & Long term)
## Convert to Vulkan
Right now i run with this structure:
- CPU calculates pixels → Framebuffer in RAM → Win32 blits to screen
But we would probably have to use VULKAN to have GPU access
- CPU sends commands to GPU → GPU calculates pixels → GPU renders directly to screen
