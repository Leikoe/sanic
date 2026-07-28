# makeCommandQueue(maxCommandBufferCount:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:)>

Creates a queue you use to submit rendering and computation commands to a GPU that has a fixed number of uncompleted command buffers.

## Declaration

```swift
func makeCommandQueue(maxCommandBufferCount: Int) -> (any MTLCommandQueue)?
```

## Parameters

- **maxCommandBufferCount** — An integer that sets the maximum number of uncompleted command buffers the queue can allow.

## Return Value

A new [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance if the method completed successfully; otherwise `nil`.

## Discussion

A Command queue can only submit commands to the GPU device instance that created it.

## See also

### Creating command queues
- [makeCommandQueue()](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue()) — Creates a queue you use to submit rendering and computation commands to a GPU.
