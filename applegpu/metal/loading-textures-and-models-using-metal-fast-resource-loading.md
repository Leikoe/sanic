# Loading textures and models using Metal fast resource loading

*Sample Code · macOS 13.0, Xcode 14.0*

<https://developer.apple.com/documentation/metal/loading-textures-and-models-using-metal-fast-resource-loading>

Stream texture and buffer data directly from disk into Metal resources using fast resource loading.

## Overview

> **Note:**
> This sample code project is associated with WWDC22 session [10104: Load resources faster with Metal 3](https://developer.apple.com/wwdc22/10104/).

### Configure the sample code project

This sample code project requires the following:

- macOS 13 or later, and a Mac with Apple silicon

- Xcode 14 or later

## See also

### Render workflows
- [Using Metal to draw a view’s contents](https://developer.apple.com/documentation/metal/using-metal-to-draw-a-view's-contents) — Create a MetalKit view and a render pass to draw the view’s contents.
- [Drawing a triangle with Metal 4](https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4) — Render a colorful, rotating 2D triangle by running draw commands with a render pipeline on a GPU.
- [Selecting device objects for graphics rendering](https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering) — Switch dynamically between multiple GPUs to efficiently render to a display.
- [Customizing render pass setup](https://developer.apple.com/documentation/metal/customizing-render-pass-setup) — Render into an offscreen texture by creating a custom render pass.
- [Creating a custom Metal view](https://developer.apple.com/documentation/metal/creating-a-custom-metal-view) — Implement a lightweight view for Metal rendering that’s customized to your app’s needs.
- [Calculating primitive visibility using depth testing](https://developer.apple.com/documentation/metal/calculating-primitive-visibility-using-depth-testing) — Determine which pixels are visible in a scene by using a depth texture.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Implementing order-independent transparency with image blocks](https://developer.apple.com/documentation/metal/implementing-order-independent-transparency-with-image-blocks) — Draw overlapping, transparent surfaces in any order by using tile shaders and image blocks.
- [Adjusting the level of detail using Metal mesh shaders](https://developer.apple.com/documentation/metal/adjusting-the-level-of-detail-using-metal-mesh-shaders) — Choose and render meshes with several levels of detail using object and mesh shaders.
- [Creating a 3D application with hydra rendering](https://developer.apple.com/documentation/metal/creating-a-3d-application-with-hydra-rendering) — Build a 3D application that integrates with Hydra and USD.
- [Culling occluded geometry using the visibility result buffer](https://developer.apple.com/documentation/metal/culling-occluded-geometry-using-the-visibility-result-buffer) — Draw a scene without rendering hidden geometry by checking whether each object in the scene is visible.
- [Improving edge-rendering quality with multisample antialiasing (MSAA)](https://developer.apple.com/documentation/metal/improving-edge-rendering-quality-with-multisample-antialiasing-msaa) — Apply MSAA to enhance the rendering of edges with custom resolve options and immediate and tile-based resolve paths.
- [Achieving smooth frame rates with a Metal display link](https://developer.apple.com/documentation/metal/achieving-smooth-frame-rates-with-a-metal-display-link) — Pace rendering with minimal input latency while providing essential information to the operating system for power-efficient rendering, thermal mitigation, and the scheduling of sustainable workloads.
