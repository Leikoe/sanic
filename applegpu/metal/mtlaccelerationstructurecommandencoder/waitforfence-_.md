# waitForFence(_:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/waitforfence(_:)>

Encodes a command that instructs the GPU to pause the acceleration structure pass until another pass updates a fence.

## Declaration

```swift
func waitForFence(_ fence: any MTLFence)
```

## Parameters

- **fence** — A fence that the pass waits for before it runs any of its commands.

## Discussion

You can synchronize memory operations of an acceleration structure pass that access resources with an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence). This method instructs the GPU to wait until another pass updates `fence` before running the acceleration structure pass. The fence indicates when the pass can access those resources without a race condition.

For more information about synchronization with fences, see:

- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization)

- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence)

### Reuse a fence by waiting first and updating second

When encoding an acceleration structure pass that reuses a fence, wait for other passes to update the fence before repurposing that fence to notify subsequent passes with an update:

1. Call the [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/waitforfence(_:)) method before encoding commands that need to wait for other passes.

2. Call the [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/updatefence(_:)) method after encoding commands that later passes depend on.

The GPU driver evaluates the fences that apply to the pass and the commands that depend on those fences when your app commits the enclosing [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer).

> **Warning:**
>  Don’t update a fence and then wait for the same fence within a pass because it can create a GPU deadlock.

## See also

### Preventing resource access conflicts
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the acceleration structure pass completes.
