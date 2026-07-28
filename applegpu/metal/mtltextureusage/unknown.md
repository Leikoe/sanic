# unknown

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/unknown>

An option for a texture whose usage is unknown.

## Declaration

```swift
static var unknown: MTLTextureUsage { get }
```

## Discussion

Set this option if you’re not sure how your app uses the given texture, but you want to be able to use it in many ways. This might be the case if you have multiple code paths and it’s unclear how your app specifically uses the texture at runtime.

This is the most flexible usage option for a texture, but it incurs a significant performance cost. Metal can’t optimize operations for the texture if you don’t set specific usage options.

In iOS devices with GPU family 5, Metal doesn’t apply lossless compression to the given texture if you set this option.

## See also

### Specifying texture usage options
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.
