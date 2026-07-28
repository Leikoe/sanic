# setIntersectionFunctionTable(_:bufferIndex:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontable(_:bufferindex:)>

Binds an intersection function table to the buffer argument table, making it callable in your Metal shaders.

## Declaration

```swift
func setIntersectionFunctionTable(_ intersectionFunctionTable: (any MTLIntersectionFunctionTable)?, bufferIndex: Int)
```

## Parameters

- **intersectionFunctionTable** — The [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) to bind.
- **bufferIndex** — The index in the buffer argument table the intersection function table binds to.

## See also

### Binding arguments for acceleration structures
- [setAccelerationStructure(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setaccelerationstructure(_:bufferindex:)) — Binds an acceleration structure to the buffer argument table, allowing functions to access it on the GPU.
