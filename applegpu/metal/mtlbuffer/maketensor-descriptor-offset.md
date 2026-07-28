# makeTensor(descriptor:offset:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlbuffer/maketensor(descriptor:offset:)>

Creates a single-plane tensor with the specified descriptor that shares storage with this buffer.

## Declaration

```swift
func makeTensor(descriptor: MTLTensorDescriptor, offset: Int) throws -> any MTLTensor
```

## Parameters

- **descriptor** — The tensor descriptor configuring the data plane.
- **offset** — The byte offset into the buffer where tensor data begins.

## Return Value

A tensor, or `nil` if validation fails.

## Discussion

This method validates the constraints documented on [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor), and additionally requires:

- `offset` is 0 when [usage](https://developer.apple.com/documentation/metal/mtltensordescriptor/usage) contains [machineLearning](https://developer.apple.com/documentation/metal/mtltensorusage/machinelearning).

- `offset` is aligned to 128 bytes if the data plane uses a format [MTLTensorDataType](https://developer.apple.com/documentation/metal/mtltensordatatype).

- `offset` is aligned to the size of the data type in bytes otherwise.

This method doesn’t create tensors that contain auxiliary planes. Use [makeTensor(descriptor:attachments:)](https://developer.apple.com/documentation/metal/mtldevice/maketensor(descriptor:attachments:)) instead to create a multi-plane tensor with per-plane buffer backing storage.
