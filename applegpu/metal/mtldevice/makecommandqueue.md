# makeCommandQueue()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue()>

Creates a queue you use to submit rendering and computation commands to a GPU.

## Declaration

```swift
func makeCommandQueue() -> (any MTLCommandQueue)?
```

## Return Value

A new [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance if the method completed successfully; otherwise `nil`.

## Discussion

A command queue can only submit commands to the GPU device instance that created it.

> **Important:**
>  The command queues you create with this method allow up to 64 uncompleted command buffers at time.

This method is the equivalent of passing `64` to the [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:)) method.

```swift
let commandQueue = device.makeCommandQueue(maxCommandBufferCount: 64)
```

```objective-c
id<MTLCommandQueue> commandQueue;

NSUInteger capacity = 64;
commandQueue = [device newCommandQueueWithMaxCommandBufferCount:capacity];
```

## See also

### Creating command queues
- [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:)) — Creates a queue you use to submit rendering and computation commands to a GPU that has a fixed number of uncompleted command buffers.
