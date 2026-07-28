# memoryBarrier(resources:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.14, tvOS 12.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(resources:)>

Creates a memory barrier that enforces the order of write and read operations for specific resources.

## Declaration

```swift
func memoryBarrier(resources: [any MTLResource])
```

## Parameters

- **resources** — An array of [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instances the barrier applies to.

## Discussion

Memory barriers ensure the relevant passes finish updating resources before starting the stages of subsequent commands that depend on those resources.

To determine whether a GPU supports memory barriers, see the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf).

## See also

### Preventing resource access conflicts
- [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/waitforfence(_:)) — Encodes a command that instructs the GPU to pause the compute pass until another pass updates a fence.
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the compute pass completes.
- [memoryBarrier(scope:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(scope:)) — Creates a memory barrier that enforces the order of write and read operations for specific resource types.
