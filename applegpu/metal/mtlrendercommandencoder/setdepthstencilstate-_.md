# setDepthStencilState(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstencilstate(_:)>

Configures the combined depth and stencil state.

## Declaration

```swift
func setDepthStencilState(_ depthStencilState: (any MTLDepthStencilState)?)
```

## Parameters

- **depthStencilState** — An instance that conforms to the [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) protocol.

## Discussion

This method changes the combined depth and stencil state for the render command encoder that’s compatible with its depth and stencil attachment configuration. For example, if the new state enables depth testing or depth writing, the render pass needs to have a depth attachment. Similarly, if the new state enables stencil testing or stencil writing, the render pass’s stencil needs to have a stencil attachment. You create depth and stencil attachments for a render pass by assigning the [depthAttachment](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/depthattachment) and [stencilAttachment](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/stencilattachment) properties of the [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) instance that creates it.

Pass `nil` to clear the state from the previous call, which restores a state that’s equivalent to the default values of an [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) instance’s properties.

## See also

### Configuring depth and stencil behavior
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment functions by a scaling factor and bias.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthclipmode(_:)) — Configures how the render pipeline handles fragments outside the near and far planes of the view frustum.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) — Configures the same comparison value for front- and back-facing primitives.
- [setStencilReferenceValues(front:back:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalues(front:back:)) — Configures different comparison values for front- and back-facing primitives.
