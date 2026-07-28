# makeArgumentEncoderForBuffer(atIndex:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/makeargumentencoderforbuffer(atindex:)>

Creates a new argument encoder for a nested argument buffer.

## Declaration

```swift
func makeArgumentEncoderForBuffer(atIndex index: Int) -> (any MTLArgumentEncoder)?
```

## Parameters

- **index** — The index of a nested argument-buffer within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## Return Value

An argument encoder targeting the nested argument buffer.

## Discussion

If an argument buffer contains nested argument buffers in its structure, then each nested argument buffer needs to use its own [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) object to encode its individual resources.
