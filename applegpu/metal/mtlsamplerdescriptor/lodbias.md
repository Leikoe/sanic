# lodBias

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodbias>

Sets the level-of-detail (lod) bias when sampling from a texture.

## Declaration

```swift
var lodBias: Float { get set }
```

## Discussion

The property’s default value is `0.0f`. The precision format is `S4.6`, and the range is `[-16.0, 15.999]`.
