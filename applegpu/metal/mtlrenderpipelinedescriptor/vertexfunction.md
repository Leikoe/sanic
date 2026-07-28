# vertexFunction

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction>

The vertex function the pipeline calls to process vertices.

## Declaration

```swift
var vertexFunction: (any MTLFunction)? { get set }
```

## Discussion

The default value is `nil`. The vertex function needs to be specified. The vertex function can be either a regular vertex function or a post-tessellation vertex function.

## See also

### Specifying graphics functions and associated data
- [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction) — The fragment function the pipeline calls to process fragments.
- [maxVertexCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexcallstackdepth) — The maximum function call depth from the top-most vertex shader function.
- [maxFragmentCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxfragmentcallstackdepth) — The maximum function call depth from the top-most fragment shader function.
