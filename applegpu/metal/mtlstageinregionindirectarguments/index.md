# MTLStageInRegionIndirectArguments

*Structure · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments>

The data layout required for the arguments needed to specify the stage-in region.

## Declaration

```swift
struct MTLStageInRegionIndirectArguments
```

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments/init())
- [init(stageInOrigin:stageInSize:)](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments/init(stageinorigin:stageinsize:))

### Instance Properties
- [stageInOrigin](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments/stageinorigin) — The location of the upper-left corner of the block.
- [stageInSize](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments/stageinsize) — The size of the block.

## See also

### Indirect compute commands
- [MTLIndirectComputeCommand](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand) — A compute command in an indirect command buffer.
- [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) — The bounds for a subset of an instance’s elements.
- [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) — A type that represents one, two, or three dimensions of a type instance, such as an array or texture.
- [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) — The coordinates for the front upper-left corner of a region.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
