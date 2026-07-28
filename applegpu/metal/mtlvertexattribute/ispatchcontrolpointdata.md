# isPatchControlPointData

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchcontrolpointdata>

A Boolean value that indicates whether this vertex attribute represents control point data.

## Declaration

```swift
var isPatchControlPointData: Bool { get }
```

## Discussion

This value is always [false](https://developer.apple.com/documentation/Swift/false) if the vertex function is not a post-tessellation vertex function.

## See also

### Describing the attribute
- [name](https://developer.apple.com/documentation/metal/mtlvertexattribute/name) — The name of the attribute.
- [attributeIndex](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributeindex) — The index of the attribute, as declared in Metal shader source code.
- [attributeType](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributetype) — The data type for the attribute, as declared in Metal shader source code.
- [isActive](https://developer.apple.com/documentation/metal/mtlvertexattribute/isactive) — A Boolean value that indicates whether this vertex attribute is active.
- [isPatchData](https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchdata) — A Boolean value that indicates whether this vertex attribute represents patch data.
