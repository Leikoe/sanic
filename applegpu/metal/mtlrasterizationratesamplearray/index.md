# MTLRasterizationRateSampleArray

*Class · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray>

An array instance that contains rasterization rates.

## Declaration

```swift
class MTLRasterizationRateSampleArray
```

## Overview

The [horizontal](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/horizontal) and [vertical](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/vertical) properties of an [MTLRasterizationRateLayerDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor) point to [MTLRasterizationRateSampleArray](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray) instances that contains rasterization rates for the layer map. You can use array subscript syntax to access the samples. [MTLRasterizationRateSampleArray](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray) instances perform bounds checking on any memory operations you make to their sample data.

## Topics

### Accessing the array
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray/subscript(_:)) — Retrieves the sample value at the specified index.

## See also

### Inspecting the layer rate function parameters
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/samplecount) — The number of rows and columns in the layer map.
- [maxSampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/maxsamplecount) — The maximum number of rows and columns in the layer map.
- [horizontal](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/horizontal) — The horizontal rasterization rates for the layer map’s rows.
- [vertical](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/vertical) — The vertical rasterization rates for the layer map’s rows.
