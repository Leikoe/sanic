# sampleCount

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/samplecount>

The number of rows and columns in the layer map.

## Declaration

```swift
var sampleCount: MTLSize { get set }
```

## Discussion

The [sampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/samplecount) property splits the logical viewport coordinate space into a 2D grid of equal-sized cells. Its [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) value is always `0`.

The default value is the same as [maxSampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/maxsamplecount).

## See also

### Inspecting the layer rate function parameters
- [maxSampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/maxsamplecount) — The maximum number of rows and columns in the layer map.
- [horizontal](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/horizontal) — The horizontal rasterization rates for the layer map’s rows.
- [vertical](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/vertical) — The vertical rasterization rates for the layer map’s rows.
- [MTLRasterizationRateSampleArray](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray) — An array instance that contains rasterization rates.
