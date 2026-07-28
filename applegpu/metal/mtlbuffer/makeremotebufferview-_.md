# makeRemoteBufferView(_:)

*Instance Method · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtlbuffer/makeremotebufferview(_:)>

Creates a remote view of the buffer for another GPU in the same peer group.

## Declaration

```swift
func makeRemoteBufferView(_ device: any MTLDevice) -> (any MTLBuffer)?
```

## Discussion

The device instance that this buffer belongs to and the device you pass to the method both need to have the same nonzero peer group identifier ([peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid)). This buffer needs to use the private storage mode ([MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private)).

A remote view doesn’t allocate any storage for the new buffer; it references the memory allocated for the original buffer. You can use remote views only as a source for copy commands encoded by an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder). For more information, see [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus).

## See also

### Creating views of buffers on other GPUs
- [remoteStorageBuffer](https://developer.apple.com/documentation/metal/mtlbuffer/remotestoragebuffer) — The buffer on another GPU that the buffer was created from, if any.
