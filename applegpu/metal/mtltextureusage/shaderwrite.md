# shaderWrite

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite>

An option for writing to the texture in a shader.

## Declaration

```swift
static var shaderWrite: MTLTextureUsage { get }
```

## Discussion

Set this option if you access the given texture with a `write()` function in any shader. This option enables the `access::write` attribute for the texture. For more information about texture functions and access attributes, see [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364).

If the texture is a read-write texture that you also access with a `read()` function in the same shader, set the [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) option to enable the `access::read_write` attribute.

In iOS devices with GPU family 5, Metal doesn’t apply lossless compression to the given texture if you set this option.

> **Important:**
>  Rendering and writing to a texture are different operations, and you don’t need to combine their usage options. Set the [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) option if you render to a given texture, but don’t set the [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) option if you don’t write to the texture. The [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) and [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) options aren’t equivalent, and setting [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) doesn’t require you to also set [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite).

## See also

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.
