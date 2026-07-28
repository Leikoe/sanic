# instanceTransformationMatrixLayout

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancetransformationmatrixlayout>

Specifies the layout for the transformation matrices in the instance descriptor buffer and the motion transformation matrix buffer.

## Declaration

```swift
var instanceTransformationMatrixLayout: MTLMatrixLayout { get set }
```

## Discussion

Metal interprets the value of this property as the layout for the buffers that both [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptorbuffer) and [motionTransformBuffer](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformbuffer) reference.

Defaults to `MTLMatrixLayoutColumnMajor`.
