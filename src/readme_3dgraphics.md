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


# Rasterization
Rasterization converts vector geometry into displayable pixels

Rasterization is the computational process that transforms the vector based 3D geometry into the 2D pixel arrays that monitors can display. This isn't a storage format, its an algorithm that determines which pixels should be lit up to represent your 3D triangles

The pipeline follows this path

Vertex processing transforms your 3D vertices through multiple coordinate spaces (object -> world -> camera -> screen)

Primitive Assembly groups vertices into triangles. The Rasterization stage determines which screen pixels each triangle covers and generates fragments (potential pixels) for each covered location

Fragment processing calculates final pixel colors, applies textures, and performs depth testing

