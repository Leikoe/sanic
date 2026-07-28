# init(_:)

*Initializer · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensorextents/init(_:)>

Creates a tensor with extents from an array of dimension values.

## Declaration

```swift
convenience init?(_ values: [Int])
```

## Discussion

You are responsible for ensuring the array contains at most `MTL_TENSOR_MAX_RANK` (`16`) elements.
