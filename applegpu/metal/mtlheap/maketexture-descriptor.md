# makeTexture(descriptor:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:)>

Creates a texture on the heap.

## Declaration

```swift
func makeTexture(descriptor: MTLTextureDescriptor) -> (any MTLTexture)?
```

## Parameters

- **descriptor** — A descriptor object that describes the properties of the texture.

## Return Value

A new texture object backed by heap memory, or `nil` if the heap memory is full.

## Discussion

You can call the method with the following restrictions:

- The heap’s type needs to be [MTLHeapType.automatic](https://developer.apple.com/documentation/metal/mtlheaptype/automatic)

- The texture’s CPU cache mode option needs to match the heap’s [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) property

- The texture’s storage mode option needs to be [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless), or match the heap’s [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) property

## See also

### Creating textures from a heap
- [makeTexture(descriptor:offset:)](https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:offset:)) — Creates a texture at a specified offset on the heap.
