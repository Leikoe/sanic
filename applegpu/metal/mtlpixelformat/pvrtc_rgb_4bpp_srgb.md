# MTLPixelFormat.pvrtc_rgb_4bpp_srgb

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 14.0, macOS 11.0, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_4bpp_srgb>

A compressed format that uses PVRTC compression and 4bpp for RGB components with a conversion between sRGB and linear space.

## Declaration

```swift
case pvrtc_rgb_4bpp_srgb
```

## Discussion

The only texture types that support this format include:

- [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d)

- [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray)

- [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube)

> **Note:**
> The format doesn’t support subimages.

## See also

### Compressed PVRTC pixel formats
- [MTLPixelFormat.pvrtc_rgb_2bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_2bpp) — A compressed format that uses PVRTC compression and 2bpp for RGB components.
- [MTLPixelFormat.pvrtc_rgb_2bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_2bpp_srgb) — A compressed format that uses PVRTC compression and 2bpp for RGB components with a conversion between sRGB and linear space.
- [MTLPixelFormat.pvrtc_rgb_4bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_4bpp) — A compressed format that uses PVRTC compression and 4bpp for RGB components.
- [MTLPixelFormat.pvrtc_rgba_2bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_2bpp) — A compressed format that uses PVRTC compression and 2bpp for RGBA components.
- [MTLPixelFormat.pvrtc_rgba_2bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_2bpp_srgb) — A compressed format that uses PVRTC compression and 2bpp for RGBA components with a conversion between sRGB and linear space.
- [MTLPixelFormat.pvrtc_rgba_4bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_4bpp) — A compressed format that uses PVRTC compression and 4bpp for RGBA components.
- [MTLPixelFormat.pvrtc_rgba_4bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_4bpp_srgb) — A compressed format that uses PVRTC compression and 4bpp for RGBA components with a conversion between sRGB and linear space.
