# outputNode

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph/outputnode>

The node with the output that’s the output of the new stitched function.

## Declaration

```swift
var outputNode: MTLFunctionStitchingFunctionNode? { get set }
```

## Discussion

The output type of the node needs to match the result type in the stitched function’s declaration.

## See also

### Configuring a function graph
- [functionName](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph/functionname) — The name of the new stitched function.
- [nodes](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph/nodes) — The nodes in the function’s call graph.
- [attributes](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph/attributes) — A list of attributes to configure how the Metal device object generates the new stitched function.
