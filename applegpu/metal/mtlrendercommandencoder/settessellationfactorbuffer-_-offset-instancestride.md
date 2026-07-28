# setTessellationFactorBuffer(_:offset:instanceStride:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorbuffer(_:offset:instancestride:)>

Configures the per-patch tessellation factors for any subsequent patch-drawing commands.

## Declaration

```swift
func setTessellationFactorBuffer(_ buffer: (any MTLBuffer)?, offset: Int, instanceStride: Int)
```

## Parameters

- **buffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that stores the per-patch tessellation factors, which can’t be empty or `nil`.
- **offset** — The distance, in bytes, between the start of the data and the start of the buffer, which needs to be a multiple of `4`.
- **instanceStride** — The number of bytes between two instances of data in `buffer`, which needs to be a multiple of `4`.

## Discussion

Call this method before encoding patch-drawing commands.

## See also

### Configuring tessellation factors
- [setTessellationFactorScale(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorscale(_:)) — Configures the scale factor for per-patch tessellation factors.
