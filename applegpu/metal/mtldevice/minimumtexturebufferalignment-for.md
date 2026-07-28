# minimumTextureBufferAlignment(for:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)>

Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.

## Declaration

```swift
func minimumTextureBufferAlignment(for format: MTLPixelFormat) -> Int
```

## Parameters

- **format** — An [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) instance.

## Discussion

Metal aligns textures to their minimum alignment value, which directly affects the [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) method’s `offset` and `bytesPerRow` parameters.

## See also

### Creating textures
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) — Creates a new texture instance.
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)) — Creates a texture that you can share across process boundaries.
- [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)) — Creates a texture that references a shared texture.
- [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) — Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.
