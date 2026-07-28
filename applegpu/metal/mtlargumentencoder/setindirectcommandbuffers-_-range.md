# setIndirectCommandBuffers(_:range:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.14, tvOS 12.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffers(_:range:)>

Encodes an array of indirect command buffers into the argument buffer.

## Declaration

```swift
func setIndirectCommandBuffers(_ buffers: [(any MTLIndirectCommandBuffer)?], range: Range<Int>)
```

## Parameters

- **buffers** — An array of indirect command buffers the method encodes.
- **range** — A range of indices within the argument buffer for each element in `buffers`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding indirect command buffers
- [setIndirectCommandBuffer(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffer(_:index:)) — Encodes a reference to an indirect command buffer into the argument buffer.
