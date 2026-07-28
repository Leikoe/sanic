# wait(for:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/wait(for:)>

Encodes a command that instructs the GPU to pause before starting the resource state commands until another pass updates a fence.

## Declaration

```swift
func wait(for fence: any MTLFence)
```

```swift
optional func wait(for fence: any MTLFence)
```

## Parameters

- **fence** — An [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instance to pause execution on until updated.

## Discussion

Fences maintain order to prevent GPU data hazards as the GPU runs various passes within the same command queue. The encoded resource state commands wait for a pass to update `fence` before running.

The GPU driver evaluates the fences that apply to the pass and the commands that depend on those fences when your app commits the enclosing [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer).

## See also

### Performing fence operations
- [update(_:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/update(_:)) — Encodes a command that instructs the GPU to update a fence, which signals passes waiting on the fence.
