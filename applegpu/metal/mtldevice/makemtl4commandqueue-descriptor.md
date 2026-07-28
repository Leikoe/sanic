# makeMTL4CommandQueue(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makemtl4commandqueue(descriptor:)>

Creates a new command queue from a queue descriptor.

## Declaration

```swift
func makeMTL4CommandQueue(descriptor: MTL4CommandQueueDescriptor) throws -> any MTL4CommandQueue
```

## Parameters

- **descriptor** — A [MTL4CommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor) instance that configures the [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) instance.

## Return Value

A [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) instance, or `nil` if the function failed.
