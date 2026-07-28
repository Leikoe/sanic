# makeTexture(descriptor:offset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:offset:)>

Creates a texture at a specified offset on the heap.

## Declaration

```swift
func makeTexture(descriptor: MTLTextureDescriptor, offset: Int) -> (any MTLTexture)?
```

## Parameters

- **descriptor** — A descriptor object that describes the properties of the texture.
- **offset** — The distance, in bytes, to place the texture relative to the start of the heap.

## Return Value

A new texture, or `nil` if the heap is not a placement heap.

## Discussion

You can call the method with the following restrictions:

- The heap’s type needs to be [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement)

- The texture’s CPU cache mode option needs to match the heap’s [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) property

- The texture’s storage mode option needs to be [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless), or match the heap’s [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) property

> **Important:**
>  Avoid potentially erratic behavior by aligning the texture correctly so that it doesn’t extend past the end of the heap.

Use the [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) to determine the correct size and alignment.

> **Note:**
>  The new texture can implicitly alias the underlying memory of other resources already in the heap within the overlapping half-open range of `[offset, offset + requiredSize)`.

## See also

### Creating textures from a heap
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:)) — Creates a texture on the heap.
