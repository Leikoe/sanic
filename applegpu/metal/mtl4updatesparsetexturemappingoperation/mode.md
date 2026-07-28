# mode

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/mode>

The mode of the mapping operation to perform.

## Declaration

```swift
var mode: MTLSparseTextureMappingMode
```

## Discussion

When mode is [MTLSparseTextureMappingMode.map](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/map), Metal walks the tiles in the region in X, Y, then Z order, assigning the next tile from the heap in increasing order, starting at [heapOffset](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/heapoffset).

When mode is [MTLSparseTextureMappingMode.unmap](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/unmap), Metal unmaps the tiles in the region, ignoring the contents of member [heapOffset](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/heapoffset).
