# fragmentBindings

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentbindings>

An array of binding instances, each of which represents a parameter of the pipeline state’s fragment shader.

## Declaration

```swift
var fragmentBindings: [any MTLBinding] { get }
```

## Discussion

The [MTLBinding](https://developer.apple.com/documentation/metal/mtlbinding) elements in the array are in the same order as the fragment shader’s declaration signature.

## See also

### Inspecting a shader’s parameter
- [meshBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/meshbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s mesh shader.
- [objectBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/objectbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s object shader.
- [tileBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilebindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s tile shader.
- [vertexBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s vertex shader.
