# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource/device>

The device object that created the resource.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

A resource can only be used with the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created it.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Identifying the resource
- [label](https://developer.apple.com/documentation/metal/mtlresource/label) — A string that identifies the resource.
