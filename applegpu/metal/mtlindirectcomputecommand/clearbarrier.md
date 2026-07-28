# clearBarrier()

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/clearbarrier()>

Removes any barrier set on the command.

## Declaration

```swift
func clearBarrier()
```

## Discussion

You need to set or clear barriers (as needed) before executing any of the commands in the indirect command buffer.

## See also

### Synchronizing command execution
- [setBarrier()](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setbarrier()) — Adds a barrier to ensure that commands executed prior to this command are complete before this command executes.
