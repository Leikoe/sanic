# setAccelerationStructure(_:bufferIndex:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setaccelerationstructure(_:bufferindex:)>

Binds an acceleration structure to the buffer argument table, allowing functions to access it on the GPU.

## Declaration

```swift
func setAccelerationStructure(_ accelerationStructure: (any MTLAccelerationStructure)?, bufferIndex: Int)
```

## Parameters

- **accelerationStructure** — An [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) instance to bind to the argument table.
- **bufferIndex** — The index the structure binds to in the argument table.

## See also

### Binding arguments for acceleration structures
- [setIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontable(_:bufferindex:)) — Binds an intersection function table to the buffer argument table, making it callable in your Metal shaders.
