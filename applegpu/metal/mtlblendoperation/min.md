# MTLBlendOperation.min

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblendoperation/min>

Minimum of the source and destination pixel values.

## Declaration

```swift
case min
```

## Discussion

`RGB = min(Source.rgb, Dest.rgb)`

`A = min(Source.a, Dest.a)`

## See also

### Blend operations
- [MTLBlendOperation.add](https://developer.apple.com/documentation/metal/mtlblendoperation/add) — Add portions of both source and destination pixel values.
- [MTLBlendOperation.subtract](https://developer.apple.com/documentation/metal/mtlblendoperation/subtract) — Subtract a portion of the destination pixel values from a portion of the source.
- [MTLBlendOperation.reverseSubtract](https://developer.apple.com/documentation/metal/mtlblendoperation/reversesubtract) — Subtract a portion of the source values from a portion of the destination pixel values.
- [MTLBlendOperation.max](https://developer.apple.com/documentation/metal/mtlblendoperation/max) — Maximum of the source and destination pixel values.
