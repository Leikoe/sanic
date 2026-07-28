# MTLMathMode.fast

*Case · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlmathmode/fast>

An indicator of the mode the compiler uses to make aggressive, potentially lossy assumptions about floating-point math.

## Declaration

```swift
case fast
```

## Discussion

This is the default for Intel and AMD devices.

## See also

### Modes
- [MTLMathMode.relaxed](https://developer.apple.com/documentation/metal/mtlmathmode/relaxed) — An indicator of the mode the compiler uses to make aggressive, potentially lossy assumptions about floating-point math, while honoring Inf/NaN.
- [MTLMathMode.safe](https://developer.apple.com/documentation/metal/mtlmathmode/safe) — An indicator of the mode the compiler uses to disable unsafe floating-point optimizations by preventing the compiler from making any transformations that could affect the results.
