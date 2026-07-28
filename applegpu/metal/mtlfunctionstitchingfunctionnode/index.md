# MTLFunctionStitchingFunctionNode

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode>

A call graph node that describes a function call and its inputs.

## Declaration

```swift
class MTLFunctionStitchingFunctionNode
```

## Overview

When the Metal device object evaluates the function graph to compile the stitched function, it evaluates the nodes stored in the [arguments](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/arguments) property that it hasn’t already evaluated, and then calls the function specified by [name](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/name) to generate the node’s output.

If the function has side effects on the input data, use the [controlDependencies](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies) property on other nodes to specify whether the Metal device object needs to evaluate this node first.

## Topics

### Initializing a function node
- [init(name:arguments:controlDependencies:)](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/init(name:arguments:controldependencies:)) — Creates a new function node.

### Configuring a function node
- [name](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/name) — The name of the function to call.
- [arguments](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/arguments) — An ordered list of the nodes that provide the function’s arguments.
- [controlDependencies](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode/controldependencies) — The list of nodes that need to execute before executing the node.

## See also

### Stitched function libraries
- [Customizing shaders using function pointers and stitching](https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching) — Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.
- [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) — A description of a new library of procedurally generated functions.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingInputNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode) — A call graph node that describes an input to the call graph.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttributeAlwaysInline](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline) — An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.
