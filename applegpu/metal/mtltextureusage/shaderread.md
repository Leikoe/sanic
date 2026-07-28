# shaderRead

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/shaderread>

An option for reading or sampling from the texture in a shader.

## Declaration

```swift
static var shaderRead: MTLTextureUsage { get }
```

## Discussion

Set this option if you access the given texture with a `read()` or `sample()` function in any shader. This option enables the `access::read` and `access::sample` attributes for the texture. For more information about texture functions and access attributes, see [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364).

If the texture is a read-write texture that you also access with a `write()` function in the same shader, set the [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) option to enable the `access::read_write` attribute.

## See also

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.
