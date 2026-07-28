# isActive

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattribute/isactive>

A Boolean value that indicates whether this vertex attribute is active.

## Declaration

```swift
var isActive: Bool { get }
```

## Discussion

If [false](https://developer.apple.com/documentation/Swift/false), this attribute is inactive and can be ignored.

## See also

### Describing the attribute
- [name](https://developer.apple.com/documentation/metal/mtlvertexattribute/name) — The name of the attribute.
- [attributeIndex](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributeindex) — The index of the attribute, as declared in Metal shader source code.
- [attributeType](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributetype) — The data type for the attribute, as declared in Metal shader source code.
- [isPatchControlPointData](https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchcontrolpointdata) — A Boolean value that indicates whether this vertex attribute represents control point data.
- [isPatchData](https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchdata) — A Boolean value that indicates whether this vertex attribute represents patch data.
