# refitScratchBufferSize

*Instance Property · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/refitscratchbuffersize>

The amount of scratch memory, in bytes, the GPU device needs to refit the acceleration structure.

## Declaration

```swift
var refitScratchBufferSize: Int
```

## Discussion

This value can be zero, which indicates that refitting the acceleration structure doesn’t require a scratch buffer.

## See also

### Retrieving the sizes
- [accelerationStructureSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/accelerationstructuresize) — The size of the acceleration structure, in bytes.
- [buildScratchBufferSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/buildscratchbuffersize) — The amount of scratch memory, in bytes, the GPU devices needs to build the acceleration structure.
