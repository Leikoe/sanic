# stepRate

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/steprate>

The interval at which the vertex and its attributes are presented to the vertex function.

## Declaration

```swift
var stepRate: Int { get set }
```

## Discussion

The default value is `1`. The `stepRate` value, in conjunction with the [stepFunction](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stepfunction) property, determines how often the function fetches new attribute data. The `stepRate` property is generally used when `stepFunction` is [MTLVertexStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perinstance). If `stepRate` is equal to `1`, new attribute data is fetched for every instance; if `stepRate` is equal to `2`, new attribute data is fetched for every two instances, and so forth.

## See also

### Organizing the vertex buffer layout
- [stepFunction](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stepfunction) — The circumstances under which the vertex and its attributes are presented to the vertex function.
- [stride](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stride) — The number of bytes between the first byte of two consecutive vertices in a buffer.
- [MTLVertexStepFunction](https://developer.apple.com/documentation/metal/mtlvertexstepfunction) — The frequency with which the vertex function or post-tessellation vertex function fetches attribute data.
