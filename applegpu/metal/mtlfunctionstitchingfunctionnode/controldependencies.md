# controlDependencies

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies>

The list of nodes that need to execute before executing the node.

## Declaration

```swift
var controlDependencies: [MTLFunctionStitchingFunctionNode] { get set }
```

## Discussion

When a stitched function calls functions that have side effects on their input data, you often need the GPU to execute functions in a specific order. In such cases, use the [controlDependencies](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies) property to specify which nodes need to run before executing this node.

## See also

### Configuring a function node
- [name](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/name) — The name of the function to call.
- [arguments](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/arguments) — An ordered list of the nodes that provide the function’s arguments.
