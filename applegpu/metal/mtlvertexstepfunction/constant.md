# MTLVertexStepFunction.constant

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexstepfunction/constant>

The vertex function fetches attribute data once and uses that data for every vertex.

## Declaration

```swift
case constant
```

## See also

### Step functions
- [MTLVertexStepFunction.perVertex](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/pervertex) — The vertex function fetches and uses new attribute data for every vertex.
- [MTLVertexStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perinstance) — The vertex function regularly fetches new attribute data for a number of instances that is determined by `stepRate`.
- [MTLVertexStepFunction.perPatch](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perpatch) — The post-tessellation vertex function fetches data based on the patch index of the patch.
- [MTLVertexStepFunction.perPatchControlPoint](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perpatchcontrolpoint) — The post-tessellation vertex function fetches data based on the control-point indices associated with the patch.
