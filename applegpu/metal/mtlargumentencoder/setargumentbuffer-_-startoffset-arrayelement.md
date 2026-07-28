# setArgumentBuffer(_:startOffset:arrayElement:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:startoffset:arrayelement:)>

Specifies an array element within a buffer where the encoder writes argument data.

## Declaration

```swift
func setArgumentBuffer(_ argumentBuffer: (any MTLBuffer)?, startOffset: Int, arrayElement: Int)
```

## Parameters

- **argumentBuffer** — The destination buffer that represents an argument buffer.
- **startOffset** — The starting byte offset of the buffer data.
- **arrayElement** — The desired element of the argument buffer array targeted by encoding.

## See also

### Creating an argument buffer
- [setArgumentBuffer(_:offset:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:offset:)) — Specifies the position in a buffer where the encoder writes argument data.
- [encodedLength](https://developer.apple.com/documentation/metal/mtlargumentencoder/encodedlength) — The number of bytes required to store the encoded resources of an argument buffer.
