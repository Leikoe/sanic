# setColorAttachmentMap(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcolorattachmentmap(_:)>

Sets the mapping from logical shader color output to physical render pass color attachments.

## Declaration

```swift
func setColorAttachmentMap(_ mapping: MTLLogicalToPhysicalColorAttachmentMap?)
```

## Parameters

- **mapping** — Mapping from logical shader outputs to physical outputs.

## Discussion

Use this method to define how the physical color attachments you specify via [colorAttachments](https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor/colorattachments) map to the logical color output the fragment shader writes to.

To use this feature, make sure to set [supportColorAttachmentMapping](https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor/supportcolorattachmentmapping) to [true](https://developer.apple.com/documentation/Swift/true).

## See also

### Configuring blend behavior
- [setBlendColor(red:green:blue:alpha:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setblendcolor(red:green:blue:alpha:)) — Configures each pixel component value, including alpha, for the render pipeline’s constant blend color.
