# primitiveDataStride

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatastride>

Defines the stride, in bytes, between each primitive’s data in the primitive data buffer [primitiveDataBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatabuffer) references.

## Declaration

```swift
var primitiveDataStride: Int { get set }
```

## Discussion

You are responsible for ensuring the stride is at least [primitiveDataElementSize](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedataelementsize) in size and a multiple of 4 bytes.

This property defaults to `0` bytes,  which indicates the stride is equal to [primitiveDataElementSize](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedataelementsize).
