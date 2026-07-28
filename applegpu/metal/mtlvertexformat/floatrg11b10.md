# MTLVertexFormat.floatRG11B10

*Case · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexformat/floatrg11b10>

A three-component vector with 11-bit floating-point values for red and green, and a 10-bit value for blue.

## Declaration

```swift
case floatRG11B10
```

## Discussion

The 11-bit components for red and green each store five exponent bits and six mantissa bits. The 10-bit blue component stores five exponent bits and five mantissa bits.

## See also

### 32-bit floating-point formats
- [MTLVertexFormat.float](https://developer.apple.com/documentation/metal/mtlvertexformat/float) — A 32-bit floating-point value.
- [MTLVertexFormat.float2](https://developer.apple.com/documentation/metal/mtlvertexformat/float2) — A two-component vector with 32-bit floating-point values.
- [MTLVertexFormat.float3](https://developer.apple.com/documentation/metal/mtlvertexformat/float3) — A three-component vector with 32-bit floating-point values.
- [MTLVertexFormat.float4](https://developer.apple.com/documentation/metal/mtlvertexformat/float4) — A four-component vector with 32-bit floating-point values.
- [MTLVertexFormat.floatRGB9E5](https://developer.apple.com/documentation/metal/mtlvertexformat/floatrgb9e5) — A three-component vector with 9-bit floating-point values for red, green, and blue, and a 5-bit shared exponent.
