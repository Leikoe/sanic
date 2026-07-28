# MTLFunctionStitchingInputNode

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode>

A call graph node that describes an input to the call graph.

## Declaration

```swift
class MTLFunctionStitchingInputNode
```

## Overview

An input node contains data from one of the stitched function’s parameters. The output data type of an input node has the same type as the matching parameter.

## Topics

### Initializing an input node
- [init(argumentIndex:)](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode/init(argumentindex:)) — Creates a new input node.

### Configuring an input node
- [argumentIndex](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode/argumentindex) — The index in the command’s buffer argument table that declares which data to read for this input node.

## See also

### Stitched function libraries
- [Customizing shaders using function pointers and stitching](https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching) — Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.
- [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) — A description of a new library of procedurally generated functions.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingFunctionNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode) — A call graph node that describes a function call and its inputs.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttributeAlwaysInline](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline) — An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.
