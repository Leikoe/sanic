# remoteStorageTexture

*Instance Property · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtltexture/remotestoragetexture>

The texture on another GPU that the texture was created from, if any.

## Declaration

```swift
var remoteStorageTexture: (any MTLTexture)? { get }
```

## Discussion

If the value of this property is non-`nil`, it contains a reference to the [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance that created this texture. If the texture isn’t a remote view, the value of this property is `nil`.

You can use remote views only as the source for copy commands encoded by an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder).

## See also

### Creating views of textures on other GPUs
- [makeRemoteTextureView(_:)](https://developer.apple.com/documentation/metal/mtltexture/makeremotetextureview(_:)) — Creates a remote texture view for another GPU in the same peer group.
