# MTLLibraryOptimizationLevel.size

*Case · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/size>

An optimization option for the Metal compiler that prioritizes minimizing the size of its output binaries, which may also reduce compile time.

## Declaration

```swift
case size
```

## Discussion

This option is similar to [MTLLibraryOptimizationLevel.default](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/default), but adds optimizations that prioritize minimizing a shader’s executable size, which may also reduce compile time.

## See also

### Optimization options
- [MTLLibraryOptimizationLevel.default](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/default) — An optimization option for the Metal compiler that prioritizes runtime performance.
