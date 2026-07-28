# makeSharedTexture(descriptor:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)>

Creates a texture that you can share across process boundaries.

## Declaration

```swift
func makeSharedTexture(descriptor: MTLTextureDescriptor) -> (any MTLTexture)?
```

## Parameters

- **descriptor** — An [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance.

## Return Value

A new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance if the method completed successfully; otherwise `nil`.

## Discussion

You can create a shared texture but only with [storageModePrivate](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeprivate). You can share the texture with another process by:

1. Creating a texture handle (see [makeSharedTextureHandle()](https://developer.apple.com/documentation/metal/mtltexture/makesharedtexturehandle()))

2. Passing the texture handle to the other process

3. Creating a texture in the other process by calling the [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:))method

> **Important:**
>  You can share a texture with another process that uses the same GPU, but not with a different GPU.

## See also

### Creating textures
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) — Creates a new texture instance.
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)) — Creates a texture that references a shared texture.
- [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) — Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.
- [minimumTextureBufferAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)) — Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.
