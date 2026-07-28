# setBuffer(_:offset:for:)

*Instance Method · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorbufferattachments/setbuffer(_:offset:for:)>

Sets the buffer and byte offset to use as backing storage for the given plane.

## Declaration

```swift
func setBuffer(_ buffer: any MTLBuffer, offset: Int, for plane: MTLTensorPlaneType)
```

## Parameters

- **buffer** — The buffer to back the plane.
- **offset** — The byte offset into the buffer.
- **plane** — The plane type to associate the buffer with.

## Discussion

The offset needs to be aligned to 128 bytes if the plane uses [MTLTensorDataType.int2](https://developer.apple.com/documentation/metal/mtltensordatatype/int2), [MTLTensorDataType.uint2](https://developer.apple.com/documentation/metal/mtltensordatatype/uint2), [MTLTensorDataType.int4](https://developer.apple.com/documentation/metal/mtltensordatatype/int4), [MTLTensorDataType.uint4](https://developer.apple.com/documentation/metal/mtltensordatatype/uint4), [MTLTensorDataType.metalFloat4e2m1](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat4e2m1), [MTLTensorDataType.metalFloat8e4m3](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e4m3), [MTLTensorDataType.metalFloat8e5m2](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e5m2), or [MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0), otherwise it needs to be aligned to the size of the plane’s data type in bytes.
