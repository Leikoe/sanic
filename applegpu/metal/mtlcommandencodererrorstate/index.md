# MTLCommandEncoderErrorState

*Enumeration · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate>

Possible error conditions for the command encoder’s commands.

## Declaration

```swift
enum MTLCommandEncoderErrorState
```

## Topics

### Getting the error state
- [MTLCommandEncoderErrorState.completed](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/completed) — A state that indicates the GPU successfully executed the commands without any errors.
- [MTLCommandEncoderErrorState.pending](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/pending) — An error state that indicates the GPU didn’t execute the commands.
- [MTLCommandEncoderErrorState.affected](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/affected) — An error state that indicates the GPU failed to fully execute the commands because of an error.
- [MTLCommandEncoderErrorState.faulted](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/faulted) — An error state that indicates the commands in the command buffer are the cause of an error.
- [MTLCommandEncoderErrorState.unknown](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/unknown) — An error state that indicates the command buffer doesn’t know the state of its commands on the GPU.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/init(rawvalue:))

## See also

### Inspecting execution information
- [label](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/label) — The name of the encoder that generates the error information.
- [debugSignposts](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/debugsignposts) — An array of debug signposts that Metal records as the GPU executes the commands of the encoder’s pass.
- [errorState](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/errorstate) — The execution status of the command encoder.
