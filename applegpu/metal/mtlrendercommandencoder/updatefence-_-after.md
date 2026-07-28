# updateFence(_:after:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/updatefence(_:after:)>

Encodes a command that instructs the GPU to update a fence after one or more stages, which can unblock other passes waiting for the fence.

## Declaration

```swift
func updateFence(_ fence: any MTLFence, after stages: MTLRenderStages)
```

## Parameters

- **fence** — A fence the pass updates after the stages in `stages` complete.
- **stages** — The render stages that need to complete before the pass updates `fence`.

## Discussion

You can synchronize memory operations of a render pass that access resources with an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence). This method instructs the pass to update `fence` after the stages you pass to the `stages` run all their memory store operations to the resources it accesses. The fence indicates when other passes can access those resources without a race condition.

For more information about synchronization with fences, see:

- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization)

- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence)

### Reuse a fence by waiting first and updating second

When encoding a render pass that reuses a fence, wait for other passes to update the fence before repurposing that fence to notify subsequent passes with an update:

1. Call the [waitForFence(_:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/waitforfence(_:before:)) method before encoding commands that need to wait for other passes.

2. Call the [updateFence(_:after:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/updatefence(_:after:)) method after encoding commands that later passes depend on.

The GPU driver evaluates the fences that apply to the pass and the commands that depend on those fences when your app commits the enclosing [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer).

> **Warning:**
>  Don’t update a fence and then wait for the same fence within a pass because it can create a GPU deadlock.

To synchronize different stages within a single pass, create an *intrapass barrier* because a fence can only synchronize memory operations between different passes. For more information, see [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass).

## See also

### Preventing resource access conflicts
- [waitForFence(_:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/waitforfence(_:before:)) — Encodes a command that instructs the GPU to pause before starting one or more stages of the render pass until a pass updates a fence.
- [memoryBarrier(resources:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(resources:after:before:)) — Creates a memory barrier that enforces the order of write and read operations for specific resources.
- [memoryBarrier(scope:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(scope:after:before:)) — Creates a memory barrier that enforces the order of write and read operations for specific resource types.
