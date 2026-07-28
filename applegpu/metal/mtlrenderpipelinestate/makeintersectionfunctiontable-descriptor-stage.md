# makeIntersectionFunctionTable(descriptor:stage:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makeintersectionfunctiontable(descriptor:stage:)>

Creates a new intersection function table.

## Declaration

```swift
func makeIntersectionFunctionTable(descriptor: MTLIntersectionFunctionTableDescriptor, stage: MTLRenderStages) -> (any MTLIntersectionFunctionTable)?
```

## Parameters

- **descriptor** — An [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) instance that configures the visible function table the method creates.
- **stage** — An [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) instance that represents the render pass stage the intersection function table applies to.

## See also

### Creating function handles and tables
- [functionHandle(function:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(function:stage:)-7uvul) — Creates a function handle for a shader.
- [makeVisibleFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makevisiblefunctiontable(descriptor:stage:)) — Creates a new visible function table.
