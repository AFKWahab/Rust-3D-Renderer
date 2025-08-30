# Adam Game Engine - 3D Software Rasterizer

A complete software-based 3D rendering engine built from scratch in Rust, demonstrating the fundamentals of 3D graphics programming without relying on GPU acceleration or graphics libraries.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [3D Graphics Pipeline](#3d-graphics-pipeline)
5. [Mathematics Foundation](#mathematics-foundation)
6. [Lighting System](#lighting-system)
7. [Rendering Pipeline](#rendering-pipeline)
8. [Win32 Integration](#win32-integration)
9. [Features](#features)
10. [Technical Implementation](#technical-implementation)
11. [Performance Considerations](#performance-considerations)
12. [TODO-List](#long-term-goals)
13. [Future Improvements](#future-improvements)
14. [References](#references)

## Overview

This engine implements a complete 3D software rasterizer that transforms 3D vector geometry into 2D pixel arrays displayed on screen. The entire rendering pipeline runs on the CPU, providing educational insight into how modern GPU-accelerated graphics work under the hood.

**Key Achievement**: Real-time rendering of multiple 3D objects with advanced lighting effects, all computed in software.

## Architecture

The engine follows a modular architecture with clear separation of concerns:

```
src/
├── math/           # Linear algebra foundation
│   ├── vec2.rs     # 2D vector operations
│   ├── vec3.rs     # 3D vector operations (positions, directions, colors)
│   ├── vec4.rs     # 4D vectors for homogeneous coordinates
│   └── matrix.rs   # 4x4 matrix transformations
├── camera.rs       # Virtual camera system
├── lighting.rs     # Advanced lighting calculations
├── mesh.rs         # 3D geometry representation
├── renderer.rs     # Rasterization and pixel rendering
├── scene.rs        # Scene management and rendering coordination
└── main.rs         # Win32 API integration and main loop
```

## Core Components

### Mathematics Library (`math/`)

The mathematical foundation implements essential linear algebra operations:

- **Vec3f**: Represents 3D positions, directions, and colors with full operator overloading
- **Vec4f**: Homogeneous coordinates for perspective projection
- **Mat4x4**: 4x4 matrices for all 3D transformations (translation, rotation, scaling, projection)

**Key Features**:
- Proper homogeneous coordinate handling
- Component-wise vector operations
- Matrix inversion using Gaussian elimination
- Cross and dot product operations

### Camera System (`camera.rs`)

Implements a perspective camera with:
- **View Matrix**: Transforms world coordinates to camera space using look-at calculations
- **Projection Matrix**: Perspective projection with configurable FOV, aspect ratio, and clipping planes
- **Camera Controls**: Forward/backward movement, strafing, and orbital rotation around targets

**Mathematical Foundation**:
The camera system solves the fundamental problem of 3D graphics: converting world coordinates (where objects exist) to camera coordinates (how they appear from the viewer's perspective).

### Mesh System (`mesh.rs`)

Defines 3D geometry through indexed triangle meshes:
- **Vertices**: Array of 3D positions
- **Triangles**: Indices into vertex array with material properties
- **Primitive Creation**: Built-in cube and triangle generators
- **Normal Calculation**: Automatic surface normal computation for lighting

**Winding Order**: All triangles use counter-clockwise winding when viewed from outside, ensuring correct backface culling.

## 3D Graphics Pipeline

The engine implements the complete 3D graphics pipeline:

1. **Model Space** → **World Space**: Object transformations (position, rotation, scale)
2. **World Space** → **Camera Space**: View transformation
3. **Camera Space** → **Clip Space**: Perspective projection
4. **Clip Space** → **Screen Space**: Viewport transformation
5. **Rasterization**: Convert triangles to pixels
6. **Fragment Processing**: Lighting calculations and depth testing

### Coordinate System Transformations

```
Object Vertices → [Model Matrix] → World Coordinates
                ↓ [View Matrix]
Camera Coordinates → [Projection Matrix] → Clip Coordinates
                    ↓ [Perspective Divide + Viewport]
                Screen Pixels
```

## Mathematics Foundation

### Vector Operations

**Position Vectors**: Represent specific locations in 3D space (x,y,z) relative to an origin. Every vertex in 3D models is defined by a position vector.

**Direction Vectors**: Represent orientation and movement without inherent position. Used for surface normals, light directions, camera look-at vectors, and movement calculations.

**Key Operations**:
- **Dot Product**: Determines lighting intensity by measuring angles between surface normals and light directions
- **Cross Product**: Generates surface normals from triangle edges and enables backface culling
- **Normalization**: Ensures direction vectors have unit length for consistent calculations

### Matrix Transformations

4x4 matrices handle all spatial transformations in homogeneous coordinates:

```
[Rxx  Rxy  Rxz  Tx]   [x]   [x']
[Ryx  Ryy  Ryz  Ty] × [y] = [y']
[Rzx  Rzy  Rzz  Tz]   [z]   [z']
[ 0    0    0   1 ]   [1]   [1 ]
```

- **Upper 3x3**: Rotation and scaling transformations
- **Right Column**: Translation (position)
- **Bottom Row**: Homogeneous coordinate enabler

**Why Matrix Inversion?**
To render 3D objects on a 2D screen, we need to convert world coordinates (where objects are) to camera coordinates (how they appear from the camera's perspective). The camera-to-world matrix describes "how to move the camera to its position," but for rendering we need the inverse: "how to move the world relative to the camera."

## Lighting System

### Advanced Lighting Model

The lighting system implements physically-based lighting with multiple light types:

#### Light Types

1. **Directional Lights**: Simulate distant light sources (sun) with parallel rays
2. **Point Lights**: Omnidirectional lights with distance-based attenuation
3. **Spot Lights**: Cone-shaped lights with inner/outer angle falloff

#### Lighting Calculations

**Diffuse Lighting (Lambert)**:
```
diffuse = max(0, normal · light_direction) × light_intensity
```

**Specular Lighting (Blinn-Phong)**:
```
half_vector = normalize(light_direction + view_direction)
specular = pow(max(0, normal · half_vector), shininess)
```

**Distance Attenuation**:
```
attenuation = 1.0 / (1.0 + 0.1×distance + 0.01×distance²)
```

#### Material System

Each surface defines:
- **Diffuse Color**: Base surface color
- **Specular Color**: Reflection highlight color
- **Specular Power**: Surface shininess (1-128)
- **Ambient Factor**: Ambient light contribution

### Lighting Features

- **Multiple Light Accumulation**: Combines lighting from all active lights
- **Backface Culling**: Skips triangles facing away from camera for performance
- **Normal Transformation**: Correctly transforms normals with object rotations
- **Ambient Lighting**: Global illumination simulation

## Rendering Pipeline

### Rasterization Process

The renderer converts 3D triangles into pixels through:

1. **Triangle Setup**: Transform vertices to screen space
2. **Bounding Box**: Calculate pixel region containing triangle
3. **Barycentric Coordinates**: Determine if pixels are inside triangle
4. **Depth Interpolation**: Calculate Z-depth for each pixel
5. **Depth Testing**: Z-buffer prevents drawing hidden surfaces
6. **Pixel Shading**: Apply lighting calculations

### Barycentric Coordinate System

For each pixel P inside triangle with vertices A, B, C:
```
P = u×A + v×B + w×C  where u + v + w = 1
```
This enables:
- **Inside/Outside Testing**: All weights ≥ 0 means pixel is inside
- **Depth Interpolation**: Smooth Z-value calculation across triangle
- **Attribute Interpolation**: Could extend to texture coordinates, colors, etc.

### Z-Buffer Algorithm

Prevents hidden surface artifacts:
```rust
for each pixel in triangle {
    interpolated_depth = u×z₀ + v×z₁ + w×z₂
    if interpolated_depth < z_buffer[pixel] {
        z_buffer[pixel] = interpolated_depth
        draw_pixel(pixel, color)
    }
}
```

## Win32 Integration

### Message-Based Architecture

Windows uses an event-driven model:
- **Window Class**: Defines window behavior and appearance
- **Window Procedure**: Handles all window messages (paint, input, close)
- **Message Loop**: Continuously processes queued messages
- **Device Context**: Provides drawing surface for pixel output

### Framebuffer Display

The engine maintains a CPU-side framebuffer (array of ARGB pixels) and uses `SetDIBitsToDevice` to efficiently blit the entire buffer to the window each frame.

**Current Architecture**:
```
CPU calculates pixels → Framebuffer in RAM → Win32 blits to screen
```

**Future GPU Architecture**:
```
CPU sends commands to GPU → GPU calculates pixels → GPU renders directly to screen
```

## Features

### Implemented Features

- ✅ **Software Rasterization**: Complete triangle rasterization with depth testing
- ✅ **3D Transformations**: Model, view, and projection matrix transforms
- ✅ **Advanced Lighting**: Diffuse, specular, and ambient lighting models
- ✅ **Multiple Light Types**: Directional, point, and spot lights with attenuation
- ✅ **Material System**: Configurable surface properties
- ✅ **Backface Culling**: Automatic hidden triangle removal
- ✅ **Z-Buffer**: Proper depth sorting and hidden surface removal
- ✅ **Real-time Rendering**: Smooth animation at ~60 FPS

### Visual Effects

- **Dynamic Lighting**: Multiple colored lights create realistic surface illumination
- **Specular Highlights**: Shiny surfaces reflect light sources
- **Smooth Rotation**: Objects rotate continuously with proper lighting updates
- **Depth Perception**: Z-buffer ensures correct object ordering

## Technical Implementation

### Performance Optimizations

1. **Barycentric Rasterization**: Efficient inside/outside triangle testing
2. **Bounding Box Culling**: Only process pixels within triangle bounds
3. **Backface Culling**: Skip triangles facing away from camera
4. **Early Z-Testing**: Depth test before expensive lighting calculations
5. **View Frustum Culling**: Skip objects outside camera view

### Memory Management

- **Static Allocation**: Fixed-size framebuffer and Z-buffer
- **Object Pooling**: Reuse transformation matrices where possible
- **Efficient Data Structures**: Contiguous arrays for cache-friendly access

### Numerical Stability

- **Normalized Vectors**: All direction vectors maintain unit length
- **Matrix Conditioning**: Gaussian elimination with partial pivoting
- **Depth Range**: Proper near/far plane handling prevents Z-fighting

## Performance Considerations

### Current Limitations

**CPU Bottleneck**: All calculations run on CPU, limiting triangle throughput
- Modern GPUs: ~10 billion triangles/second
- This engine: ~100,000 triangles/second

**Single-Threaded**: No parallelization of rasterization process

### Optimization Potential

1. **Multi-threading**: Parallelize triangle rasterization across CPU cores
2. **SIMD Instructions**: Vectorize math operations using AVX/SSE
3. **Tile-Based Rendering**: Process screen regions independently
4. **Level-of-Detail**: Reduce triangle count for distant objects
5. **Frustum Culling**: Skip entire objects outside camera view

## TODO List

### Foundation (Immediate Priorities)

#### Input System
- [x] WASD camera movement with proper delta time
- [x] Mouse look with configurable sensitivity settings
- [x] Keyboard state management for smooth movement
- [x] Mouse capture/release toggle (escape key)

#### Asset Loading
- [ ] OBJ file parser for loading 3D models
- [ ] Basic model validation and error handling
- [ ] Support for multiple meshes per file
- [ ] Material (.mtl) file parsing
- [ ] Binary asset format for faster loading

#### Scene Management
- [ ] Transform hierarchies (parent/child relationships)
- [ ] Component system architecture
- [ ] Object spawning/destruction at runtime
- [ ] Scene serialization/deserialization
- [ ] Basic scene graph optimization

### Core Engine Features

#### Rendering Improvements
- [ ] Texture mapping with UV coordinate interpolation
- [ ] Wireframe rendering mode for debugging
- [ ] Configurable render modes (solid, wireframe, points)
- [ ] Basic anti-aliasing (supersampling)
- [ ] Mipmapping for texture filtering

#### Performance Optimizations
- [ ] Multi-threaded rasterization across CPU cores
- [ ] Frustum culling (skip objects outside camera view)
- [ ] Level-of-detail (LOD) system for distant objects
- [ ] Object pooling for frequently created/destroyed objects
- [ ] SIMD optimizations for math operations

#### Math Library Extensions
- [ ] Quaternion rotations (smoother than Euler angles)
- [ ] Proper inverse transpose matrix for normal transformations
- [ ] Ray-casting for mouse picking and selection
- [ ] Collision detection primitives (AABB, sphere, OBB)
- [ ] Spatial partitioning (octree or BSP tree)

### Phase 3: Advanced Graphics Features

#### Lighting Enhancements
- [ ] Shadow mapping for directional lights
- [ ] Point light shadow mapping (cube maps)
- [ ] Light volume culling and batching
- [ ] Volumetric lighting effects
- [ ] HDR rendering with tone mapping

#### Post-Processing Pipeline
- [ ] Gamma correction and linear color space
- [ ] Basic bloom effect with threshold
- [ ] Screen-space ambient occlusion (SSAO)
- [ ] Temporal anti-aliasing (TAA)
- [ ] Motion blur effects

#### Material System
- [ ] Physically-based rendering (PBR) materials
- [ ] Normal mapping for surface detail enhancement
- [ ] Multiple texture support per material (diffuse, normal, roughness, metallic)
- [ ] Material property animation and tweening
- [ ] Shader-like material definition system

### Engine Architecture

#### Resource Management
- [ ] Texture atlas system for efficient GPU usage
- [ ] Mesh instancing for repeated objects
- [ ] Memory pool allocators for performance
- [ ] Async asset loading with progress tracking
- [ ] Resource reference counting and cleanup

#### Debug Tools
- [ ] Performance profiler with frame time graphs
- [ ] Debug draw system (lines, wireframes, bounding boxes)
- [ ] Console commands for runtime tweaking
- [ ] Memory usage visualization and leak detection
- [ ] Render state inspection tools

#### Configuration System
- [ ] Settings file (JSON/TOML) for graphics options
- [ ] Runtime graphics quality adjustment
- [ ] Keybinding customization system
- [ ] Resolution and display mode management
- [ ] Graphics preset system (Low/Medium/High/Ultra)

### Game-Specific Features

#### Physics Integration
- [ ] Basic rigid body physics simulation
- [ ] Collision response and resolution
- [ ] Trigger volumes for gameplay events
- [ ] Simple particle system for effects
- [ ] Physics material properties

#### Audio System
- [ ] 3D positional audio with distance attenuation
- [ ] Basic sound effect playback and management
- [ ] Audio streaming for background music
- [ ] Volume mixing and audio groups
- [ ] Audio occlusion and reverb effects

#### User Interface
- [ ] Immediate mode GUI for debug panels
- [ ] Text rendering system with multiple fonts
- [ ] Basic menu system with navigation
- [ ] HUD elements and overlay rendering
- [ ] UI animation and transitions

### Long-Term Goals

#### Graphics API Migration
- [ ] Vulkan backend implementation
- [ ] Render graph system for efficient GPU usage
- [ ] GPU-driven rendering and culling
- [ ] Compute shader integration
- [ ] Multi-GPU support

#### Advanced Rendering Techniques
- [ ] Clustered deferred rendering
- [ ] Screen-space reflections (SSR)
- [ ] Temporal upsampling techniques (DLSS-like)
- [ ] Ray-traced reflections and shadows
- [ ] Global illumination solutions

#### Engine Tools
- [ ] Visual scene editor with gizmos
- [ ] Material editor with real-time preview
- [ ] Animation system and timeline editor
- [ ] Scripting integration (Lua/WASM)
- [ ] Asset pipeline and build system

## Future Improvements

### Rendering Features

- [ ] **Texture Mapping**: UV coordinate interpolation and texture sampling
- [ ] **Anti-Aliasing**: Multi-sample or temporal anti-aliasing
- [ ] **Shadow Mapping**: Real-time shadow rendering
- [ ] **Normal Mapping**: Per-pixel surface detail
- [ ] **Post-Processing**: Bloom, tone mapping, gamma correction

### Architecture Improvements

- [ ] **Vulkan Integration**: GPU-accelerated rendering pipeline
- [ ] **Multi-threading**: Parallel rasterization
- [ ] **Scene Graph**: Hierarchical object management
- [ ] **Asset Loading**: OBJ, FBX, or glTF model importers
- [ ] **Input System**: Keyboard and mouse camera controls

### Advanced Graphics

- [ ] **Physically-Based Rendering (PBR)**: Metallic/roughness workflow
- [ ] **Deferred Rendering**: Separate geometry and lighting passes
- [ ] **Screen-Space Techniques**: SSAO, SSR, SSGI
- [ ] **Volume Rendering**: Fog, clouds, atmospheric effects

## References

### 3D Graphics Theory
- [Scratchapixel - 3D Basic Rendering](https://www.scratchapixel.com/lessons/3d-basic-rendering/computing-pixel-coordinates-of-3d-point/mathematics-computing-2d-coordinates-of-3d-points.html)
- [Game Math - Vectors](https://gamemath.com/book/vectors.html)
- [Graphics Textbook - Vectors](https://math.hws.edu/graphicsbook/c3/s5.html)

### Win32 API Documentation
- [Creating a Window with Rust](https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html)
- [Win32 Programming Introduction](https://learn.microsoft.com/en-us/windows/win32/learnwin32/your-first-windows-program)
- [Windows Message System](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues#system-defined-messages)
- [Window Procedures](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc)
- [Cursor Handling](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)

### Computer Graphics Resources
- Real-Time Rendering (Akenine-Möller, Haines, Hoffman)
- Computer Graphics: Principles and Practice (Hughes, van Dam, et al.)
- Fundamentals of Computer Graphics (Shirley, Marschner)

---

**Built with Rust 🦀** | **No external graphics libraries** | **Pure software rendering**

This project demonstrates that complex 3D graphics can be implemented entirely in software, providing deep insight into how modern GPU pipelines work internally.