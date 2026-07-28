# stencilAttachmentPixelFormat

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/stencilattachmentpixelformat>

The pixel format of the attachment that stores stencil data.

## Declaration

```swift
var stencilAttachmentPixelFormat: MTLPixelFormat { get set }
```

## Discussion

By default, the pixel format of the rendering pipeline state for each attachment is `MTLPixelFormatInvalid`.

## See also

### Specifying rendering pipeline state
- [reset()](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/reset()) — Specifies the default rendering pipeline state values for the descriptor.
- [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/colorattachments) — An array of attachments that store color data.
- [depthAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/depthattachmentpixelformat) — The pixel format of the attachment that stores depth data.
