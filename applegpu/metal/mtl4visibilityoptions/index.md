# MTL4VisibilityOptions

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4visibilityoptions>

Memory consistency options for synchronization commands.

## Declaration

```swift
struct MTL4VisibilityOptions
```

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4visibilityoptions/init(rawvalue:))

### Type Properties
- [device](https://developer.apple.com/documentation/metal/mtl4visibilityoptions/device) — Flushes caches to the GPU (device) memory coherence point.
- [resourceAlias](https://developer.apple.com/documentation/metal/mtl4visibilityoptions/resourcealias) — Flushes caches to ensure that aliased virtual addresses are memory consistent.

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
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
