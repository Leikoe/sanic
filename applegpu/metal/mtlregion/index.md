# MTLRegion

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlregion>

The bounds for a subset of an instance’s elements.

## Declaration

```swift
struct MTLRegion
```

## Overview

Metal has many instance types that represent arrays of discrete elements. For example, a texture has an array of pixel elements, and a thread grid has an array of computational threads. Use [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) instances to describe subsets of these instances.

The origin is the front upper-left corner of the region, and its extents go towards the back lower-right corner. Conceptually, when using an [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) instance to describe a subset of an instance, treat the instance as a 3D array of elements, even if it has fewer dimensions. For a 2D instance, set the z coordinate of the origin to `0` and the depth to `1`. For a 1D instance, set the y and z coordinates of the origin to `0` and the height and depth to `1`.

## Topics

### Creating regions
- [init()](https://developer.apple.com/documentation/metal/mtlregion/init()) — Initializes a new region.
- [init(origin:size:)](https://developer.apple.com/documentation/metal/mtlregion/init(origin:size:)) — Initializes a new region with the specified origin and size.
- [MTLRegionMake1D(_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake1d(_:_:)) — Creates a 3D representation of a 1D region.
- [MTLRegionMake2D(_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake2d(_:_:_:_:)) — Creates a 3D representation of a 2D region.
- [MTLRegionMake3D(_:_:_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlregionmake3d(_:_:_:_:_:_:)) — Creates a 3D region.

### Getting and setting region information
- [origin](https://developer.apple.com/documentation/metal/mtlregion/origin) — The coordinates of the front upper-left corner of the region.
- [size](https://developer.apple.com/documentation/metal/mtlregion/size) — The dimensions of the region.

## See also

### Indirect compute commands
- [MTLIndirectComputeCommand](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand) — A compute command in an indirect command buffer.
- [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) — A type that represents one, two, or three dimensions of a type instance, such as an array or texture.
- [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) — The coordinates for the front upper-left corner of a region.
- [MTLStageInRegionIndirectArguments](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments) — The data layout required for the arguments needed to specify the stage-in region.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
