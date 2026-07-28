# setStageInRegion(_:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setstageinregion(_:)>

Sets the dimensions over the thread grid of how your compute kernel receives stage-in arguments.

## Declaration

```swift
func setStageInRegion(_ region: MTLRegion)
```

## Parameters

- **region** — The [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) defining how to interpret a thread’s location as a coordinate for stage-in data.

## Discussion

The region’s origin point, starting from `(0,0,0)` in the upper left of the bound data, determines the final index of `[[stage_in]]` data. Note that the total number of threads Metal launches may be larger than your stage-in data.

To determine the index used to fetch `[[stage_in]]` data for a given thread, the GPU adds the values specified by the region’s origin to the thread position in the grid. Threads in the grid outside of the maximum stage-in data size have undefined behavior when accessing the stage-in memory region.

## See also

### Configuring stage-in data
- [setStageInRegionWithIndirectBuffer(_:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setstageinregionwithindirectbuffer(_:indirectbufferoffset:)) — Sets the region of the stage-in attributes to apply to a compute kernel using an indirect buffer.
