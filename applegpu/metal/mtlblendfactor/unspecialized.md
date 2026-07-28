# MTLBlendFactor.unspecialized

*Case · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlblendfactor/unspecialized>

Defers assigning the blend factor.

## Declaration

```swift
case unspecialized
```

## Discussion

Until you specialize this value in the pipeline state, it:

- behaves as `MTLBlendFactorOne` for `sourceRGBBlendFactor` and `sourceAlphaBlendFactor`

- behaves as `MTLBlendFactorZero` for `destinationRGBBlendFactor` and `destinationAlphaBlendFactor`
