# MTL4TimestampGranularity.precise

*Case · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4timestampgranularity/precise>

A timestamp as precise as possible.

## Declaration

```swift
case precise
```

## Discussion

Using this granularity may incur in a performance penalty, for example, it may cause splitting of command encoders.
