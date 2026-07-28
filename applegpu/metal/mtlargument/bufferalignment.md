# bufferAlignment

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/bufferalignment>

The required byte alignment in memory for the buffer data.

## Declaration

```swift
var bufferAlignment: Int { get }
```

## Discussion

If the argument is not a buffer, querying this property is a fatal error.

## See also

### Describing a buffer argument
- [bufferDataSize](https://developer.apple.com/documentation/metal/mtlargument/bufferdatasize) — The size, in bytes, of the buffer data.
- [bufferDataType](https://developer.apple.com/documentation/metal/mtlargument/bufferdatatype) — The data type of the buffer data.
- [bufferStructType](https://developer.apple.com/documentation/metal/mtlargument/bufferstructtype) — A description of the structure data of a buffer argument.
- [bufferPointerType](https://developer.apple.com/documentation/metal/mtlargument/bufferpointertype) — A description of the pointer to a buffer argument.
