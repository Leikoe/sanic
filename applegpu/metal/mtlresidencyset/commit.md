# commit()

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencyset/commit()>

Applies any pending additions to and removals from the residency set.

## Declaration

```swift
func commit()
```

## Discussion

Call the method when have no other changes to stage, such as with [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)), [removeAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocation(_:)), and their sibling methods.
