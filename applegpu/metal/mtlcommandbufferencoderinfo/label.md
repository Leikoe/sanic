# label

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/label>

The name of the encoder that generates the error information.

## Declaration

```swift
var label: String { get }
```

## Discussion

Metal assigns the value of the property to the encoder’s [label](https://developer.apple.com/documentation/metal/mtlcommandencoder/label) property.

## See also

### Inspecting execution information
- [debugSignposts](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/debugsignposts) — An array of debug signposts that Metal records as the GPU executes the commands of the encoder’s pass.
- [errorState](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/errorstate) — The execution status of the command encoder.
- [MTLCommandEncoderErrorState](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate) — Possible error conditions for the command encoder’s commands.
