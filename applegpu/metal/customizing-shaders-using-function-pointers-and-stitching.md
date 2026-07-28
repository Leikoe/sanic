# Customizing shaders using function pointers and stitching

*Sample Code · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, Xcode 26.3*

<https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching>

Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.

## Overview

> **Note:**
> This sample code project is associated with WWDC2021 session [10229: Discover compilation workflows in Metal](https://developer.apple.com/wwdc21/10229/) and WWDC2022 session [6596: Target and optimize GPU binaries with Metal 3](https://developer.apple.com/wwdc22/6596).

## See also

### Stitched function libraries
- [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) — A description of a new library of procedurally generated functions.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingInputNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode) — A call graph node that describes an input to the call graph.
- [MTLFunctionStitchingFunctionNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode) — A call graph node that describes a function call and its inputs.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttributeAlwaysInline](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline) — An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.
