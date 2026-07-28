# MTLLoadAction

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlloadaction>

Types of actions performed for an attachment at the start of a rendering pass.

## Declaration

```swift
enum MTLLoadAction
```

## Topics

### Load actions
- [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare) — The GPU has permission to discard the existing contents of the attachment at the start of the render pass, replacing them with arbitrary data.
- [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) — The GPU preserves the existing contents of the attachment at the start of the render pass.
- [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear) — The GPU writes a value to every pixel in the attachment at the start of the render pass.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlloadaction/init(rawvalue:))

## See also

### Encoding a render pass in parallel
- [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) — An instance that splits up a single render pass so that it can be simultaneously encoded from multiple threads.
- [MTLStoreAction](https://developer.apple.com/documentation/metal/mtlstoreaction) — Types of actions performed for an attachment at the end of a rendering pass.
- [MTLStoreActionOptions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions) — Options that modify a store action.
