# MTLBufferLayoutDescriptorArray

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray>

An array of buffer layout descriptor objects.

## Declaration

```swift
class MTLBufferLayoutDescriptorArray
```

## Overview

An [MTLBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray) defines the data layout and loading for compute data, using [MTLBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor) instances.

## Topics

### Array accessors
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray/subscript(_:)) — Returns the state of the specified buffer layout.

## See also

### Configuring compute pass inputs
- [stageInputDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/stageinputdescriptor) — The organization of input and output data for the next kernel call.
- [MTLAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlattributedescriptor) — A descriptor of an argument’s format and where its data is in memory.
- [MTLAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray) — An array of attribute descriptor objects.
- [MTLBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor) — A description of how a compute function fetches input data for an attribute.
