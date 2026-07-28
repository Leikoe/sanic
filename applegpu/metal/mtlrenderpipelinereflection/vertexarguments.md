# vertexArguments

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexarguments>

An array of argument instances, each of which represent a parameter of the pipeline state’s vertex shader.

## Declaration

```swift
var vertexArguments: [MTLArgument]? { get }
```

## Discussion

The [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) elements in the array are in the same order as the vertex shader’s declaration signature.

## See also

### Deprecated
- [fragmentArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s fragment shader.
- [tileArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilearguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s tile shader.
