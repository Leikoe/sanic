# MTLBufferLayoutDescriptor

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor>

A description of how a compute function fetches input data for an attribute.

## Declaration

```swift
class MTLBufferLayoutDescriptor
```

## Topics

### Describing fetch behavior
- [stride](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stride) — The number of bytes from one buffer entry to the next.
- [stepFunction](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stepfunction) — Determines how and when compute functions fetch data.
- [stepRate](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/steprate) — How frequently the step function should load data.
- [MTLStepFunction](https://developer.apple.com/documentation/metal/mtlstepfunction) — The frequency and locations at which a function fetches attribute data.

## See also

### Configuring compute pass inputs
- [stageInputDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/stageinputdescriptor) — The organization of input and output data for the next kernel call.
- [MTLAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlattributedescriptor) — A descriptor of an argument’s format and where its data is in memory.
- [MTLAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray) — An array of attribute descriptor objects.
- [MTLBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray) — An array of buffer layout descriptor objects.
