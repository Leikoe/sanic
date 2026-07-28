# sampleCount

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/samplecount>

The number of samples in each fragment.

## Declaration

```swift
var sampleCount: Int { get set }
```

## Discussion

The default value is `1`. If [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturetype) is not [MTLTextureType.type2DMultisample](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisample) or [MTLTextureType.type2DMultisampleArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisamplearray), this value needs to be `1`.

Support for different sample count values varies by device. Call the [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method to determine if your desired sample count value is supported.

## See also

### Specifying texture attributes
- [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturetype) — The dimension and arrangement of texture image data.
- [pixelFormat](https://developer.apple.com/documentation/metal/mtltexturedescriptor/pixelformat) — The size and bit layout of all pixels in the texture.
- [width](https://developer.apple.com/documentation/metal/mtltexturedescriptor/width) — The width of the texture image for the base level mipmap, in pixels.
- [height](https://developer.apple.com/documentation/metal/mtltexturedescriptor/height) — The height of the texture image for the base level mipmap, in pixels.
- [depth](https://developer.apple.com/documentation/metal/mtltexturedescriptor/depth) — The depth of the texture image for the base level mipmap, in pixels.
- [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) — The number of mipmap levels for this texture.
- [arrayLength](https://developer.apple.com/documentation/metal/mtltexturedescriptor/arraylength) — The number of array elements for this texture.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtltexturedescriptor/resourceoptions) — The behavior of a new memory allocation.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/cpucachemode) — The CPU cache mode used for the CPU mapping of the texture.
- [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) — The location and access permissions of the texture.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/hazardtrackingmode) — The texture’s hazard tracking mode.
- [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexturedescriptor/allowgpuoptimizedcontents) — A Boolean value indicating whether the GPU is allowed to adjust the texture’s contents to improve GPU performance.
- [usage](https://developer.apple.com/documentation/metal/mtltexturedescriptor/usage) — Options that determine how you can use the texture.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexturedescriptor/swizzle) — The pattern you want the GPU to apply to pixels when you read or sample pixels from the texture.
- [MTLTextureSwizzleChannels](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels) — A pattern that modifies the data read or sampled from a texture by rearranging or duplicating the elements of a vector.
