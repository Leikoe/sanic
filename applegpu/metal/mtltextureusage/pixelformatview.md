# pixelFormatView

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview>

An option to create texture views with a different component layout.

## Declaration

```swift
static var pixelFormatView: MTLTextureUsage { get }
```

## Discussion

Set this option if you need to call any of these methods of the texture to create a texture view with a different component layout:

- [makeTextureView(pixelFormat:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:))

- [makeTextureView(pixelFormat:textureType:levels:slices:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:))

- [newTextureViewWithPixelFormat:textureType:levels:slices:](https://developer.apple.com/documentation/metal/mtltexture/newtextureviewwithpixelformat:texturetype:levels:slices:)

- [makeTextureView(pixelFormat:textureType:levels:slices:swizzle:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:swizzle:))

- [newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:](https://developer.apple.com/documentation/metal/mtltexture/newtextureviewwithpixelformat:texturetype:levels:slices:swizzle:)

For example, if your texture uses the [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm) pixel format, you can reinterpret the data as [MTLPixelFormat.r32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r32uint). The pixel layout is considered different if the number of components differs, or if their size or order is different from the components in the original pixel format.

Don’t set this option if your texture view needs to read the component values in a different order. Instead, create a texture view with a swizzle pattern that specifies the new order.

Don’t set this option if your texture view only converts between linear space and sRGB. For example, if your texture uses the [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm) pixel format and your texture view uses [MTLPixelFormat.bgra8Unorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra8unorm_srgb).

In iOS devices with GPU family 5 and later, Metal doesn’t apply lossless compression to the given texture if you set this option.

## See also

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
