# auxiliaryPlanes

*Instance Property · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/auxiliaryplanes>

The auxiliary plane configurations for this tensor.

## Declaration

```swift
var auxiliaryPlanes: MTLTensorAuxiliaryPlaneDescriptorMap? { get set }
```

## Discussion

Set this property with a populated [MTLTensorAuxiliaryPlaneDescriptorMap](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap) to create a multi-plane tensor. When `nil`, the tensor has only a data plane.

Multi-plane tensors do not support [machineLearning](https://developer.apple.com/documentation/metal/mtltensorusage/machinelearning). Use [compute](https://developer.apple.com/documentation/metal/mtltensorusage/compute) or [render](https://developer.apple.com/documentation/metal/mtltensorusage/render).

Multi-plane tensors do not support data types larger than one byte as the data plane type.

Multi-plane tensors do not support rank zero.

The default value is `nil`.
