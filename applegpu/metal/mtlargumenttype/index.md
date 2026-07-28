# MTLArgumentType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumenttype>

The resource type for an argument of a function.

## Declaration

```swift
enum MTLArgumentType
```

## Topics

### Argument types
- [MTLArgumentType.buffer](https://developer.apple.com/documentation/metal/mtlargumenttype/buffer) — The argument is a buffer.
- [MTLArgumentType.threadgroupMemory](https://developer.apple.com/documentation/metal/mtlargumenttype/threadgroupmemory) — The argument is a pointer to threadgroup memory.
- [MTLArgumentType.texture](https://developer.apple.com/documentation/metal/mtlargumenttype/texture) — The argument is a texture.
- [MTLArgumentType.sampler](https://developer.apple.com/documentation/metal/mtlargumenttype/sampler) — The argument is a texture sampler.
- [MTLArgumentType.imageblock](https://developer.apple.com/documentation/metal/mtlargumenttype/imageblock) — The argument is an imageblock.
- [MTLArgumentType.imageblockData](https://developer.apple.com/documentation/metal/mtlargumenttype/imageblockdata) — The argument is imageblock data.
- [MTLArgumentType.visibleFunctionTable](https://developer.apple.com/documentation/metal/mtlargumenttype/visiblefunctiontable) — The argument is a visible function table.
- [MTLArgumentType.intersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlargumenttype/intersectionfunctiontable) — The argument is an intersection function table.
- [MTLArgumentType.primitiveAccelerationStructure](https://developer.apple.com/documentation/metal/mtlargumenttype/primitiveaccelerationstructure) — The argument is a bottom-level ray tracing acceleraton structure for a set of primitives.
- [MTLArgumentType.instanceAccelerationStructure](https://developer.apple.com/documentation/metal/mtlargumenttype/instanceaccelerationstructure) — The argument is a top-level ray tracing acceleration structure for a set of instances.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlargumenttype/init(rawvalue:))

## See also

### Function arguments
- [MTLAttribute](https://developer.apple.com/documentation/metal/mtlattribute) — An object that describes an attribute defined in the stage-in argument for a shader.
- [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute) — An instance that represents an attribute of a vertex function.
- [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) — Information about an argument of a graphics or compute function.
- [MTLAutoreleasedArgument](https://developer.apple.com/documentation/metal/mtlautoreleasedargument) — A convenience type alias for an autoreleased argument instance.
- [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess) — Function access restrictions to argument data in the shading language code.
