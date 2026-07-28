# patchType

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction/patchtype>

The tessellation patch type of a post-tessellation vertex function.

## Declaration

```swift
var patchType: MTLPatchType { get }
```

## Discussion

This value is [MTLPatchType.none](https://developer.apple.com/documentation/metal/mtlpatchtype/none) if the function isn’t a post-tessellation vertex function.

## See also

### Identifying the tessellation patch
- [patchControlPointCount](https://developer.apple.com/documentation/metal/mtlfunction/patchcontrolpointcount) — The number of patch control points in the post-tessellation vertex function.
- [MTLPatchType](https://developer.apple.com/documentation/metal/mtlpatchtype) — Types of tessellation patches that can be inputs of a post-tessellation vertex function.
