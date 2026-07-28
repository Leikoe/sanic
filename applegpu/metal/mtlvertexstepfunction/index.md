# MTLVertexStepFunction

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexstepfunction>

The frequency with which the vertex function or post-tessellation vertex function fetches attribute data.

## Declaration

```swift
enum MTLVertexStepFunction
```

## Topics

### Step functions
- [MTLVertexStepFunction.constant](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/constant) — The vertex function fetches attribute data once and uses that data for every vertex.
- [MTLVertexStepFunction.perVertex](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/pervertex) — The vertex function fetches and uses new attribute data for every vertex.
- [MTLVertexStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perinstance) — The vertex function regularly fetches new attribute data for a number of instances that is determined by `stepRate`.
- [MTLVertexStepFunction.perPatch](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perpatch) — The post-tessellation vertex function fetches data based on the patch index of the patch.
- [MTLVertexStepFunction.perPatchControlPoint](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/perpatchcontrolpoint) — The post-tessellation vertex function fetches data based on the control-point indices associated with the patch.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlvertexstepfunction/init(rawvalue:))

## See also

### Organizing the vertex buffer layout
- [stepFunction](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stepfunction) — The circumstances under which the vertex and its attributes are presented to the vertex function.
- [stepRate](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/steprate) — The interval at which the vertex and its attributes are presented to the vertex function.
- [stride](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stride) — The number of bytes between the first byte of two consecutive vertices in a buffer.
