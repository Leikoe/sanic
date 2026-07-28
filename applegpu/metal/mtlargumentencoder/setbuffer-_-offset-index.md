# setBuffer(_:offset:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffer(_:offset:index:)>

Encodes a reference to a buffer into the argument buffer.

## Declaration

```swift
func setBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — A buffer the method encodes.
- **offset** — A byte offset for `buffer`.
- **index** — The index of a buffer within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding buffers
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffers(_:offsets:range:)) — Encodes references to an array of buffers into the argument buffer.
