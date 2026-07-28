# isRasterizationEnabled

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/israsterizationenabled>

Determines whether the pipeline rasterizes primitives.

## Declaration

```swift
var isRasterizationEnabled: Bool { get set }
```

## Discussion

By default, this value is [true](https://developer.apple.com/documentation/Swift/true), specifying that this pipeline rasterizes primitives. Set this property to [false](https://developer.apple.com/documentation/Swift/false) when you don’t provide a fragment shader function via function [fragmentFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/fragmentfunctiondescriptor).
