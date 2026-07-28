# threadgroupSizeMatchesTileSize

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/threadgroupsizematchestilesize>

A Boolean value that indicates whether all threadgroups for this pipeline completely cover tiles.

## Declaration

```swift
var threadgroupSizeMatchesTileSize: Bool { get set }
```

## Discussion

Metal can optimize code generation when the threadgroup and tile sizes match.

## See also

### Specifying rasterization and visibility state
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/rastersamplecount) — The number of samples in each fragment.
