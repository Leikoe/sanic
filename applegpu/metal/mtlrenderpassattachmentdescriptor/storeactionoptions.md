# storeActionOptions

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeactionoptions>

The options that modify the store action performed by this attachment.

## Declaration

```swift
var storeActionOptions: MTLStoreActionOptions { get set }
```

## Discussion

This property specifies additional behavior for the store action specified by the [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property.

The default value is [MTLStoreActionOptionNone](https://developer.apple.com/documentation/metal/mtlstoreactionoptions/mtlstoreactionoptionnone).

## See also

### Specifying rendering pass actions
- [loadAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction) — The action performed by this attachment at the start of a rendering pass for a render command encoder.
- [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) — The action performed by this attachment at the end of a rendering pass for a render command encoder.
