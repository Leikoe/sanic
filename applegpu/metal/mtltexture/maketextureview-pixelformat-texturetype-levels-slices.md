# makeTextureView(pixelFormat:textureType:levels:slices:)

*Instance Method · iOS 9.0, iPadOS 9.0, Mac Catalyst 9.0, macOS 10.11, tvOS 9.0, visionOS*

<https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:)>

Creates a new view of the texture, reinterpreting a subset of its data using a different type and pixel format.

## Declaration

```swift
func makeTextureView(pixelFormat: MTLPixelFormat, textureType: MTLTextureType, levels levelRange: Range<Int>, slices sliceRange: Range<Int>) -> (any MTLTexture)?
```

## Parameters

- **pixelFormat** — A new pixel format, which needs to be compatible with the original pixel format.
- **textureType** — A new texture type, which can be cast according to the original texture type as listed the table below.
- **levelRange** — A new base level range that restricts which mipmap levels are visible in the new texture.
- **sliceRange** — A new base slice range that restricts which array slices are visible in the new texture.

## Return Value

A new texture object that shares the same storage allocation of the calling texture object.

## Discussion

The texture type can be cast between the targets listed in the following table.

| Original texture type | New texture type |
|---|---|
| [MTLTextureType.type1D](https://developer.apple.com/documentation/metal/mtltexturetype/type1d) | [MTLTextureType.type1D](https://developer.apple.com/documentation/metal/mtltexturetype/type1d) |
| [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d) | [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d) or [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray) |
| [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray), [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube), or [MTLTextureType.typeCubeArray](https://developer.apple.com/documentation/metal/mtltexturetype/typecubearray) | [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d), [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray), [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube), or [MTLTextureType.typeCubeArray](https://developer.apple.com/documentation/metal/mtltexturetype/typecubearray) |
| [MTLTextureType.type3D](https://developer.apple.com/documentation/metal/mtltexturetype/type3d) | [MTLTextureType.type3D](https://developer.apple.com/documentation/metal/mtltexturetype/type3d) |

The `length` value of the `sliceRange` parameter needs to be `6` if the new texture type value is [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube), or a multiple of `6` if the new texture type value is [MTLTextureType.typeCubeArray](https://developer.apple.com/documentation/metal/mtltexturetype/typecubearray).

For more information on pixel format restrictions, see [makeTextureView(pixelFormat:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:))

## See also

### Related Documentation
- [parentRelativeLevel](https://developer.apple.com/documentation/metal/mtltexture/parentrelativelevel) — The base level of the parent texture used to create this texture.
- [parent](https://developer.apple.com/documentation/metal/mtltexture/parent) — The parent texture used to create this texture, if any.
- [parentRelativeSlice](https://developer.apple.com/documentation/metal/mtltexture/parentrelativeslice) — The base slice of the parent texture used to create this texture.

### Creating textures by reinterpreting existing texture data
- [makeTextureView(pixelFormat:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:)) — Creates a new view of the texture, reinterpreting its data using a different pixel format.
- [makeTextureView(pixelFormat:textureType:levels:slices:swizzle:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:swizzle:)) — Creates a new view of the texture, reinterpreting a subset of its data using a different type, pixel format, and swizzle pattern.
