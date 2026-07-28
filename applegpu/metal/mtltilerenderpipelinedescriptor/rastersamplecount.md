# rasterSampleCount

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/rastersamplecount>

The number of samples in each fragment.

## Declaration

```swift
var rasterSampleCount: Int { get set }
```

## Discussion

The default value is `1`. This value is used only if the pipeline render targets support multisampling. If the render targets don’t support multisampling, then this value needs to be `1`.

When you create a  [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder), the [sampleCount](https://developer.apple.com/documentation/metal/mtltexture/samplecount) value of all attachments need to match this `sampleCount` value. Furthermore, the texture type of all attachments need to be [MTLTextureType.type2DMultisample](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisample).

Support for different sample count values varies by device instance. Call the [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method on an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance to determine whether it supports a specific sample count.

## See also

### Specifying rasterization and visibility state
- [threadgroupSizeMatchesTileSize](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/threadgroupsizematchestilesize) — A Boolean value that indicates whether all threadgroups for this pipeline completely cover tiles.
