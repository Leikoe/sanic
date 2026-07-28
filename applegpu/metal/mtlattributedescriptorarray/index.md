# MTLAttributeDescriptorArray

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlattributedescriptorarray>

An array of attribute descriptor objects.

## Declaration

```swift
class MTLAttributeDescriptorArray
```

## Overview

An [MTLAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray) defines the data format and index binding for the attribute argument table, using [MTLAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlattributedescriptor) instances.

## Topics

### Accessing attribute state objects
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray/subscript(_:)) — Returns the state of the specified attribute.

## See also

### Configuring compute pass inputs
- [stageInputDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/stageinputdescriptor) — The organization of input and output data for the next kernel call.
- [MTLAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlattributedescriptor) — A descriptor of an argument’s format and where its data is in memory.
- [MTLBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor) — A description of how a compute function fetches input data for an attribute.
- [MTLBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray) — An array of buffer layout descriptor objects.
