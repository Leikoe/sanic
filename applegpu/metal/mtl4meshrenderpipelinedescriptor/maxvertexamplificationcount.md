# maxVertexAmplificationCount

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/maxvertexamplificationcount>

Determines the maximum value that can you can pass as the pipeline’s amplification count.

## Declaration

```swift
var maxVertexAmplificationCount: Int { get set }
```

## Discussion

This property controls the maximum count you pass to [setVertexAmplificationCount:viewMappings:](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setvertexamplificationcount:viewmappings:) when using vertex amplification with this pipeline.
