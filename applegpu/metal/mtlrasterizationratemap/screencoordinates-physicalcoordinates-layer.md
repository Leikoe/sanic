# screenCoordinates(physicalCoordinates:layer:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screencoordinates(physicalcoordinates:layer:)>

Converts a point in physical coordinates inside a layer to its corresponding logical viewport coordinates.

## Declaration

```swift
func screenCoordinates(physicalCoordinates: MTLCoordinate2D, layer layerIndex: Int) -> MTLCoordinate2D
```

## Parameters

- **physicalCoordinates** — A point in layer coordinates.
- **layerIndex** — The index of the rate map to use.

## Return Value

A point in the view coordinates corresponding to the source point.

## Discussion

The returned coordinates are always greater than or equal to the input coordinates because the rasterization rate never exceeds 1:1 in any region.

## See also

### Converting between viewport and physical coordinates
- [physicalCoordinates(screenCoordinates:layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalcoordinates(screencoordinates:layer:)) — Converts a point in logical viewport coordinates to the corresponding physical coordinates in a render layer.
