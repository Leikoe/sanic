# textureBarrier()

*Instance Method · macOS 10.11*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/texturebarrier()>

Adds a barrier, which forces any texture read operations to wait until write operations to the same texture finish.

## Declaration

```swift
func textureBarrier()
```

## Discussion

Use a barrier if you use the same texture for both an input to a shader and as a rendering destination for the render pass.

A barrier let’s your app safely write to and then correctly read from the same texture. The barrier ensures that the draw calls before the barrier finish their write operations before any draw calls after the barrier read from the texture.

## See also

### Deprecated methods
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [use(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [use(_:count:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [use(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.
- [use(_:count:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.
