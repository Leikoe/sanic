# shaderAtomic

*Type Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic>

An option that enables atomic memory operations on texture elements in shader code.

## Declaration

```swift
static var shaderAtomic: MTLTextureUsage { get }
```

## Discussion

Shaders can run atomic memory operations on textures with specific element type and pixel format combinations:

| Shader element type | Pixel format |
|---|---|
| `int` | [MTLPixelFormat.r32Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r32sint) |
| `uint` | [MTLPixelFormat.r32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r32uint) |
| `ulong` | [MTLPixelFormat.rg32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg32uint) |

> **Note:**
>  Applying this usage option to a texture disables lossless compression.

## See also

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.
