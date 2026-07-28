# MTLArgumentBuffersTier

*Enumeration · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentbufferstier>

The values that determine the limits and capabilities of argument buffers.

## Declaration

```swift
enum MTLArgumentBuffersTier
```

## Overview

See [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) for more information about argument buffer tiers, limits, and capabilities. Query the [argumentBuffersSupport](https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport) property to determine argument buffer tier support for a given device.

## Topics

### Enumeration cases
- [MTLArgumentBuffersTier.tier1](https://developer.apple.com/documentation/metal/mtlargumentbufferstier/tier1) — Support for tier 1 argument buffers.
- [MTLArgumentBuffersTier.tier2](https://developer.apple.com/documentation/metal/mtlargumentbufferstier/tier2) — Support for tier 2 argument buffers.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlargumentbufferstier/init(rawvalue:))

## See also

### Enumerations
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLLogStateError](https://developer.apple.com/documentation/metal/mtllogstateerror)
- [MTLMathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlmathfloatingpointfunctions) — Indicates which FP32 math functions Metal uses.
- [MTLMathMode](https://developer.apple.com/documentation/metal/mtlmathmode) — An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
- [MTLMatrixLayout](https://developer.apple.com/documentation/metal/mtlmatrixlayout)
- [MTLReadWriteTextureTier](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier) — The support level for read-write texture formats.
- [MTLShaderValidation](https://developer.apple.com/documentation/metal/mtlshadervalidation) — Indicates whether shader validation in an enabled or disabled state, or neither state.
- [MTLTransformType](https://developer.apple.com/documentation/metal/mtltransformtype)
