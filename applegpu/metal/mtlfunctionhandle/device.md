# device

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionhandle/device>

The device object that created the shader function.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

You can only use the function handle with this [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice).

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Querying handle properties
- [functionType](https://developer.apple.com/documentation/metal/mtlfunctionhandle/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunctionhandle/name) — The function’s name.
