# physicalCoordinates(screenCoordinates:layer:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalcoordinates(screencoordinates:layer:)>

Converts a point in logical viewport coordinates to the corresponding physical coordinates in a render layer.

## Declaration

```swift
func physicalCoordinates(screenCoordinates: MTLCoordinate2D, layer layerIndex: Int) -> MTLCoordinate2D
```

## Parameters

- **screenCoordinates** — A point in viewport coordinates.
- **layerIndex** — The index of the rate map to use.

## Return Value

A point in the layer’s physical coordinate system corresponding to the source point.

## Discussion

The returned coordinates are always less than or equal to the input coordinates because the rasterization rate never exceeds 1:1 in any region.

## See also

### Converting between viewport and physical coordinates
- [screenCoordinates(physicalCoordinates:layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screencoordinates(physicalcoordinates:layer:)) — Converts a point in physical coordinates inside a layer to its corresponding logical viewport coordinates.
