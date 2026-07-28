# objectThreadgroupSizeIsMultipleOfThreadExecutionWidth

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/objectthreadgroupsizeismultipleofthreadexecutionwidth>

Provides a guarantee to Metal regarding the number of threadgroup threads for the object stage of a pipeline you create from this descriptor.

## Declaration

```swift
var objectThreadgroupSizeIsMultipleOfThreadExecutionWidth: Bool { get set }
```

## Discussion

If you set this property to [true](https://developer.apple.com/documentation/Swift/true), you state to Metal that when you use a mesh render pipeline you create from this descriptor, the number of threadgroup threads you dispatch for the object stage is a multiple of its [objectThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/objectthreadexecutionwidth). The compiler’s optimizer can use this guarantee to generate more efficient code.

This property’s default value is [false](https://developer.apple.com/documentation/Swift/false).
