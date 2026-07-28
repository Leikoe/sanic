# arguments

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/arguments>

An ordered list of the nodes that provide the function’s arguments.

## Declaration

```swift
var arguments: [any MTLFunctionStitchingNode] { get set }
```

## Discussion

Each node’s output data types needs to match the input data type of the matching argument.

## See also

### Configuring a function node
- [name](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/name) — The name of the function to call.
- [controlDependencies](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies) — The list of nodes that need to execute before executing the node.
