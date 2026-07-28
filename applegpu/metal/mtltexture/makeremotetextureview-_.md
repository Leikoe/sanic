# makeRemoteTextureView(_:)

*Instance Method · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtltexture/makeremotetextureview(_:)>

Creates a remote texture view for another GPU in the same peer group.

## Declaration

```swift
func makeRemoteTextureView(_ device: any MTLDevice) -> (any MTLTexture)?
```

## Discussion

The device instance that created this texture and the device instance passed into this method need to have the same nonzero peer group identifier ([peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid)). This texture needs to either use the private storage mode ([MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private)) or be backed by an [IOSurface](https://developer.apple.com/documentation/IOSurface/IOSurface).

A remote view doesn’t allocate any storage for the new texture; it references the memory allocated for the original texture. You can use remote views only as a source for copy commands encoded by an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder). For more information, see [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus).

## See also

### Creating views of textures on other GPUs
- [remoteStorageTexture](https://developer.apple.com/documentation/metal/mtltexture/remotestoragetexture) — The texture on another GPU that the texture was created from, if any.
