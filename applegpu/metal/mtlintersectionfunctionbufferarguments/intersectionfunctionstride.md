# intersectionFunctionStride

*Instance Property · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctionbufferarguments/intersectionfunctionstride>

## Declaration

```swift
var intersectionFunctionStride: UInt64
```

## Discussion

The stride between intersection function entries in intersectionFunctionBuffer. The stride needs to be either 0 or aligned to 8 bytes. Note that only the first 12 bits of this value are used by Metal.
