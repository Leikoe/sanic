# MTL4UpdateSparseBufferMappingOperation

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation>

Groups together arguments for an operation to update a sparse buffer mapping.

## Declaration

```swift
struct MTL4UpdateSparseBufferMappingOperation
```

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation/init())
- [init(mode:bufferRange:heapOffset:)](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation/init(mode:bufferrange:heapoffset:))

### Instance Properties
- [bufferRange](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation/bufferrange) — The range in the buffer, in tiles.
- [heapOffset](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation/heapoffset) — The starting offset in the heap, in tiles.
- [mode](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation/mode) — The mode of the mapping operation to perform.

## See also

### Sparse resources
- [MTLBufferSparseTier](https://developer.apple.com/documentation/metal/mtlbuffersparsetier) — Enumerates the different support levels for sparse buffers.
- [MTL4CopySparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation) — Groups together arguments for an operation to copy a sparse buffer mapping.
- [MTLTextureSparseTier](https://developer.apple.com/documentation/metal/mtltexturesparsetier) — Enumerates the different support levels for sparse textures.
- [MTL4CopySparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation) — Groups together arguments for an operation to copy a sparse texture mapping.
- [MTL4UpdateSparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation) — Groups together arguments for an operation to update a sparse texture mapping.
