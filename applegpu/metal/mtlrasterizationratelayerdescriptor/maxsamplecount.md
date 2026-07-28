# maxSampleCount

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/maxsamplecount>

The maximum number of rows and columns in the layer map.

## Declaration

```swift
var maxSampleCount: MTLSize { get }
```

## Discussion

Its [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) value is always `0`.

## See also

### Inspecting the layer rate function parameters
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/samplecount) — The number of rows and columns in the layer map.
- [horizontal](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/horizontal) — The horizontal rasterization rates for the layer map’s rows.
- [vertical](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/vertical) — The vertical rasterization rates for the layer map’s rows.
- [MTLRasterizationRateSampleArray](https://developer.apple.com/documentation/metal/mtlrasterizationratesamplearray) — An array instance that contains rasterization rates.
