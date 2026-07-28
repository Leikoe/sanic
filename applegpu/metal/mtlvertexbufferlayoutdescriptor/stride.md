# stride

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stride>

The number of bytes between the first byte of two consecutive vertices in a buffer.

## Declaration

```swift
var stride: Int { get set }
```

## Discussion

Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Organizing the vertex buffer layout
- [stepFunction](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stepfunction) — The circumstances under which the vertex and its attributes are presented to the vertex function.
- [stepRate](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/steprate) — The interval at which the vertex and its attributes are presented to the vertex function.
- [MTLVertexStepFunction](https://developer.apple.com/documentation/metal/mtlvertexstepfunction) — The frequency with which the vertex function or post-tessellation vertex function fetches attribute data.
