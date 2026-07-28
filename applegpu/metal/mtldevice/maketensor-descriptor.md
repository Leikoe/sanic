# makeTensor(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/maketensor(descriptor:)>

Creates a tensor with the specified descriptor.

## Declaration

```swift
func makeTensor(descriptor: MTLTensorDescriptor) throws -> any MTLTensor
```

## Parameters

- **descriptor** — The tensor descriptor configuring the data plane and auxiliary planes.

## Return Value

A tensor, or `nil` if validation fails.

## Discussion

This method validates the constraints documented on [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor).
