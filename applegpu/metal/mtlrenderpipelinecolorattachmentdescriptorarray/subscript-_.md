# subscript(_:)

*Instance Subscript · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray/subscript(_:)>

Returns the render pipeline state for the specified color attachment.

## Declaration

```swift
subscript(attachmentIndex: Int) -> MTLRenderPipelineColorAttachmentDescriptor! { get set }
```

## Parameters

- **attachmentIndex** — An index in the color attachment array.

## Return Value

An [MTLRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor) instance that describes the render pipeline information for a color attachment.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)
