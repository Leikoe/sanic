# requiredThreadsPerThreadgroup

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlcompileoptions/requiredthreadsperthreadgroup>

## Declaration

```swift
var requiredThreadsPerThreadgroup: MTLSize { get set }
```

## Discussion

Sets the required threads-per-threadgroup during dispatches. The `threadsPerThreadgroup` argument of any dispatch must match this value if it is set. Setting this to a size of 0 in every dimension disables this property
