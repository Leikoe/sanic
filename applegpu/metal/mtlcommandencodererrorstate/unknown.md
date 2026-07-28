# MTLCommandEncoderErrorState.unknown

*Case · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/unknown>

An error state that indicates the command buffer doesn’t know the state of its commands on the GPU.

## Declaration

```swift
case unknown
```

## See also

### Getting the error state
- [MTLCommandEncoderErrorState.completed](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/completed) — A state that indicates the GPU successfully executed the commands without any errors.
- [MTLCommandEncoderErrorState.pending](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/pending) — An error state that indicates the GPU didn’t execute the commands.
- [MTLCommandEncoderErrorState.affected](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/affected) — An error state that indicates the GPU failed to fully execute the commands because of an error.
- [MTLCommandEncoderErrorState.faulted](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate/faulted) — An error state that indicates the commands in the command buffer are the cause of an error.
