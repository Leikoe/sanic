# MTLResourceViewPool

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlresourceviewpool>

Contains views over resources of a specific type, and allows you to manage those views.

## Declaration

```swift
protocol MTLResourceViewPool : NSObjectProtocol
```

## Topics

### Instance Properties
- [baseResourceID](https://developer.apple.com/documentation/metal/mtlresourceviewpool/baseresourceid) — Obtains the resource ID corresponding to the resource view at index 0 in this resource view pool.
- [device](https://developer.apple.com/documentation/metal/mtlresourceviewpool/device) — Obtains a reference to the GPU device this pool belongs to.
- [label](https://developer.apple.com/documentation/metal/mtlresourceviewpool/label) — Queries the optional debug label of this resource view pool.
- [resourceViewCount](https://developer.apple.com/documentation/metal/mtlresourceviewpool/resourceviewcount) — Queries the number of resource views that this pool contains.

### Instance Methods
- [copyResourceViews(sourcePool:sourceRange:destinationIndex:)](https://developer.apple.com/documentation/metal/mtlresourceviewpool/copyresourceviews(sourcepool:sourcerange:destinationindex:)) — Copies a range of resource views from a source view pool to a destination location in this view pool.

## See also

### View pools
- [MTLResourceViewPoolDescriptor](https://developer.apple.com/documentation/metal/mtlresourceviewpooldescriptor) — Provides parameters for creating a resource view pool.
- [MTLTextureViewPool](https://developer.apple.com/documentation/metal/mtltextureviewpool) — A pool of lightweight texture views.
- [MTLTextureViewDescriptor](https://developer.apple.com/documentation/metal/mtltextureviewdescriptor)
