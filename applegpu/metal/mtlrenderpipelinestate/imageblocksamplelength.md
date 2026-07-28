# imageblockSampleLength

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/imageblocksamplelength>

The memory size, in byes, of the render pipeline’s imageblock for a single sample.

## Declaration

```swift
var imageblockSampleLength: Int { get }
```

## See also

### Checking tile shader memory requirements
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadsperthreadgroup) — The largest number of threads the pipeline state can have in a single tile shader threadgroup.
- [threadgroupSizeMatchesTileSize](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/threadgroupsizematchestilesize) — A Boolean value that indicates whether the pipeline state needs a threadgroup’s size to equal a tile’s size.
- [imageblockMemoryLength(forDimensions:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/imageblockmemorylength(fordimensions:)) — Returns the length of an imageblock’s memory for the specified imageblock dimensions.
