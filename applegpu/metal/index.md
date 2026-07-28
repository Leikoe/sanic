# Metal

*Framework · iOS 8.0, iPadOS, Mac Catalyst 13.0, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal>

Render advanced 3D graphics and compute data in parallel with graphics processors.

## Overview

The Metal framework gives your app direct access to a device’s graphics processing unit (GPU). With Metal, apps can leverage a GPU to quickly render complex scenes and run computational tasks in parallel. For example, apps in these categories use Metal to maximize their performance:

- Games that render sophisticated 2D or 3D environments

- Video processing apps, like Final Cut Pro

- Scientific research apps that analyze and process large datasets

- Fully immersive visionOS apps

Metal works hand-in-hand with other frameworks that supplement its capability. For example, [MetalFX](https://developer.apple.com/documentation/MetalFX) upscales your renderings in less time than rendering them natively, and [MetalKit](https://developer.apple.com/documentation/MetalKit) simplifies the tasks that display your Metal content onscreen. The [Metal Performance Shaders](https://developer.apple.com/documentation/MetalPerformanceShaders) framework provides a large library of optimized compute and rendering shaders that take advantage of each GPU’s unique hardware. In visionOS, create fully immersive stereoscopic content with the help of the [Compositor Services](https://developer.apple.com/documentation/CompositorServices) framework.

Many high-level Apple frameworks leverage the performance of Metal, including [RealityKit](https://developer.apple.com/documentation/RealityKit), [SpriteKit](https://developer.apple.com/documentation/SpriteKit), and [Core Image](https://developer.apple.com/documentation/CoreImage). These high-level frameworks implement the GPU programming details for you. However, you can typically get better performance by writing your own custom Metal and shader code. See the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) for shader implementation details.

## Topics

### Essentials
- [Understanding the Metal 4 core API](https://developer.apple.com/documentation/metal/understanding-the-metal-4-core-api) — Discover the features and functionality in the Metal 4 foundational APIs.
- [Drawing a triangle with Metal 4](https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4) — Render a colorful, rotating 2D triangle by running draw commands with a render pipeline on a GPU.
- [Performing calculations on a GPU](https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu) — Use Metal to find GPUs and perform calculations on them.
- [Using Metal to draw a view’s contents](https://developer.apple.com/documentation/metal/using-metal-to-draw-a-view's-contents) — Create a MetalKit view and a render pass to draw the view’s contents.

### Samples
- [Metal sample code library](https://developer.apple.com/documentation/metal/metal-sample-code-library) — Explore the complete set of Metal samples.

### GPU devices
- [GPU devices and work submission](https://developer.apple.com/documentation/metal/gpu-devices-and-work-submission) — Find any available GPU, submit work to it with command buffers, suspend work, and coordinate between multiple GPUs.

### Command encoders
- [Render passes](https://developer.apple.com/documentation/metal/render-passes) — Encode a render pass to draw graphics into an image.
- [Compute passes](https://developer.apple.com/documentation/metal/compute-passes) — Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
- [Machine learning passes](https://developer.apple.com/documentation/metal/machine-learning-passes) — Add machine learning model inference to your Metal app’s GPU workflow.
- [Blit passes](https://developer.apple.com/documentation/metal/blit-passes) — Encode a block information transfer pass to adjust and copy data to and from GPU resources, such as buffers and textures.
- [Indirect command encoding](https://developer.apple.com/documentation/metal/indirect-command-encoding) — Store draw commands in Metal buffers and run them at a later time on the GPU, either once or repeatedly.
- [Ray tracing with acceleration structures](https://developer.apple.com/documentation/metal/ray-tracing-with-acceleration-structures) — Build a representation of your scene’s geometry using triangles and bounding volumes to quickly trace rays through the scene.

### Resources
- [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) — Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
- [Buffers](https://developer.apple.com/documentation/metal/buffers) — Create and manage untyped data your app uses to exchange information with its shader functions.
- [Textures](https://developer.apple.com/documentation/metal/textures) — Create and manage typed data your app uses to exchange information with its shader functions.
- [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps) — Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
- [Resource loading](https://developer.apple.com/documentation/metal/resource-loading) — Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) — Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.

### Shader compilation and libraries
- [Using the Metal 4 compilation API](https://developer.apple.com/documentation/metal/using-the-metal-4-compilation-api) — Control when and how you compile an app’s shaders.
- [Shader libraries](https://developer.apple.com/documentation/metal/shader-libraries) — Manage and load your app’s Metal shaders.
- [Using function specialization to build pipeline variants](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants) — Create pipelines for different levels of detail from a common shader source.

### Presentation
- [Managing your game window for Metal in macOS](https://developer.apple.com/documentation/metal/managing-your-game-window-for-metal-in-macos) — Set up a window and view for optimally displaying your Metal content.
- [Managing your Metal app window in iPadOS](https://developer.apple.com/documentation/metal/managing-your-metal-app-window-in-ipados) — Set up a window that handles dynamically resizing your Metal content.
- [Adapting your game interface for smaller screens](https://developer.apple.com/documentation/metal/adapting-your-game-interface-for-smaller-screens) — Make text legible on all devices the player chooses to run your game on.
- [Onscreen presentation](https://developer.apple.com/documentation/metal/onscreen-presentation) — Show the output from a GPU’s rendering pass to the user in your app.
- [HDR content](https://developer.apple.com/documentation/metal/hdr-content) — Take advantage of high dynamic range to present more vibrant colors in your apps and games.

### Developer tools
- [Supporting Simulator in a Metal app](https://developer.apple.com/documentation/metal/supporting-simulator-in-a-metal-app) — Configure alternative render paths in your Metal app to enable running your app in Simulator.
- [Capturing Metal commands programmatically](https://developer.apple.com/documentation/metal/capturing-metal-commands-programmatically) — Invoke a Metal frame capture from your app, then save the resulting GPU trace to a file or view it in Xcode.
- [Logging shader debug messages](https://developer.apple.com/documentation/metal/logging-shader-debug-messages) — Print debugging messages that a shader generates using shader logging.
- [Developing Metal apps that run in Simulator](https://developer.apple.com/documentation/metal/developing-metal-apps-that-run-in-simulator) — Prototype and test your Metal apps in Simulator.
- [Improving your game’s graphics performance and settings](https://developer.apple.com/documentation/metal/improving-your-games-graphics-performance-and-settings) — Fix performance glitches and develop default settings for smooth experiences on Apple platforms using the powerful suite of Metal development tools.
- [Metal debugger](https://developer.apple.com/documentation/Xcode/Metal-debugger) — Debug and profile your Metal workload with a GPU trace.
- [Metal developer workflows](https://developer.apple.com/documentation/Xcode/Metal-developer-workflows) — Locate and fix issues related to your app’s use of the Metal API and GPU functions.
- [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers) — Retrieve runtime data from a GPU device by sampling one or more of its counters.
- [Metal debugging types](https://developer.apple.com/documentation/metal/metal-debugging-types) — Create capture managers and capture scopes, and review a GPU device’s log after it runs a command buffer.

### Apple silicon
- [Porting your Metal code to Apple silicon](https://developer.apple.com/documentation/Apple-Silicon/porting-your-metal-code-to-apple-silicon) — Create a version of your Metal app that runs on both Apple silicon and Intel-based Mac computers.
- [Tailor your apps for Apple GPUs and tile-based deferred rendering](https://developer.apple.com/documentation/metal/tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering) — Learn about characteristic Apple GPU features, including imageblocks, tile shaders, and raster order groups.

### Reference
- [Metal structures](https://developer.apple.com/documentation/metal/metal-structures)
- [Metal enumerations](https://developer.apple.com/documentation/metal/metal-enumerations)
- [Metal constants](https://developer.apple.com/documentation/metal/metal-constants)
- [Metal data types](https://developer.apple.com/documentation/metal/metal-data-types)
- [Metal variables](https://developer.apple.com/documentation/metal/metal-variables)

### Classes
- [MTLTensorAuxiliaryPlaneDescriptor](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptor) — A configuration for an auxiliary plane in a multi-plane tensor.
- [MTLTensorAuxiliaryPlaneDescriptorMap](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap) — A map of auxiliary plane descriptors keyed by plane type.
- [MTLTensorAuxiliaryPlaneType](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanetype) — An auxiliary plane that a shader’s tensor argument requires.
- [MTLTensorBufferAttachments](https://developer.apple.com/documentation/metal/mtltensorbufferattachments) — An object that associates each plane of a tensor with a buffer and byte offset for buffer-backed tensor creation.

### Protocols
- [MTLTensorAuxiliaryPlane](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane) — A type that represents the configuration and storage of an auxiliary plane in a multi-plane tensor.

### Structures
- [MTLDeviceError](https://developer.apple.com/documentation/metal/mtldeviceerror-swift.struct)

### Variables
- [MTLDeviceErrorDomain](https://developer.apple.com/documentation/metal/mtldeviceerrordomain)

### Enumerations
- [MTLFloatingPointConversionRoundingMode](https://developer.apple.com/documentation/metal/mtlfloatingpointconversionroundingmode)
- [MTLTensorPlaneType](https://developer.apple.com/documentation/metal/mtltensorplanetype) — The possible tensor plane types.

## See also

### Related Documentation
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)
- [Metal Best Practices Guide](https://developer.apple.com/library/archive/documentation/3DDrawing/Conceptual/MTLBestPracticesGuide/index.html#//apple_ref/doc/uid/TP40016642)
