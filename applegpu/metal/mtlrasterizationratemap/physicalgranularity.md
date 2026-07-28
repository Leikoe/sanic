# physicalGranularity

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalgranularity>

The granularity, in physical pixels, at which the rasterization rate varies.

## Declaration

```swift
var physicalGranularity: MTLSize { get }
```

## Discussion

If you’re using a rendering algorithm that uses binning or tiling to partition the rendered image, you may want to use the value of this property to determine your bin sizes.

The depth component of the returned [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) structure is always `0`.

## See also

### Inspecting geometric and rendering properties
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/layercount) — The number of layers in the rate map.
- [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screensize) — The logical size, in pixels, of the viewport coordinate system.
- [physicalSize(layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalsize(layer:)) — Returns the dimensions, in pixels, of the area in the render target affected by the rasterization rate map.
