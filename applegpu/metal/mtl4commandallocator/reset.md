# reset()

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandallocator/reset()>

Marks the command allocator’s heaps for reuse.

## Declaration

```swift
func reset()
```

## Discussion

Calling this method allows new [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) to reuse its existing internal memory heaps to encode new GPU commands.

You are responsible to ensure that all command buffers with memory originating from this allocator instance are complete before calling resetting it.
