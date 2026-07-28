# makeIntersectionFunctionTable(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makeintersectionfunctiontable(descriptor:)>

Creates a new intersection function table.

## Declaration

```swift
func makeIntersectionFunctionTable(descriptor: MTLIntersectionFunctionTableDescriptor) -> (any MTLIntersectionFunctionTable)?
```

## Parameters

- **descriptor** — An [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) instance that configures the created table.

## Return Value

A new intersection function table, or `nil` if an error occurred in creation.

## See also

### Creating function tables
- [makeVisibleFunctionTable(descriptor:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makevisiblefunctiontable(descriptor:)) — Creates a new visible function table.
