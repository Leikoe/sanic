# fragmentArguments

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentarguments>

An array of argument instances, each of which represent a parameter of the pipeline state’s fragment shader.

## Declaration

```swift
var fragmentArguments: [MTLArgument]? { get }
```

## Discussion

The [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) elements in the array are in the same order as the fragment shader’s declaration signature.

## See also

### Deprecated
- [vertexArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s vertex shader.
- [tileArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilearguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s tile shader.
