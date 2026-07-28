# MTLOrigin

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlorigin>

The coordinates for the front upper-left corner of a region.

## Declaration

```swift
struct MTLOrigin
```

## Topics

### Creating origin points
- [init()](https://developer.apple.com/documentation/metal/mtlorigin/init()) — Initializes a new origin.
- [init(x:y:z:)](https://developer.apple.com/documentation/metal/mtlorigin/init(x:y:z:)) — Initializes a new origin with the specified coordinates.
- [MTLOriginMake(_:_:_:)](https://developer.apple.com/documentation/metal/mtloriginmake(_:_:_:)) — Returns a new origin with the specified coordinates.

### Getting and setting coordinate values
- [x](https://developer.apple.com/documentation/metal/mtlorigin/x) — The x coordinate of the origin.
- [y](https://developer.apple.com/documentation/metal/mtlorigin/y) — The y coordinate of the origin.
- [z](https://developer.apple.com/documentation/metal/mtlorigin/z) — The z coordinate of the origin.

## See also

### Indirect compute commands
- [MTLIndirectComputeCommand](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand) — A compute command in an indirect command buffer.
- [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) — The bounds for a subset of an instance’s elements.
- [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) — A type that represents one, two, or three dimensions of a type instance, such as an array or texture.
- [MTLStageInRegionIndirectArguments](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments) — The data layout required for the arguments needed to specify the stage-in region.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
