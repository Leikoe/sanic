# MTLBlitPassSampleBufferAttachmentDescriptorArray

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray>

A container that stores an array of sample buffer attachments for a blit pass.

## Declaration

```swift
class MTLBlitPassSampleBufferAttachmentDescriptorArray
```

## Overview

The number of elements in the array is at least the number of elements in an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) property.

## Topics

### Accessing a sample buffer attachment descriptor
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray/subscript(_:)) — Accesses one of the array’s blit pass sample buffer attachment descriptor instances.

## See also

### Configuring a blit command encoder
- [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) — A configuration you create to customize a blit command encoder, which affects the runtime behavior of the blit pass you encode with it.
- [MTLBlitPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a blit pass.
