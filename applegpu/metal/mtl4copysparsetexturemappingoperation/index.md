# MTL4CopySparseTextureMappingOperation

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation>

Groups together arguments for an operation to copy a sparse texture mapping.

## Declaration

```swift
struct MTL4CopySparseTextureMappingOperation
```

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/init())
- [init(sourceRegion:sourceLevel:sourceSlice:destinationOrigin:destinationLevel:destinationSlice:)](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/init(sourceregion:sourcelevel:sourceslice:destinationorigin:destinationlevel:destinationslice:))

### Instance Properties
- [destinationLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationlevel) — The index of the mipmap level in the destination texture.
- [destinationOrigin](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationorigin) — The origin in the destination texture to copy into, in tiles.
- [destinationSlice](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationslice) — The index of the array slice in the destination texture to copy into.
- [sourceLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourcelevel) — The index of the mipmap level in the source texture.
- [sourceRegion](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourceregion) — The region in the source texture, in tiles.
- [sourceSlice](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourceslice) — The index of the array slice in the texture source of the copy operation.

## See also

### Sparse resources
- [MTLBufferSparseTier](https://developer.apple.com/documentation/metal/mtlbuffersparsetier) — Enumerates the different support levels for sparse buffers.
- [MTL4CopySparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation) — Groups together arguments for an operation to copy a sparse buffer mapping.
- [MTL4UpdateSparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation) — Groups together arguments for an operation to update a sparse buffer mapping.
- [MTLTextureSparseTier](https://developer.apple.com/documentation/metal/mtltexturesparsetier) — Enumerates the different support levels for sparse textures.
- [MTL4UpdateSparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation) — Groups together arguments for an operation to update a sparse texture mapping.
