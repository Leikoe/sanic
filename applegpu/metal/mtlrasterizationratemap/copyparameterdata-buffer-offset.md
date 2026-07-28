# copyParameterData(buffer:offset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap/copyparameterdata(buffer:offset:)>

Copies the parameter data into the provided buffer.

## Declaration

```swift
func copyParameterData(buffer: any MTLBuffer, offset: Int)
```

## Parameters

- **buffer** — The buffer instance to copy the data into. It needs to have an [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) storage mode, and there needs to be enough room in the buffer to store the data.
- **offset** — The location in the buffer to copy the data to. The offset needs to be a multiple of the parameter alignment.

## Discussion

To convert coordinate values inside your shader, pass the rate map data into the shader in an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance. The [parameterDataSizeAndAlign](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/parameterdatasizeandalign) property provides the size and alignment requirements for the buffer.

You can convert between screen space and physical fragment space by binding the buffer to the shader with type `rasterization_rate_map_data`, then constructing `rasterization_rate_map_decoder` with the buffer data. For more details, see the “Variable Rasterization Rate” section of the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).

## See also

### Obtaining coordinate transformation data
- [parameterDataSizeAndAlign](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/parameterdatasizeandalign) — The size and alignment requirements to contain the coordinate transformation information in this rate map.
