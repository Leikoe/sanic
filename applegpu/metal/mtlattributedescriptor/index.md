# MTLAttributeDescriptor

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlattributedescriptor>

A descriptor of an argument’s format and where its data is in memory.

## Declaration

```swift
class MTLAttributeDescriptor
```

## Overview

Attribute descriptors are part of an [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) or [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) instance to provide layout information about a function’s arguments. Each descriptor is for a single argument, containing information about the attached data, offset and stride, and data type.

## Topics

### Defining attribute location
- [bufferIndex](https://developer.apple.com/documentation/metal/mtlattributedescriptor/bufferindex) — The index in the buffer argument table for the buffer that contains the data for this attribute.
- [offset](https://developer.apple.com/documentation/metal/mtlattributedescriptor/offset) — The offset, in bytes, from the start of the buffer that contains the attribute data to the start of the data itself.
- [format](https://developer.apple.com/documentation/metal/mtlattributedescriptor/format) — The format of the attribute’s data.
- [MTLAttributeFormat](https://developer.apple.com/documentation/metal/mtlattributeformat) — The data format options for acceleration structures.

## See also

### Configuring compute pass inputs
- [stageInputDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/stageinputdescriptor) — The organization of input and output data for the next kernel call.
- [MTLAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray) — An array of attribute descriptor objects.
- [MTLBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor) — A description of how a compute function fetches input data for an attribute.
- [MTLBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray) — An array of buffer layout descriptor objects.
