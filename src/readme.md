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

## Triangle rasterization
The Z-buffer (depth buffer) stores the distance from the camera for each pixel on the screen

We need this, because if we imagine that we have two trinagles:
- Triangle A at distance 5.0 from camera (further away)
- Triangle B at distance 3.0 from camera (closer)
Both triangle cover the same pixel on the screen. Which color should that pixel be?
Without Z-buffer: "Whatever triangle i drew last wins"

Besides that we also need triangle rasterization. This is used because right now we draw lines between points: A -> B, B-> C, C->A, and we get a hollow wireframe outline


Triangle rasterization fills in all the pixels inside the triangle 

### Barycentric coordinates
Barycentric coordinates describe any point inside a triangle using the triangles three vertices
If we have a triangle ABC made of three different colored lights, any point P inside the triangle gets lit by a mixture of these three lights

We need this for interpolation. once we know the barycentric coordinate of a pixel, we can smoothly blend any property across the triangle 