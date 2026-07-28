# remoteStorageBuffer

*Instance Property · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtlbuffer/remotestoragebuffer>

The buffer on another GPU that the buffer was created from, if any.

## Declaration

```swift
var remoteStorageBuffer: (any MTLBuffer)? { get }
```

## Discussion

If the value of this property is non-`nil`, it contains a reference to the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that created this buffer. If the buffer isn’t a remote view, the value of this property is `nil`.

You can use remote views only as a source for copy commands encoded by an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder).

## See also

### Creating views of buffers on other GPUs
- [makeRemoteBufferView(_:)](https://developer.apple.com/documentation/metal/mtlbuffer/makeremotebufferview(_:)) — Creates a remote view of the buffer for another GPU in the same peer group.
