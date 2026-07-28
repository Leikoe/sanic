# renderTargetHeight

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor/rendertargetheight>

Sets the height, in pixels, to which Metal constrains the render target.

## Declaration

```swift
var renderTargetHeight: Int { get set }
```

## Discussion

When this value is non-zero, you need to assign it to be smaller than or equal to the minimum height of all attachments.

The default value of this property is `0`.
