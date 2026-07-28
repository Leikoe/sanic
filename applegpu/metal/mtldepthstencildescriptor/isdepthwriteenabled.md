# isDepthWriteEnabled

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/isdepthwriteenabled>

A Boolean value that indicates whether depth values can be written to the depth attachment.

## Declaration

```swift
var isDepthWriteEnabled: Bool { get set }
```

## Discussion

The default value is [false](https://developer.apple.com/documentation/Swift/false), which indicates the depth attachment is read-only.

## See also

### Specifying depth operations
- [depthCompareFunction](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/depthcomparefunction) — The comparison that is performed between a fragment’s depth value and the depth value in the attachment, which determines whether to discard the fragment.
