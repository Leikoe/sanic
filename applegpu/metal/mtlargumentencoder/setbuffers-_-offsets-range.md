# setBuffers(_:offsets:range:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffers(_:offsets:range:)>

Encodes references to an array of buffers into the argument buffer.

## Declaration

```swift
func setBuffers(_ buffers: [(any MTLBuffer)?], offsets: [Int], range: Range<Int>)
```

## Parameters

- **buffers** — An array of buffers the method encodes.
- **offsets** — An array of byte offsets for each element in `buffers`.
- **range** — A range of indices within the argument buffer for each element in `buffers`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding buffers
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffer(_:offset:index:)) — Encodes a reference to a buffer into the argument buffer.
