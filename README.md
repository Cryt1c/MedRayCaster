### Dedalus - Medical Visualization Task ###

TODO:
- [x] Clean up original project (Cryt1c/opengl_rs).
- [] Implement loader for .raw files.
- [] Implement unit tests.
- [] Implement additional shader/improve existing shader.

This repository is an extension of (https://github.com/Cryt1c/opengl_rs). Original README.md:

### OpenGL Rust ###
This repository contains a prototype for a simple OpenGL application that is able to read volumetric data (.vol files) and render it using raycasting. The application is written in Rust and uses the `glow` crate for OpenGL bindings. `egui` is used for the graphical user interface.

The program loads the volume information from an external file into a 3D texture. There is a unit cube rendered in the scene. In the fragment shader, rays are cast from the camera through the unit cube. The data from the 3D texture is sampled along the ray, and the result is accumulated whereby the exact method depends on selected shader. The shader for the semi-transparent rendering is inspired by the book "OpenGL Development Cookbook: Over 40 Recipes to Help You Learn, Understand, and Implement Modern OpenGL in Your Applications" by Muhammad Mobeen Movania.

There are two shaders that can be switched on the fly. One is a Maximum Intensity Projection and the other creates a semi-transparent rendering result. It's also possible to zoom in and out and rotate the volume.

## Controls:

| Combination         	| Description     	|
|---------------------	|-----------------	|
| CTRL + Mouse Scroll 	| Zoom in and out 	|

## Code Structure #
The code is structured using modules:

# main.rs #
The main file creates the eframe/egui context and creates the Rendering instance.

# Renderer #
The Renderer module contains the OpenGL code, including the render loop, which is done in an egui painter callback. It creates the OpenGL context and loads the texture (volume data) and the shaders. It also contains the code for rendering the volume.

# Shader #
The Shader module is a helper to load the shader code, link the program, and set the uniforms for the shaders. This leverages the Uniform helper struct.

# Uniform #
The Uniform module provides a uniform trait. Based on the base type of the uniform value, the correct trait function is called.

# Volume #
The Volume module handles loading the volume (supports .vol files) and provides the unit cube for the volume rendering.
