# MTLPatchType.quad

*Case · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpatchtype/quad>

A quad patch.

## Declaration

```swift
case quad
```

## Discussion

Metal uses this value if the shader is a post-tessellation vertex function with the `[[patch(quad)]]` attribute.

## See also

### Patch types
- [MTLPatchType.none](https://developer.apple.com/documentation/metal/mtlpatchtype/none) — An option that indicates that this isn’t a post-tessellation vertex function.
- [MTLPatchType.triangle](https://developer.apple.com/documentation/metal/mtlpatchtype/triangle) — A triangle patch.
