# MTLSize

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlsize>

A type that represents one, two, or three dimensions of a type instance, such as an array or texture.

## Declaration

```swift
struct MTLSize
```

## Overview

Metal has many types that represent arrays of discrete elements, such as:

- A texture, which has an array of pixel elements

- A thread grid, which has an array of computational threads

Types and methods that work with these array-like types frequently have an [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) property or parameter that refers to the extents of a specific instance of the type, or a region within the instance.

> **Important:**
> Treat each size instance as a measure of something in 3D, even if it represents something with only one or two dimensions, by assigning `1` to the irrelevant dimensions.

The following are some examples for setting a size for an instance that has less than three dimentions:

- For a 2D texture that has a height and width of `5`, set a size’s [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) property to `1` so that it represents `[5, 5, 1]`.

- For a 1D array with length `42`, set a size’s [height](https://developer.apple.com/documentation/metal/mtlsize/height), [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) properties to `1`, so that it represents `[42, 1, 1]`.

## Topics

### Creating a size instance
- [init()](https://developer.apple.com/documentation/metal/mtlsize/init()) — Creates a default size instance by setting the initial values for its width, height, and depth properties to zero.
- [init(width:height:depth:)](https://developer.apple.com/documentation/metal/mtlsize/init(width:height:depth:)) — Creates a size instance with values for its width, height, and depth properties.
- [MTLSizeMake(_:_:_:)](https://developer.apple.com/documentation/metal/mtlsizemake(_:_:_:)) — Creates a size instance with values for its width, height, and depth properties.

### Accessing a size’s dimensions
- [width](https://developer.apple.com/documentation/metal/mtlsize/width) — A value for the x-axis dimension.
- [height](https://developer.apple.com/documentation/metal/mtlsize/height) — A value for the y-axis dimension.
- [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) — A value for the z-axis dimension.

## See also

### Indirect compute commands
- [MTLIndirectComputeCommand](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand) — A compute command in an indirect command buffer.
- [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) — The bounds for a subset of an instance’s elements.
- [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) — The coordinates for the front upper-left corner of a region.
- [MTLStageInRegionIndirectArguments](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments) — The data layout required for the arguments needed to specify the stage-in region.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
