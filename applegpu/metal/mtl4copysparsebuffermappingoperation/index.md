# MTL4CopySparseBufferMappingOperation

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation>

Groups together arguments for an operation to copy a sparse buffer mapping.

## Declaration

```swift
struct MTL4CopySparseBufferMappingOperation
```

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation/init())
- [init(sourceRange:destinationOffset:)](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation/init(sourcerange:destinationoffset:))

### Instance Properties
- [destinationOffset](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation/destinationoffset) — The origin in the destination buffer, in tiles.
- [sourceRange](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation/sourcerange) — The range in the source buffer, in tiles.

## See also

### Sparse resources
- [MTLBufferSparseTier](https://developer.apple.com/documentation/metal/mtlbuffersparsetier) — Enumerates the different support levels for sparse buffers.
- [MTL4UpdateSparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation) — Groups together arguments for an operation to update a sparse buffer mapping.
- [MTLTextureSparseTier](https://developer.apple.com/documentation/metal/mtltexturesparsetier) — Enumerates the different support levels for sparse textures.
- [MTL4CopySparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation) — Groups together arguments for an operation to copy a sparse texture mapping.
- [MTL4UpdateSparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation) — Groups together arguments for an operation to update a sparse texture mapping.
