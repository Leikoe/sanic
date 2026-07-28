# memoryBarrier(scope:after:before:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 13.0, macOS 10.14, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(scope:after:before:)>

Creates a memory barrier that enforces the order of write and read operations for specific resource types.

## Declaration

```swift
func memoryBarrier(scope: MTLBarrierScope, after: MTLRenderStages, before: MTLRenderStages)
```

## Parameters

- **scope** — An [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) instance that represents the resource types the barrier synchronizes operations on.
- **after** — The render stages of previous draw commands that modify resources of the types that `scope` defines.
- **before** — The render stages of subsequent draw commands that read or modify resources of the types that `scope` defines.

## Discussion

Memory barriers ensure the relevant stages of prior draw commands finish updating resources before starting the stages of subsequent commands that depend on those resources.

To determine whether a GPU supports memory barriers, see the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf).

## See also

### Preventing resource access conflicts
- [waitForFence(_:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/waitforfence(_:before:)) — Encodes a command that instructs the GPU to pause before starting one or more stages of the render pass until a pass updates a fence.
- [updateFence(_:after:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/updatefence(_:after:)) — Encodes a command that instructs the GPU to update a fence after one or more stages, which can unblock other passes waiting for the fence.
- [memoryBarrier(resources:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(resources:after:before:)) — Creates a memory barrier that enforces the order of write and read operations for specific resources.
