# setIndirectCommandBuffer(_:index:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffer(_:index:)>

Encodes a reference to an indirect command buffer into the argument buffer.

## Declaration

```swift
func setIndirectCommandBuffer(_ indirectCommandBuffer: (any MTLIndirectCommandBuffer)?, index: Int)
```

## Parameters

- **indirectCommandBuffer** — An indirect command-buffer the method encodes.
- **index** — The index of an inline, constant-data argument within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding indirect command buffers
- [setIndirectCommandBuffers(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffers(_:range:)) — Encodes an array of indirect command buffers into the argument buffer.
