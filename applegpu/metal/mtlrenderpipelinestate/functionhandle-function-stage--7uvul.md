# functionHandle(function:stage:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(function:stage:)-7uvul>

Creates a function handle for a shader.

## Declaration

```swift
func functionHandle(function: any MTLFunction, stage: MTLRenderStages) -> (any MTLFunctionHandle)?
```

## Parameters

- **function** — An [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance that represents the shader the method creates a handle for.
- **stage** — An [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) instance that represents the rendering stage that invokes the shader that `function` represents.

## See also

### Creating function handles and tables
- [makeVisibleFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makevisiblefunctiontable(descriptor:stage:)) — Creates a new visible function table.
- [makeIntersectionFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makeintersectionfunctiontable(descriptor:stage:)) — Creates a new intersection function table.
