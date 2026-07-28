# MTLRenderStages

*Structure · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderstages>

The stages in a render pass that triggers a synchronization command.

## Declaration

```swift
struct MTLRenderStages
```

## Overview

Render stage boundaries provide synchronization opportunities within a render pass for specific resources and resource types. For example, you can designate render stage a synchronization point for a memory barrier or a fence (see [memoryBarrier(resources:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(resources:after:before:)) and [MTLFence](https://developer.apple.com/documentation/metal/mtlfence), respectively). This allows a GPU to overlap its execution of two adjacent stages, which can shorten its overall runtime for the render pass.

## Topics

### Render pass stages
- [object](https://developer.apple.com/documentation/metal/mtlrenderstages/object) — The object rendering stage.
- [mesh](https://developer.apple.com/documentation/metal/mtlrenderstages/mesh) — The mesh rendering stage.
- [vertex](https://developer.apple.com/documentation/metal/mtlrenderstages/vertex) — The vertex rendering stage.
- [fragment](https://developer.apple.com/documentation/metal/mtlrenderstages/fragment) — The fragment rendering stage.
- [tile](https://developer.apple.com/documentation/metal/mtlrenderstages/tile) — The tile rendering stage.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlrenderstages/init(rawvalue:)) — Creates a render stage from a raw value.

## See also

### Synchronizing with barriers and fences
- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass) — Block GPU stages in the a pass from running until other stages in the same pass finish.
- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence) — Block GPU stages in a pass until another pass unblocks it by signaling a fence.
- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers) — Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) — The segments of command execution within the Metal pass types.
- [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) — A synchronization mechanism that orders memory operations between GPU passes.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
