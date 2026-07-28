# makeTexture(descriptor:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)>

Creates a new texture instance.

## Declaration

```swift
func makeTexture(descriptor: MTLTextureDescriptor) -> (any MTLTexture)?
```

## Parameters

- **descriptor** — An [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance.

## Return Value

A new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance if the method completed successfully; otherwise `nil`.

## See also

### Creating textures
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)) — Creates a texture that you can share across process boundaries.
- [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)) — Creates a texture that references a shared texture.
- [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) — Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.
- [minimumTextureBufferAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)) — Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.
