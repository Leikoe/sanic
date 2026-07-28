# fragmentFunctionDescriptor

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/fragmentfunctiondescriptor>

Assigns the shader function that this pipeline executes for each fragment.

## Declaration

```swift
@NSCopying var fragmentFunctionDescriptor: MTL4FunctionDescriptor? { get set }
```

## Discussion

When you don’t specify a fragment function, you need to disable rasterization by setting property [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/israsterizationenabled) to false.
