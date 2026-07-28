# rasterizationRateMap

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rasterizationratemap>

The rasterization rate map to use when executing the render pass.

## Declaration

```swift
var rasterizationRateMap: (any MTLRasterizationRateMap)? { get set }
```

## Discussion

The default value is `nil`, which means that viewport coordinates are in the same coordinate system as the physical coordinates in the render target. Otherwise, Metal uses the rate map to convert between viewport coordinates and physical coordinates in the render target.
