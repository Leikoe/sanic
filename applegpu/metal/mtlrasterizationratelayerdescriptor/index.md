# MTLRasterizationRateLayerDescriptor

*Class · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor>

The minimum rasterization rates to apply to sections of a layer in the render target.

## Declaration

```swift
class MTLRasterizationRateLayerDescriptor
```

## Overview

Use a layer map to divide the logical viewport coordinate system into a 2D grid of equal-sized rectangles, and choose different rasterization rates for each cell.

Specify rasterization rates using floating-point numbers between `0.0` and `1.0`, inclusive. A rate of `1.0` represents the normal rasterization rate, where each logical unit is equal to a physical pixel; a rate of `0.5` means that two logical units equate to one physical pixel, and so on. A value of `0.0` means that the GPU renders at its lowest quality level. When you create the map, the device object chooses the nearest rasterization rate supported by the GPU that meets or exceeds the rate you specified.

In the layer map, you provide separate rasterization rates for the grid’s rows and columns. The horizontal rates specify a horizontal rasterization rate for each column, and the vertical rates specify a vertical rasterization rate for each row. Each cell calculates its physical size in pixels by using the logical size of cells in the map, the horizontal rate from the cell’s column, and the vertical rate from its row.

## Topics

### Creating a layer rasterization rate descriptor
- [init(sampleCount:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(samplecount:)) — Initializes the layer map with an empty grid.
- [init(horizontal:vertical:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(horizontal:vertical:)) — Initializes a layer rate map with a set of horizontal and vertical rasterization rates.

### Inspecting the layer rate function parameters
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/samplecount) — The number of rows and columns in the layer map.
- [maxSampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/maxsamplecount) — The maximum number of rows and columns in the layer map.
- [horizontal](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/horizontal) — The horizontal rasterization rates for the layer map’s rows.
- [vertical](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/vertical) — The vertical rasterization rates for the layer map’s rows.
- [MTLRasterizationRateSampleArray](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray) — An array instance that contains rasterization rates.

## See also

### Accessing members of the array
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray/subscript(_:)) — Retrieves the sample value at the specified index.
