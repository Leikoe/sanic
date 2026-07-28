# MTLStages

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlstages>

The segments of command execution within the Metal pass types.

## Declaration

```swift
struct MTLStages
```

## Overview

Metal associates each command with one or more stages within a pass. Use these stage identifiers to synchronize command execution within a pass by selecting which stages wait for other stages to complete.

Metal 4 introduces the following unified command encoders that combine multiple stages into a single pass:

- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) instances encode render passes that run vertex, fragment, object, mesh, and tile stages.

- [MTL4ComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtl4computecommandencoder) instances encode unified compute passes that run blit, dispatch, and acceleration structure stages.

- [MTL4MachineLearningCommandEncoder](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder) instances encode passes that run machine learning stages.

Metal 3 provides separate command encoders for different types of work:

- [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instances encode render passes that run vertex, fragment, object, mesh, and tile stages.

- [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instances encode compute passes that run dispatch stages.

- [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instances encode blit passes that run blit stages, which initialize and copy data for resources, such as buffers and textures.

- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) instances encode passes that run acceleration structure stages, such as for ray tracing.

## Topics

### Render pass stages
- [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex) — Represents all vertex shader stage work in a render pass.
- [fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) — Represents all fragment shader stage work in a render pass.
- [tile](https://developer.apple.com/documentation/metal/mtlstages/tile) — Represents all tile shading stage work in a render pass.
- [object](https://developer.apple.com/documentation/metal/mtlstages/object) — Represents all object shader stage work in a render pass.
- [mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh) — Represents all mesh shader stage work work in a render pass.

### Compute pass stages
- [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) — Represents all compute dispatches in a compute pass.
- [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) — Represents all blit operations in a pass.
- [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) — Represents all acceleration structure operations.
- [machineLearning](https://developer.apple.com/documentation/metal/mtlstages/machinelearning) — Represents all machine learning network dispatch operations.

### Resource pass stages
- [resourceState](https://developer.apple.com/documentation/metal/mtlstages/resourcestate) — Represents all sparse and placement sparse resource mapping updates.

### Convenience values
- [all](https://developer.apple.com/documentation/metal/mtlstages/all) — Convenience mask representing all stages of GPU work.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlstages/init(rawvalue:))

## See also

### Synchronizing with barriers and fences
- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass) — Block GPU stages in the a pass from running until other stages in the same pass finish.
- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence) — Block GPU stages in a pass until another pass unblocks it by signaling a fence.
- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers) — Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) — A synchronization mechanism that orders memory operations between GPU passes.
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
