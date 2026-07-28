# endCommandBuffer()

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandbuffer/endcommandbuffer()>

Closes a command buffer to prepare it for submission to a command queue.

## Declaration

```swift
func endCommandBuffer()
```

## Discussion

Explicitly ending the command buffer allows you to reuse the [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) to start servicing other command buffers. It is an error to call `commit` on a command buffer previously recording before calling this method.
