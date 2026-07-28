# updateMappings(texture:heap:operations:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/updatemappings(texture:heap:operations:)>

Updates multiple regions within a placement sparse texture to alias specific tiles of a Metal heap.

## Declaration

```swift
func updateMappings(texture: any MTLTexture, heap: (any MTLHeap)?, operations: [MTL4UpdateSparseTextureMappingOperation])
```

## Parameters

- **texture** — A placement sparse [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).
- **heap** — [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) you allocate with type [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement).
- **operations** — An array of [MTL4UpdateSparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation) instances to perform.

## Discussion

You can provide a `nil` parameter to the `heap` argument only if when you perform unmap operations. Otherwise, you are responsible for ensuring the heap is non-nil and has a [maxCompatiblePlacementSparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/maxcompatibleplacementsparsepagesize) of at least the texture’s [placementSparsePageSize](https://developer.apple.com/documentation/metal/mtltexturedescriptor/placementsparsepagesize).

When performing a sparse mapping update, you are responsible for issuing a barrier against stage `MTLStageResourceState`.

You can determine the sparse texture tier by calling `MTLTexture/sparseTextureTier`.
