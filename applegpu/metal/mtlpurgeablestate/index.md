# MTLPurgeableState

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpurgeablestate>

The purgeable state of the resource.

## Declaration

```swift
enum MTLPurgeableState
```

## Topics

### Specifying purgeable states
- [MTLPurgeableState.keepCurrent](https://developer.apple.com/documentation/metal/mtlpurgeablestate/keepcurrent) — The current state is queried but doesn’t change.
- [MTLPurgeableState.nonVolatile](https://developer.apple.com/documentation/metal/mtlpurgeablestate/nonvolatile) — The contents of the resource aren’t allowed to be discarded.
- [MTLPurgeableState.volatile](https://developer.apple.com/documentation/metal/mtlpurgeablestate/volatile) — The system is allowed to discard the resource to free up memory.
- [MTLPurgeableState.empty](https://developer.apple.com/documentation/metal/mtlpurgeablestate/empty) — A state that indicates to the system that it needs to consider the contents of a resource as invalid, typically because you’re discarding it.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlpurgeablestate/init(rawvalue:))

## See also

### Setting the purgeable state of the resource
- [setPurgeableState(_:)](https://developer.apple.com/documentation/metal/mtlresource/setpurgeablestate(_:)) — Specifies or queries the resource’s purgeable state.
