# Metal sample code library

<https://developer.apple.com/documentation/metal/metal-sample-code-library>

Explore the complete set of Metal samples.

## Overview

Browse the topics below to find samples relevant to a concept you want to learn more about, starting with the basic computation and render workflows. The samples in the lighting and multiple technique sections demonstrate how to take advantage of the unique GPU architecture of Apple silicon.

## Topics

### Compute workflows
- [Performing calculations on a GPU](https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu) — Use Metal to find GPUs and perform calculations on them.
- [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing) — Switch dynamically between multiple GPUs to efficiently execute a compute-intensive simulation.
- [Customizing a TensorFlow operation](https://developer.apple.com/documentation/metal/customizing-a-tensorflow-operation) — Implement a custom operation that uses Metal kernels to accelerate neural-network training performance.
- [Customizing a PyTorch operation](https://developer.apple.com/documentation/metal/customizing-a-pytorch-operation) — Implement a custom operation in PyTorch that uses Metal kernels to improve performance.

### Machine learning workflows
- [Running a machine learning model on the GPU timeline](https://developer.apple.com/documentation/metal/running-a-machine-learning-model-on-the-gpu-timeline) — Dispatch model inference commands with a machine learning pass in a Metal 4 command buffer.
- [Training a neural network to render irradiance in real time](https://developer.apple.com/documentation/metal/training-a-neural-network-to-render-irradiance-in-real-time) — Train a small neural network on the GPU to approximate diffuse irradiance, and compare the result against Monte Carlo integration and a pre-trained ML model.
- [Running inline ML operations in a shader with Metal 4](https://developer.apple.com/documentation/metal/running-inline-ml-operations-in-a-shader-with-metal-4) — Multiply matrices across multiple GPU cores with inline tensor operations.

### Render workflows
- [Using Metal to draw a view’s contents](https://developer.apple.com/documentation/metal/using-metal-to-draw-a-view's-contents) — Create a MetalKit view and a render pass to draw the view’s contents.
- [Drawing a triangle with Metal 4](https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4) — Render a colorful, rotating 2D triangle by running draw commands with a render pipeline on a GPU.
- [Selecting device objects for graphics rendering](https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering) — Switch dynamically between multiple GPUs to efficiently render to a display.
- [Customizing render pass setup](https://developer.apple.com/documentation/metal/customizing-render-pass-setup) — Render into an offscreen texture by creating a custom render pass.
- [Creating a custom Metal view](https://developer.apple.com/documentation/metal/creating-a-custom-metal-view) — Implement a lightweight view for Metal rendering that’s customized to your app’s needs.
- [Calculating primitive visibility using depth testing](https://developer.apple.com/documentation/metal/calculating-primitive-visibility-using-depth-testing) — Determine which pixels are visible in a scene by using a depth texture.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Implementing order-independent transparency with image blocks](https://developer.apple.com/documentation/metal/implementing-order-independent-transparency-with-image-blocks) — Draw overlapping, transparent surfaces in any order by using tile shaders and image blocks.
- [Loading textures and models using Metal fast resource loading](https://developer.apple.com/documentation/metal/loading-textures-and-models-using-metal-fast-resource-loading) — Stream texture and buffer data directly from disk into Metal resources using fast resource loading.
- [Adjusting the level of detail using Metal mesh shaders](https://developer.apple.com/documentation/metal/adjusting-the-level-of-detail-using-metal-mesh-shaders) — Choose and render meshes with several levels of detail using object and mesh shaders.
- [Creating a 3D application with hydra rendering](https://developer.apple.com/documentation/metal/creating-a-3d-application-with-hydra-rendering) — Build a 3D application that integrates with Hydra and USD.
- [Culling occluded geometry using the visibility result buffer](https://developer.apple.com/documentation/metal/culling-occluded-geometry-using-the-visibility-result-buffer) — Draw a scene without rendering hidden geometry by checking whether each object in the scene is visible.
- [Improving edge-rendering quality with multisample antialiasing (MSAA)](https://developer.apple.com/documentation/metal/improving-edge-rendering-quality-with-multisample-antialiasing-msaa) — Apply MSAA to enhance the rendering of edges with custom resolve options and immediate and tile-based resolve paths.
- [Achieving smooth frame rates with a Metal display link](https://developer.apple.com/documentation/metal/achieving-smooth-frame-rates-with-a-metal-display-link) — Pace rendering with minimal input latency while providing essential information to the operating system for power-efficient rendering, thermal mitigation, and the scheduling of sustainable workloads.

### Textures
- [Combining blit and compute operations in a single pass](https://developer.apple.com/documentation/metal/combining-blit-and-compute-operations-in-a-single-pass) — Run concurrent blit commands and then a compute dispatch in a single pass with a unified compute encoder.
- [Reading pixel data from a drawable texture](https://developer.apple.com/documentation/metal/reading-pixel-data-from-a-drawable-texture) — Access texture data from the CPU by copying it to a buffer.
- [Creating and sampling textures](https://developer.apple.com/documentation/metal/creating-and-sampling-textures) — Load image data into a texture and apply it to a quadrangle.
- [Streaming large images with Metal sparse textures](https://developer.apple.com/documentation/metal/streaming-large-images-with-metal-sparse-textures) — Limit texture memory usage for large textures by loading or unloading image detail on the basis of MIP and tile region.

### Argument buffers
- [Managing groups of resources with argument buffers](https://developer.apple.com/documentation/metal/managing-groups-of-resources-with-argument-buffers) — Create argument buffers to organize related resources.
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Encoding argument buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu) — Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
- [Rendering terrain dynamically with argument buffers](https://developer.apple.com/documentation/metal/rendering-terrain-dynamically-with-argument-buffers) — Use argument buffers to render terrain in real time with a GPU-driven pipeline.

### Shaders
- [Creating a Metal dynamic library](https://developer.apple.com/documentation/metal/creating-a-metal-dynamic-library) — Compile a library of shaders and write it to a file as a dynamically linked library.
- [Using function specialization to build pipeline variants](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants) — Create pipelines for different levels of detail from a common shader source.

### Synchronization
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.

### Lighting techniques
- [Rendering a scene with forward plus lighting using tile shaders](https://developer.apple.com/documentation/metal/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders) — Implement a forward plus renderer using the latest features on Apple GPUs.
- [Rendering a scene with deferred lighting in Objective-C](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-objective-c) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering a scene with deferred lighting in Swift](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-swift) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering a scene with deferred lighting in C++](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-c++) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering reflections with fewer render passes](https://developer.apple.com/documentation/metal/rendering-reflections-with-fewer-render-passes) — Use layer selection to reduce the number of render passes needed to generate an environment map.

### Multiple techniques
- [Modern rendering with Metal](https://developer.apple.com/documentation/metal/modern-rendering-with-metal) — Use advanced Metal features such as indirect command buffers, sparse textures, and variable rate rasterization to implement complex rendering techniques.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.

### Ray tracing
- [Rendering reflections in real time using ray tracing](https://developer.apple.com/documentation/metal/rendering-reflections-in-real-time-using-ray-tracing) — Implement realistic real-time lighting by dynamically generating reflection maps by encoding a ray-tracing compute pass.
- [Accelerating ray tracing using Metal](https://developer.apple.com/documentation/metal/accelerating-ray-tracing-using-metal) — Implement ray-traced rendering using GPU-based parallel processing.
- [Control the ray tracing process using intersection queries](https://developer.apple.com/documentation/metal/control-the-ray-tracing-process-using-intersection-queries) — Explicitly enumerate a ray’s intersections with acceleration structures by creating an intersection query object.
- [Accelerating ray tracing and motion blur using Metal](https://developer.apple.com/documentation/metal/accelerating-ray-tracing-and-motion-blur-using-metal) — Generate ray-traced images with motion blur using GPU-based parallel processing.
- [Rendering a curve primitive in a ray tracing scene](https://developer.apple.com/documentation/metal/rendering-a-curve-primitive-in-a-ray-tracing-scene) — Implement ray traced rendering using GPU-based parallel processing.

### HDR
- [Processing HDR images with Metal](https://developer.apple.com/documentation/metal/processing-hdr-images-with-metal) — Implement a post-processing pipeline using the latest features on Apple GPUs.

### OpenGL
- [Migrating OpenGL code to Metal](https://developer.apple.com/documentation/metal/migrating-opengl-code-to-metal) — Replace your app’s deprecated OpenGL code with Metal.
- [Mixing Metal and OpenGL rendering in a view](https://developer.apple.com/documentation/metal/mixing-metal-and-opengl-rendering-in-a-view) — Draw with Metal and OpenGL in the same view using an interoperable texture.
