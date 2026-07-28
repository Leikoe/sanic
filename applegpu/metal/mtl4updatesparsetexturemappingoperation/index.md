# MTL4UpdateSparseTextureMappingOperation

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation>

Groups together arguments for an operation to update a sparse texture mapping.

## Declaration

```swift
struct MTL4UpdateSparseTextureMappingOperation
```

## Overview

When performing a sparse mapping update, you are responsible for issuing a barrier against stage `MTLStageResourceState`.

You can determine the sparse texture tier by calling [sparseTextureTier](https://developer.apple.com/documentation/metal/mtltexture/sparsetexturetier).

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/init())
- [init(mode:textureRegion:textureLevel:textureSlice:heapOffset:)](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/init(mode:textureregion:texturelevel:textureslice:heapoffset:))

### Instance Properties
- [heapOffset](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/heapoffset) — The starting offset in the heap, in tiles.
- [mode](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/mode) — The mode of the mapping operation to perform.
- [textureLevel](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/texturelevel) — The index of the mipmap level in the texture to update.
- [textureRegion](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/textureregion) — The region in the texture to update, in tiles.
- [textureSlice](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/textureslice) — The index of the array slice in the texture to update.

## See also

### Sparse resources
- [MTLBufferSparseTier](https://developer.apple.com/documentation/metal/mtlbuffersparsetier) — Enumerates the different support levels for sparse buffers.
- [MTL4CopySparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation) — Groups together arguments for an operation to copy a sparse buffer mapping.
- [MTL4UpdateSparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation) — Groups together arguments for an operation to update a sparse buffer mapping.
- [MTLTextureSparseTier](https://developer.apple.com/documentation/metal/mtltexturesparsetier) — Enumerates the different support levels for sparse textures.
- [MTL4CopySparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation) — Groups together arguments for an operation to copy a sparse texture mapping.
