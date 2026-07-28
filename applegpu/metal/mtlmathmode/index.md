# MTLMathMode

*Enumeration · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlmathmode>

An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.

## Declaration

```swift
enum MTLMathMode
```

## Topics

### Modes
- [MTLMathMode.fast](https://developer.apple.com/documentation/metal/mtlmathmode/fast) — An indicator of the mode the compiler uses to make aggressive, potentially lossy assumptions about floating-point math.
- [MTLMathMode.relaxed](https://developer.apple.com/documentation/metal/mtlmathmode/relaxed) — An indicator of the mode the compiler uses to make aggressive, potentially lossy assumptions about floating-point math, while honoring Inf/NaN.
- [MTLMathMode.safe](https://developer.apple.com/documentation/metal/mtlmathmode/safe) — An indicator of the mode the compiler uses to disable unsafe floating-point optimizations by preventing the compiler from making any transformations that could affect the results.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlmathmode/init(rawvalue:))

## See also

### Enumerations
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLArgumentBuffersTier](https://developer.apple.com/documentation/metal/mtlargumentbufferstier) — The values that determine the limits and capabilities of argument buffers.
- [MTLLogStateError](https://developer.apple.com/documentation/metal/mtllogstateerror)
- [MTLMathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlmathfloatingpointfunctions) — Indicates which FP32 math functions Metal uses.
- [MTLMatrixLayout](https://developer.apple.com/documentation/metal/mtlmatrixlayout)
- [MTLReadWriteTextureTier](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier) — The support level for read-write texture formats.
- [MTLShaderValidation](https://developer.apple.com/documentation/metal/mtlshadervalidation) — Indicates whether shader validation in an enabled or disabled state, or neither state.
- [MTLTransformType](https://developer.apple.com/documentation/metal/mtltransformtype)
