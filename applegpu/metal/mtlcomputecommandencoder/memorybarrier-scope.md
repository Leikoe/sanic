# memoryBarrier(scope:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(scope:)>

Creates a memory barrier that enforces the order of write and read operations for specific resource types.

## Declaration

```swift
func memoryBarrier(scope: MTLBarrierScope)
```

## Parameters

- **scope** — An [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) instance that represents the resource types the barrier synchronizes operations on.

## Discussion

Memory barriers ensure the relevant passes finish updating resources before starting the stages of subsequent commands that depend on those resources.

To determine whether a GPU supports memory barriers, see the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf).

## See also

### Preventing resource access conflicts
- [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/waitforfence(_:)) — Encodes a command that instructs the GPU to pause the compute pass until another pass updates a fence.
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the compute pass completes.
- [memoryBarrier(resources:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(resources:)) — Creates a memory barrier that enforces the order of write and read operations for specific resources.
