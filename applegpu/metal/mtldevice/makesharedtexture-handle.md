# makeSharedTexture(handle:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)>

Creates a texture that references a shared texture.

## Declaration

```swift
func makeSharedTexture(handle sharedHandle: MTLSharedTextureHandle) -> (any MTLTexture)?
```

## Parameters

- **sharedHandle** — An [MTLSharedTextureHandle](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle) instance, typically from another process using the same GPU device.

## Return Value

A new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance if the method completed successfully; otherwise `nil`.

## Discussion

Call this method from the same [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance that created the shared texture instance.

> **Tip:**
>  You can identify the correct device with the texture handle’s [device](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/device) property.

## See also

### Creating textures
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) — Creates a new texture instance.
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)) — Creates a texture that you can share across process boundaries.
- [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) — Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.
- [minimumTextureBufferAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)) — Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.
