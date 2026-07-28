# device

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/device>

The device object that created the argument encoder.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

You can only use the encoder to encode data into buffers created by the same Metal device object.

## See also

### Identifying the argument encoder
- [label](https://developer.apple.com/documentation/metal/mtlargumentencoder/label) — A string that identifies the argument buffer.
