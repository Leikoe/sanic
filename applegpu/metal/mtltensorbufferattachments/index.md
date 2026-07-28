# MTLTensorBufferAttachments

*Class · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorbufferattachments>

An object that associates each plane of a tensor with a buffer and byte offset for buffer-backed tensor creation.

## Declaration

```swift
class MTLTensorBufferAttachments
```

## Topics

### Instance Methods
- [buffer(for:)](https://developer.apple.com/documentation/metal/mtltensorbufferattachments/buffer(for:)) — Returns the buffer backing the given plane, or `nil` if none has been set.
- [offset(for:)](https://developer.apple.com/documentation/metal/mtltensorbufferattachments/offset(for:)) — Returns the byte offset into the buffer for the given plane.
- [reset()](https://developer.apple.com/documentation/metal/mtltensorbufferattachments/reset()) — Empties the container of all its elements.
- [setBuffer(_:offset:for:)](https://developer.apple.com/documentation/metal/mtltensorbufferattachments/setbuffer(_:offset:for:)) — Sets the buffer and byte offset to use as backing storage for the given plane.
