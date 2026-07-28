# maxVertexAmplificationCount

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexamplificationcount>

The maximum vertex amplification count you can set when encoding render commands.

## Declaration

```swift
var maxVertexAmplificationCount: Int { get set }
```

## Discussion

Before setting this property, call the [supportsVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)) method on the device object to determine whether that amplification count is supported.

## See also

### Related Documentation
- [setVertexAmplificationCount(_:viewMappings:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexamplificationcount(_:viewmappings:)) — Configures the number of output vertices the render pipeline produces for each input vertex, optionally with render target and viewport offsets.
- [supportsVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)) — Returns a Boolean value that indicates whether the GPU supports an amplification factor.
