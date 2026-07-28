# MTL4LogicalToPhysicalColorAttachmentMappingState.inherited

*Case · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4logicaltophysicalcolorattachmentmappingstate/inherited>

Deduces the color attachment mapping by inheriting it from the color attachment map of the current encoder.

## Declaration

```swift
case inherited
```

## Discussion

Use this setting to indicate Metal should inherit the mapping from the `colorAttachmentMap` property of the current [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) or [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) in use at draw time.
