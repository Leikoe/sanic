# fragmentFunction

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction>

The fragment function the pipeline calls to process fragments.

## Declaration

```swift
var fragmentFunction: (any MTLFunction)? { get set }
```

## Discussion

The default value is `nil`. If this value is `nil`, then there is no fragment function and therefore no writes to the color render target occur. Depth and stencil writes and visibility result counting can still proceed.

## See also

### Specifying graphics functions and associated data
- [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) — The vertex function the pipeline calls to process vertices.
- [maxVertexCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexcallstackdepth) — The maximum function call depth from the top-most vertex shader function.
- [maxFragmentCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxfragmentcallstackdepth) — The maximum function call depth from the top-most fragment shader function.
