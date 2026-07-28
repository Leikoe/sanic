# name

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/name>

The name of the function to call.

## Declaration

```swift
var name: String { get set }
```

## Discussion

The name needs to match one of the functions in the stitched library descriptor’s [functions](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functions) property.

## See also

### Configuring a function node
- [arguments](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/arguments) — An ordered list of the nodes that provide the function’s arguments.
- [controlDependencies](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies) — The list of nodes that need to execute before executing the node.
