# physicalSize(layer:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalsize(layer:)>

Returns the dimensions, in pixels, of the area in the render target affected by the rasterization rate map.

## Declaration

```swift
func physicalSize(layer layerIndex: Int) -> MTLSize
```

## Parameters

- **layerIndex** — The index of the layer.

## Return Value

The dimensions, in pixels, of the area in the render target affected by the rasterization rate map.

## Discussion

Your render targets should be at least as large as the physical size returned by this method. Each layer may have different rasterization rates and therefore different physical size requirements.

## See also

### Inspecting geometric and rendering properties
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/layercount) — The number of layers in the rate map.
- [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screensize) — The logical size, in pixels, of the viewport coordinate system.
- [physicalGranularity](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalgranularity) — The granularity, in physical pixels, at which the rasterization rate varies.
