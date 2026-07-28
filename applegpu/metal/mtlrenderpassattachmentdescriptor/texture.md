# texture

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/texture>

The texture object associated with this attachment.

## Declaration

```swift
var texture: (any MTLTexture)? { get set }
```

## Discussion

You need to set the attachment’s `texture` property, choosing an appropriate pixel format for the texture.

- To store color values in an attachment, use a texture with a color-renderable pixel format.

- To store depth values, use a texture with a depth-renderable pixel format, such as [MTLPixelFormat.depth32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/depth32float).

- To store stencil values, use a texture with a stencil-renderable pixel format, such as [MTLPixelFormat.stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/stencil8).

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Specifying the texture for the attachment
- [level](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/level) — The mipmap level of the texture used for rendering to the attachment.
- [slice](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/slice) — The slice of the texture used for rendering to the attachment.
- [depthPlane](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/depthplane) — The depth plane of the texture used for rendering to the attachment.
