# MTLRegionMake3D(_:_:_:_:_:_:)

*Function · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlregionmake3d(_:_:_:_:_:_:)>

Creates a 3D region.

## Declaration

```swift
func MTLRegionMake3D(_ x: Int, _ y: Int, _ z: Int, _ width: Int, _ height: Int, _ depth: Int) -> MTLRegion
```

## Parameters

- **x** — The x coordinate of the origin.
- **y** — The y coordinate of the origin.
- **z** — The z coordinate of the origin.
- **width** — The width of the volume.
- **height** — The height of the volume.
- **depth** — The depth of the volume.

## Return Value

A 3D region with the specified values.

## See also

### Creating regions
- [init()](https://developer.apple.com/documentation/metal/mtlregion/init()) — Initializes a new region.
- [init(origin:size:)](https://developer.apple.com/documentation/metal/mtlregion/init(origin:size:)) — Initializes a new region with the specified origin and size.
- [MTLRegionMake1D(_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake1d(_:_:)) — Creates a 3D representation of a 1D region.
- [MTLRegionMake2D(_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake2d(_:_:_:_:)) — Creates a 3D representation of a 2D region.
