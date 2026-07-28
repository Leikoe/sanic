# encodedLength

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/encodedlength>

The number of bytes required to store the encoded resources of an argument buffer.

## Declaration

```swift
var encodedLength: Int { get }
```

## Discussion

After creating an [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) instance, use this value to create the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that represents an argument buffer.

```swift
id <MTLArgumentEncoder> encoder = [_function newArgumentEncoderWithBufferIndex:0];
id <MTLBuffer> buffer = [_device newBufferWithLength:encoder.encodedLength options:_options];
[encoder setArgumentBuffer:buffer offset:0];
```

## See also

### Creating an argument buffer
- [setArgumentBuffer(_:offset:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:offset:)) — Specifies the position in a buffer where the encoder writes argument data.
- [setArgumentBuffer(_:startOffset:arrayElement:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:startoffset:arrayelement:)) — Specifies an array element within a buffer where the encoder writes argument data.
