# MTLStitchedLibraryDescriptor

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor>

A description of a new library of procedurally generated functions.

## Declaration

```swift
class MTLStitchedLibraryDescriptor
```

## Overview

An [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) describes a library of new stitched functions. A *stitched function* is a visible function you create by composing other Metal shader functions together in a function graph.

Configure a stitched library descriptor by assigning an array of one or more [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) instances, each describing a stitched function, to the [functionGraphs](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functiongraphs) property. Then assign an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) array that includes all the functions the graphs depend on to the [functions](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functions) property.

Create a stitched library from the descriptor by passing it to the [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice). You can change the descriptor to create other libraries without affecting any existing ones.

## Topics

### Configuring a stitched library
- [functions](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functions) — The list of functions for creating the stitched library.
- [functionGraphs](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functiongraphs) — The function graphs that define the new stitched library’s functions.

### Instance Properties
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/binaryarchives)
- [options](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/options)

## See also

### Stitched function libraries
- [Customizing shaders using function pointers and stitching](https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching) — Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingInputNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode) — A call graph node that describes an input to the call graph.
- [MTLFunctionStitchingFunctionNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode) — A call graph node that describes a function call and its inputs.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttributeAlwaysInline](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline) — An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.
