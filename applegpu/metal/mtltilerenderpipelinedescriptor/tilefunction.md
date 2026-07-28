# tileFunction

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilefunction>

The compute kernel or fragment function the pipeline calls.

## Declaration

```swift
var tileFunction: any MTLFunction { get set }
```

## Discussion

Kernel-based and fragment-based tile pipeline dispatches act as a barrier against previous draw commands and other dispatches. Kernel-based pipelines wait until all prior access to the tile completes. Fragment-based pipelines wait only until all prior access to the fragment’s location completes.

## See also

### Specifying graphics functions and associated data
- [tileBuffers](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilebuffers) — An array that contains the buffer mutability options for a render pipeline’s tile function.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in tile shaders.
