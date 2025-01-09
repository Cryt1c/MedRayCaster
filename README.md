# MedRayCast #

Future ideas:
- []Create histogram and threshold in hounsfield units.
- [] Adapt histogram when setting the threshold.
- [x] Make camera adaptable by dragging with the mouse on the canvas.
- [] Make parts of the model selectable and markable depending on hounsfield range.
- [] Add docs.rs.
- Add a file selector dialog.
- [x] Port to the three-d crate (exposes glow) to leverage existing helpers (e.g. Orbital Camera).
- [x] Implement rendering in WGPU. (different repo)
- [x] Use WASM to deploy app to the browser.

How to use:
- The file sinus.mhd and sinus.raw must be located inside examples/assets.
- There is a binary located in the root of the project (dedalus.exe).

This repository contains a prototype for a simple OpenGL application that is able to read volumetric data (.vol files) and render it using raycasting. The application is written in Rust and uses the `glow` crate for OpenGL bindings. `egui` is used for the graphical user interface.

The program loads the volume information from an external file into a 3D texture. There is a unit cube rendered in the scene. In the fragment shader, rays are cast from the camera through the unit cube. The data from the 3D texture is sampled along the ray, and the result is accumulated whereby the exact method depends on selected shader. The shader for the semi-transparent rendering is inspired by the book "OpenGL Development Cookbook: Over 40 Recipes to Help You Learn, Understand, and Implement Modern OpenGL in Your Applications" by Muhammad Mobeen Movania.

There are two shaders that can be switched on the fly. One is a Maximum Intensity Projection and the other creates a semi-transparent rendering result. It's also possible to zoom in and out and rotate the volume.

## User Interface ##
### Threshold / Histogram ###
The histogram shows the distribution of pixel values on a scale from 0-255. You can use the provided histogram to set lower and upper threshold values to only display specific materials.

### Camera Controls ###
Using translate you can reposition the camera. It's always looking at the origin where the model is rendered. To rotate on a specific axis you can use the rotation controls.

### Shaders ###
There are three shader types implemented. The default shader is inspired by the OpenGL Development Cookbook. The Maximum Intensity Projection (MIP) shader uses the maximum value that is encountered on the casted ray. The Average Intensity Projection (AIP) sums all values encountered on the ray and averages them.

## Controls: ##
| Combination         	| Description     	|
|---------------------	|-----------------	|
| CTRL + Mouse Scroll 	| Zoom in and out 	|

## Code Structure ##
The code is structured using modules:

### main.rs ###
The main file creates the eframe/egui context and creates the Rendering instance.

### Renderer ###
The Renderer module contains the OpenGL code, including the render loop, which is done in an egui painter callback. It creates the OpenGL context and loads the texture (volume data) and the shaders. It also contains the code for rendering the volume.

### Shader ###
The Shader module is a helper to load the shader code, link the program, and set the uniforms for the shaders. This leverages the Uniform helper struct.

### Uniform ###
The Uniform module provides a uniform trait. Based on the base type of the uniform value, the correct trait function is called.

### Volume ###
The volume loading supports .mhd and .raw files in little endian 16-bit unsigned short containing hounsfield unit (range 0-4095).
The loader can read the DimSize from the mhd file. NDims and ElementSpacing are prepared to be used in code.
It also provides the unit cube for the volume rendering.

### UserInterface ###
In this module the construction of the user controls and the histogram is done. It also provides a frame timer to display the current frames per second.

### Renderer ###
The renderer contains structs like Renderer, Scene, Camera and the Uniforms that composite the rendering. The information from these structs are used for rendering the scene. All values passed to the shaders are contained in the Uniforms.


## WASM ##
To build for WASM run

```npx wasm-pack build "." --target web --out-name web --out-dir ./web/pkg```

To serve the wasm build run the following inside the /web directory

```
npm install
npm run serve
```

## WSL cross-compilation for windows ##

```sudo apt-get install mingw-w64```
```rustup target add x86_64-pc-windows-gnu```
```cargo build --target x86_64-pc-windows-gnu```
