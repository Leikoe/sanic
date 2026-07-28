# tileArguments

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilearguments>

An array of argument instances, each of which represent a parameter of the pipeline state’s tile shader.

## Declaration

```swift
var tileArguments: [MTLArgument]? { get }
```

## Discussion

The [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) elements in the array are in the same order as the tile shader’s declaration signature.

## See also

### Deprecated
- [vertexArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s vertex shader.
- [fragmentArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s fragment shader.
