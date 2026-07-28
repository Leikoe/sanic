# renderTarget

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget>

An option for rendering to the texture in a render pass.

## Declaration

```swift
static var renderTarget: MTLTextureUsage { get }
```

## Discussion

Set this option if you use the given texture as a color, depth, or stencil render target in any render pass. This option allows you to assign the texture to the [texture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/texture) property of an [MTLRenderPassAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor).

> **Important:**
>  Rendering and writing to a texture are different operations, and you don’t need to combine their usage options. Set the [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) option if you render to a given texture, but don’t set the [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) option if you don’t write to the texture. The [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) and [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) options aren’t equivalent, and setting [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) doesn’t require you to also set [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite).

## See also

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.
