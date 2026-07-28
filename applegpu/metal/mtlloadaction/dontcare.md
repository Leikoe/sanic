# MTLLoadAction.dontCare

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlloadaction/dontcare>

The GPU has permission to discard the existing contents of the attachment at the start of the render pass, replacing them with arbitrary data.

## Declaration

```swift
case dontCare
```

## See also

### Load actions
- [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) — The GPU preserves the existing contents of the attachment at the start of the render pass.
- [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear) — The GPU writes a value to every pixel in the attachment at the start of the render pass.
