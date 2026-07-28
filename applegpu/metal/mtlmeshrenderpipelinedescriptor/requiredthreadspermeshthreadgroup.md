# requiredThreadsPerMeshThreadgroup

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/requiredthreadspermeshthreadgroup>

## Declaration

```swift
var requiredThreadsPerMeshThreadgroup: MTLSize { get set }
```

## Discussion

Sets the required mesh threads-per-threadgroup during mesh draws. The `threadsPerMeshThreadgroup` argument of any draw must match to this value if it is set. Setting this to a size of 0 in every dimension disables this property
