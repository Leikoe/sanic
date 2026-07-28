# beginCommandBuffer(allocator:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandbuffer/begincommandbuffer(allocator:)>

Prepares a command buffer for encoding.

## Declaration

```swift
func beginCommandBuffer(allocator: any MTL4CommandAllocator)
```

## Parameters

- **allocator** — [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) to attach to.

## Discussion

Attaches the command buffer to the specified [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) and declares that the application is ready to encode commands into the command buffer.

Command allocators only service a single command buffer at a time. If you need to issue multiple calls to this method simultaneously, for example, in a multi-threaded command encoding scenario, create multiple instances of `MTLCommandAllocator` and use one for each call.

You can safely reuse command allocators after ending the command buffer using it by calling [endCommandBuffer()](https://developer.apple.com/documentation/metal/mtl4commandbuffer/endcommandbuffer()).

After calling this method, any prior calls to [useResidencySet(_:)](https://developer.apple.com/documentation/metal/mtl4commandbuffer/useresidencyset(_:)) and [useResidencySets:count:](https://developer.apple.com/documentation/metal/mtl4commandbuffer/useresidencysets:count:) on this command buffer instance no longer apply. Make sure to call these methods again to signal your residency requirements to Metal.
