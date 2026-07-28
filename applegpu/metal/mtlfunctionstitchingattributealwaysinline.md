# MTLFunctionStitchingAttributeAlwaysInline

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline>

An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.

## Declaration

```swift
class MTLFunctionStitchingAttributeAlwaysInline
```

## Overview

To inline functions in a call graph, instantiate an instance of this class and assign it as an attribute on the [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph).

## See also

### Related Documentation
- [attributes](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph/attributes) — A list of attributes to configure how the Metal device object generates the new stitched function.

### Stitched function libraries
- [Customizing shaders using function pointers and stitching](https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching) — Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.
- [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) — A description of a new library of procedurally generated functions.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingInputNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode) — A call graph node that describes an input to the call graph.
- [MTLFunctionStitchingFunctionNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode) — A call graph node that describes a function call and its inputs.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.
