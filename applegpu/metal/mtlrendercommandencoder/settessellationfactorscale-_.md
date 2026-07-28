# setTessellationFactorScale(_:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorscale(_:)>

Configures the scale factor for per-patch tessellation factors.

## Declaration

```swift
func setTessellationFactorScale(_ scale: Float)
```

## Parameters

- **scale** — A positive, normal floating-point scale factor the render pass applies to the per-patch tessellation factors. The value of `scale` can’t be negative, infinite, equal to `NaN` (not a number), or a denormalized number.

## Discussion

The command converts `scale` to a half-precision floating-point value before it applies it to the per-patch tessellation factors (see [setTessellationFactorBuffer(_:offset:instanceStride:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorbuffer(_:offset:instancestride:))).

## See also

### Configuring tessellation factors
- [setTessellationFactorBuffer(_:offset:instanceStride:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorbuffer(_:offset:instancestride:)) — Configures the per-patch tessellation factors for any subsequent patch-drawing commands.
