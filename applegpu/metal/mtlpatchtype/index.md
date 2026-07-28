# MTLPatchType

*Enumeration · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpatchtype>

Types of tessellation patches that can be inputs of a post-tessellation vertex function.

## Declaration

```swift
enum MTLPatchType
```

## Topics

### Patch types
- [MTLPatchType.none](https://developer.apple.com/documentation/metal/mtlpatchtype/none) — An option that indicates that this isn’t a post-tessellation vertex function.
- [MTLPatchType.triangle](https://developer.apple.com/documentation/metal/mtlpatchtype/triangle) — A triangle patch.
- [MTLPatchType.quad](https://developer.apple.com/documentation/metal/mtlpatchtype/quad) — A quad patch.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlpatchtype/init(rawvalue:))

## See also

### Identifying the tessellation patch
- [patchType](https://developer.apple.com/documentation/metal/mtlfunction/patchtype) — The tessellation patch type of a post-tessellation vertex function.
- [patchControlPointCount](https://developer.apple.com/documentation/metal/mtlfunction/patchcontrolpointcount) — The number of patch control points in the post-tessellation vertex function.
