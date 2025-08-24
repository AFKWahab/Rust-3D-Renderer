# Documentation for 3dgraphics

# Vectors VS Rasters
Vectors represent mathematical geometry and transformations, while rasterization is the process that converts this vecor data into the pixel arrays

# Vectors

Introduction to vectors mathematically:
https://gamemath.com/book/vectors.html
https://math.hws.edu/graphicsbook/c3/s5.html

Vectors form the mathematical backbone of 3D graphics. 

## Position vectors
The position vectors represents specific locations in the 3D space (x,y,z) coordinates relative to an origin 
Every vertex in the 3D Models is defined by a position vector that describes where that point exists in space

## Direction vectors
Direction vectors represents orientation and movement without inherent position. They describe which way and how far but not where. These can be used for surface normals, light directions, camera look-at vectors and movement calculations

## Vector operations
Vector operations enables rendering calculations. The dot product determines lighting intensity by measuring angles between surface normals and light directions

The cross product generates surface normals from triangle edges and helps with backface culling

Matrix-vector multiplication handles all transformations-translation, rotation, scaling and projection


# Matrices, and matrice operations and why they matter
One might wonder why we need complex inverse functions for 4x4 matrices etc.

The real problem is that we want to draw a 3D world on a 2D screen. But the 3D points we have in our vectors, are described from the worlds perspective, but we want to know how they look from the cameras perspective.

Think of it like taking a photo, the world coordinates is where the things are, but the camera is not at the world origin. 

To draw these objects, we need camera coordinates, from the cameras view: 
- Whats left/right of the camera? (x-axis)
- Whats UP/DOWN from the camera? (y-axis)
- Whats IN FRONT/Behind the camera (z-axis)?

The matrix transformation: A camera to world matrix describes "If i move the camera from (0,0,0) to its actual position/rotation, what transformation do i apply?"

But for rendering, we need the OPPOSITE: "If i have a world point, how do i see it from the camera's perspective"?

Real word analogy would be
- **Camera-to-world**: "Move my camera 5 steps forward, 2 steps right"
- **World-to-camera**: "Move the entire world 5 steps backward, 2 steps left" (opposite direction)

And thats what the inverse does! It reverses the transformation.

After we transform world points to camera space, we can actualyl project it into 2D. This is more clear because we need the inverse to flip the pserpective from world view to camera view.

## But what does these matrices actually represent then?
This is its structure
[Rxx  Rxy  Rxz  Tx]
[Ryx  Ryy  Ryz  Ty]
[Rzx  Rzy  Rzz  Tz]
[ 0    0    0   1 ]

The upper left 3x3 part (the R values) handles rotation & scale
[Rxx  Rxy  Rxz]
[Ryx  Ryy  Ryz]  ← This part handles rotation and scaling
[Rzx  Rzy  Rzz]
These 9 values defines the objects orientation and size. For pure rotation: these are the 3D rotation values
For scaling: diagonal values (Rxx, Ryy, Rzz) control stretch/shrink

The right column (the T values) - Translation:
[Tx]
[Ty]  ← This part handles position/movement
[Tz]
These 3 values define WHERE the object is in 3D space
Tx = left/right position
Ty = up/down position
Tz = forward/back position

The bottom part is just 0 0 0 1, which enables the math to work (homogenous coordinates)

What the matrix represents overall:
"If I place an object at the origin (0,0,0) with no rotation, this matrix describes where it ends up and how it's oriented."

This can be an example of a camera matrix
[1.0  0.0  0.0   5.0]  ← Camera moved 5 units right
[0.0  0.8  0.6   2.0]  ← Camera moved 2 units up, tilted
[0.0 -0.6  0.8  -10.0] ← Camera moved 10 units back, tilted
[0.0  0.0  0.0   1.0]  ← Always this

# Rasterization
Rasterization converts vector geometry into displayable pixels

Rasterization is the computational process that transforms the vector based 3D geometry into the 2D pixel arrays that monitors can display. This isn't a storage format, its an algorithm that determines which pixels should be lit up to represent your 3D triangles

The pipeline follows this path

Vertex processing transforms your 3D vertices through multiple coordinate spaces (object -> world -> camera -> screen)

Primitive Assembly groups vertices into triangles. The Rasterization stage determines which screen pixels each triangle covers and generates fragments (potential pixels) for each covered location

Fragment processing calculates final pixel colors, applies textures, and performs depth testing





# Project 3D vector into pixels
https://www.scratchapixel.com/lessons/3d-basic-rendering/computing-pixel-coordinates-of-3d-point/mathematics-computing-2d-coordinates-of-3d-points.html
