# MTLRegionMake2D(_:_:_:_:)

*Function · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlregionmake2d(_:_:_:_:)>

Creates a 3D representation of a 2D region.

## Declaration

```swift
func MTLRegionMake2D(_ x: Int, _ y: Int, _ width: Int, _ height: Int) -> MTLRegion
```

## Parameters

- **x** — The x coordinate of the origin.
- **y** — The y coordinate of the origin.
- **width** — The width of the volume.
- **height** — The height of the volume.

## Return Value

A region whose x, y, width, and height values are as specified. The z coordinate of the region’s origin is set to `0`, and the region’s depth is set to `1`.

## See also

### Creating regions
- [init()](https://developer.apple.com/documentation/metal/mtlregion/init()) — Initializes a new region.
- [init(origin:size:)](https://developer.apple.com/documentation/metal/mtlregion/init(origin:size:)) — Initializes a new region with the specified origin and size.
- [MTLRegionMake1D(_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake1d(_:_:)) — Creates a 3D representation of a 1D region.
- [MTLRegionMake3D(_:_:_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake3d(_:_:_:_:_:_:)) — Creates a 3D region.
