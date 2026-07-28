# setBarrier()

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setbarrier()>

Adds a barrier to ensure that commands executed prior to this command are complete before this command executes.

## Declaration

```swift
func setBarrier()
```

## Discussion

Set or clear barriers (as needed) before encoding the command.

## See also

### Synchronizing command execution
- [clearBarrier()](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/clearbarrier()) — Removes any barrier set on the command.
