# maxVertexCallStackDepth

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexcallstackdepth>

The maximum function call depth from the top-most vertex shader function.

## Declaration

```swift
var maxVertexCallStackDepth: Int { get set }
```

## Discussion

The default value is 1.

## See also

### Specifying graphics functions and associated data
- [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) — The vertex function the pipeline calls to process vertices.
- [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction) — The fragment function the pipeline calls to process fragments.
- [maxFragmentCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxfragmentcallstackdepth) — The maximum function call depth from the top-most fragment shader function.
