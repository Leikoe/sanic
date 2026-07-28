# setInputDimensions(_:bufferIndex:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/setinputdimensions(_:bufferindex:)-34gir>

Sets the dimension of an input tensor at a buffer index.

## Declaration

```swift
func setInputDimensions(_ dimensions: MTLTensorExtents?, bufferIndex: Int)
```

## Parameters

- **dimensions** — The dimensions of the tensor.
- **bufferIndex** — Index of the tensor to modify.

## Discussion

When the compiled model declares the input as unranked (unknown rank), any concrete `dimensions` are accepted. Otherwise `dimensions.rank` must equal the model’s input rank, and each static (non `-1`) dimension must match.
