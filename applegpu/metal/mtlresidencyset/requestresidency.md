# requestResidency()

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencyset/requestresidency()>

Tells Metal to do as much preparatory work as it can, with the system’s current conditions, to make the set’s resource allocations resident.

## Declaration

```swift
func requestResidency()
```

## Discussion

Call the method anytime after calling a residency set’s [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method, ideally well before calling the [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method of any [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) that uses it.

The method may postpone some of the necessary steps to make resources resident in scenarios where other apps concurrently need resources in residency.
