# MTLIntersectionFunctionTable

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable>

A table of intersection functions that Metal calls to perform ray-tracing intersection tests.

## Declaration

```swift
protocol MTLIntersectionFunctionTable : MTLResource
```

## Overview

Don’t implement this protocol yourself. Instead create an [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) instance and configure its properties. Then call the appropriate method on the pipeline state that you want to use this table with:

- **Compute pipeline** — [makeIntersectionFunctionTable(descriptor:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makeintersectionfunctiontable(descriptor:))

- **Render pipeline** — [makeIntersectionFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makeintersectionfunctiontable(descriptor:stage:))

If you use the same ray-tracing functions with more than one pipeline, make a separate table for each.

Use the methods on this instance to set the table entries to point at the intersection functions, and to provide buffers as arguments for those functions. For more information about intersection functions, see [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).

## Topics

### Setting a table entry
- [setFunction(_:index:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setfunction(_:index:)) — Sets an entry in the table.
- [setFunctions(_:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setfunctions(_:range:)) — Sets a range of entries in the table.

### Specifying arguments for intersection functions
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setbuffer(_:offset:index:)) — Sets a buffer for the intersection functions.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setbuffers(_:offsets:range:)) — Sets a range of buffers for the intersection functions.
- [setVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setvisiblefunctiontable(_:bufferindex:)) — Sets a visible function table for the intersection functions.
- [setVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setvisiblefunctiontables(_:bufferrange:)) — Sets a range of visible function tables for the intersection functions.

### Specifying opaque triangle intersection testing
- [setOpaqueTriangleIntersectionFunction(signature:index:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquetriangleintersectionfunction(signature:index:)) — Sets an entry in the intersection table to point to a system-defined opaque triangle intersection function.
- [setOpaqueTriangleIntersectionFunction(signature:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquetriangleintersectionfunction(signature:range:)) — Sets a range of entries in the intersection table to point to a system-defined opaque triangle intersection function.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/gpuresourceid)

### Instance Methods
- [setOpaqueCurveIntersectionFunction(signature:index:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquecurveintersectionfunction(signature:index:))
- [setOpaqueCurveIntersectionFunction(signature:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquecurveintersectionfunction(signature:range:))

## See also

### Intersection function tables
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionSignature](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature) — Constants for specifying different types of custom intersection functions.
- [MTLIntersectionFunctionBufferArguments](https://developer.apple.com/documentation/metal/mtlintersectionfunctionbufferarguments)
