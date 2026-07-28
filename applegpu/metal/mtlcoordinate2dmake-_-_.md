# MTLCoordinate2DMake(_:_:)

*Function · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlcoordinate2dmake(_:_:)>

Returns a new 2D point with the specified coordinates.

## Declaration

```swift
func MTLCoordinate2DMake(_ x: Float, _ y: Float) -> MTLCoordinate2D
```

## Parameters

- **x** — The x coordinate of the new point.
- **y** — The y coordinate of the new point.

## See also

### Rasterization settings
- [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates) — Configure a rasterization rate map to vary rasterization rates depending on the amount of detail needed.
- [Creating a rasterization rate map](https://developer.apple.com/documentation/metal/creating-a-rasterization-rate-map) — Define the rasterization rates for each part of your render target.
- [Rendering with a rasterization rate map](https://developer.apple.com/documentation/metal/rendering-with-a-rasterization-rate-map) — Create offscreen textures to hold intermediate rasterized data.
- [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content) — Use the rate map data to scale the content to fill your destination texture.
- [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) — An object that you use to configure new rasterization rate maps.
- [MTLRasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrasterizationratemap) — A compiled read-only instance that determines how to apply variable rasterization rates when rendering.
- [MTLCoordinate2D](https://developer.apple.com/documentation/metal/mtlcoordinate2d) — A coordinate in the viewport.
