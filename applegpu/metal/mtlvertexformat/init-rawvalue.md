# init(rawValue:)

*Initializer · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexformat/init(rawvalue:)>

Creates a vertex format from a raw integer value.

## Declaration

```swift
init?(rawValue: UInt)
```

## Parameters

- **rawValue** — The underlying integer value that represents a vertex format.

## Discussion

Use the [MTLVertexFormat](https://developer.apple.com/documentation/metal/mtlvertexformat) structure’s type properties, such as [MTLVertexFormat.uchar4Normalized_bgra](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar4normalized_bgra), instead of this initializer.
