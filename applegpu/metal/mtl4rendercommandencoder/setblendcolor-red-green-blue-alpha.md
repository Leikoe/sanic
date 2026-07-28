# setBlendColor(red:green:blue:alpha:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setblendcolor(red:green:blue:alpha:)>

Configures each pixel component value, including alpha, for the render pipeline’s constant blend color.

## Declaration

```swift
func setBlendColor(red: Float, green: Float, blue: Float, alpha: Float)
```

## Parameters

- **red** — A value for the red component for the blend color constant.
- **green** — A value for the green component for the blend color constant.
- **blue** — A value for the blue component for the blend color constant.
- **alpha** — A value for the alpha component for the blend color constant.

## See also

### Configuring blend behavior
- [setColorAttachmentMap(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcolorattachmentmap(_:)) — Sets the mapping from logical shader color output to physical render pass color attachments.
