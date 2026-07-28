# minimumLinearTextureAlignment(for:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)>

Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.

## Declaration

```swift
func minimumLinearTextureAlignment(for format: MTLPixelFormat) -> Int
```

## Parameters

- **format** — An [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) instance that can’t be any of the depth, stencil, or compressed pixel formats.

## Discussion

Metal aligns linear textures to their minimum alignment value, which directly affects the [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) method’s `offset` and `bytesPerRow` parameters.

## See also

### Creating textures
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) — Creates a new texture instance.
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)) — Creates a texture that you can share across process boundaries.
- [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)) — Creates a texture that references a shared texture.
- [minimumTextureBufferAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)) — Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.
