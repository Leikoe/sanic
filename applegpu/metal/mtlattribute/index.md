# MTLAttribute

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlattribute>

An object that describes an attribute defined in the stage-in argument for a shader.

## Declaration

```swift
class MTLAttribute
```

## Topics

### Reading an attribute’s properties
- [name](https://developer.apple.com/documentation/metal/mtlattribute/name) — The name of the attribute.
- [attributeIndex](https://developer.apple.com/documentation/metal/mtlattribute/attributeindex) — The index of the attribute, as declared in Metal shader source code.
- [attributeType](https://developer.apple.com/documentation/metal/mtlattribute/attributetype) — The data type for the attribute, as declared in Metal shader source code.
- [isActive](https://developer.apple.com/documentation/metal/mtlattribute/isactive) — A Boolean value that indicates whether the attribute is active.
- [isPatchControlPointData](https://developer.apple.com/documentation/metal/mtlattribute/ispatchcontrolpointdata) — A Boolean value that indicates whether the attribute represents control point data.
- [isPatchData](https://developer.apple.com/documentation/metal/mtlattribute/ispatchdata) — A Boolean value that indicates whether the attribute represents tessellation patch data.

## See also

### Function arguments
- [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute) — An instance that represents an attribute of a vertex function.
- [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) — Information about an argument of a graphics or compute function.
- [MTLAutoreleasedArgument](https://developer.apple.com/documentation/metal/mtlautoreleasedargument) — A convenience type alias for an autoreleased argument instance.
- [MTLArgumentType](https://developer.apple.com/documentation/metal/mtlargumenttype) — The resource type for an argument of a function.
- [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess) — Function access restrictions to argument data in the shading language code.
