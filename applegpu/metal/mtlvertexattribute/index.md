# MTLVertexAttribute

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattribute>

An instance that represents an attribute of a vertex function.

## Declaration

```swift
class MTLVertexAttribute
```

## Overview

An [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute) instance represents an attribute for per-vertex input in a vertex function. You use vertex attribute instances to inspect the inputs of a vertex function by examining the [vertexAttributes](https://developer.apple.com/documentation/metal/mtlfunction/vertexattributes) property of the corresponding [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance.

## Topics

### Describing the attribute
- [name](https://developer.apple.com/documentation/metal/mtlvertexattribute/name) — The name of the attribute.
- [attributeIndex](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributeindex) — The index of the attribute, as declared in Metal shader source code.
- [attributeType](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributetype) — The data type for the attribute, as declared in Metal shader source code.
- [isActive](https://developer.apple.com/documentation/metal/mtlvertexattribute/isactive) — A Boolean value that indicates whether this vertex attribute is active.
- [isPatchControlPointData](https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchcontrolpointdata) — A Boolean value that indicates whether this vertex attribute represents control point data.
- [isPatchData](https://developer.apple.com/documentation/metal/mtlvertexattribute/ispatchdata) — A Boolean value that indicates whether this vertex attribute represents patch data.

## See also

### Function arguments
- [MTLAttribute](https://developer.apple.com/documentation/metal/mtlattribute) — An object that describes an attribute defined in the stage-in argument for a shader.
- [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) — Information about an argument of a graphics or compute function.
- [MTLAutoreleasedArgument](https://developer.apple.com/documentation/metal/mtlautoreleasedargument) — A convenience type alias for an autoreleased argument instance.
- [MTLArgumentType](https://developer.apple.com/documentation/metal/mtlargumenttype) — The resource type for an argument of a function.
- [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess) — Function access restrictions to argument data in the shading language code.
