# MTLBlitPassDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitpassdescriptor>

A configuration you create to customize a blit command encoder, which affects the runtime behavior of the blit pass you encode with it.

## Declaration

```swift
class MTLBlitPassDescriptor
```

## Overview

You can customize an encoder for a blit pass by creating and configuring an [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) instance and passing it to [makeBlitCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:)).

## Topics

### Configuring sample buffer attachment descriptors for a blit pass
- [sampleBufferAttachments](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor/samplebufferattachments) — An array of counter sample buffer attachments that you configure for a blit pass.

## See also

### Configuring a blit command encoder
- [MTLBlitPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a blit pass.
- [MTLBlitPassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a blit pass.
