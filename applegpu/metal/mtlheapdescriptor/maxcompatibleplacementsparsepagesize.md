# maxCompatiblePlacementSparsePageSize

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/maxcompatibleplacementsparsepagesize>

Specifies the largest sparse page size that the Metal heap supports.

## Declaration

```swift
var maxCompatiblePlacementSparsePageSize: MTLSparsePageSize { get set }
```

## Discussion

This parameter only affects the heap if you set the [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) property of this descriptor to [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement).

The value you assign to this property determines the compatibility of the Metal heap with with placement sparse resources, because placement sparse resources require that their sparse page size be less than or equal to the placement sparse page of the Metal heap that this property controls.
