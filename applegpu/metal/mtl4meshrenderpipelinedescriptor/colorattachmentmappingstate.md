# colorAttachmentMappingState

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/colorattachmentmappingstate>

Sets the logical-to-physical rendering remap state.

## Declaration

```swift
var colorAttachmentMappingState: MTL4LogicalToPhysicalColorAttachmentMappingState { get set }
```

## Discussion

Use this property to assign how a [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) instance maps the output of your fragment shader to physical color attachments.
