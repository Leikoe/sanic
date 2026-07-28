# placementSparsePageSize

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/placementsparsepagesize>

Determines the page size for a placement sparse texture.

## Declaration

```swift
var placementSparsePageSize: MTLSparsePageSize { get set }
```

## Discussion

Set this property to a non-zero value to create a *placement sparse texture*.

Placement sparse textures are instances of [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) that you assign memory to using a [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance of type [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement) and a [maxCompatiblePlacementSparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/maxcompatibleplacementsparsepagesize) at least as large as the [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) value you assign to this property.

This value is 0 by default.
