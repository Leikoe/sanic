# MTLIntersectionFunctionSignature

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature>

Constants for specifying different types of custom intersection functions.

## Declaration

```swift
struct MTLIntersectionFunctionSignature
```

## Overview

For more information on declaring intersection functions in MSL, see [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).

## Topics

### Initializing the intersection function signature
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/init(rawvalue:)) — Returns a new signature description from a specified raw value.

### Specifying the intersection function signature
- [instancing](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/instancing) — A flag indicating that function signature uses instancing.
- [triangleData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/triangledata) — A flag indicating that function signature uses triangle data.
- [worldSpaceData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/worldspacedata) — A flag indicating that function signature uses world space data.

### Type Properties
- [curveData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/curvedata)
- [extendedLimits](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/extendedlimits)
- [instanceMotion](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/instancemotion)
- [intersectionFunctionBuffer](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/intersectionfunctionbuffer)
- [maxLevels](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/maxlevels)
- [primitiveMotion](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/primitivemotion)
- [userData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/userdata)

## See also

### Intersection function tables
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionBufferArguments](https://developer.apple.com/documentation/metal/mtlintersectionfunctionbufferarguments)
