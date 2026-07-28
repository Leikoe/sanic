# allowGPUOptimizedContents

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/allowgpuoptimizedcontents>

A Boolean value indicating whether the GPU is allowed to adjust the texture’s contents to improve GPU performance.

## Declaration

```swift
var allowGPUOptimizedContents: Bool { get set }
```

## Discussion

The default value is `true`, which means that the Metal device is allowed to adjust the private layout of the texture in memory to improve GPU performance. For a shared or managed texture, this optimization can cause slower performance when accessing the texture from the CPU. Setting this property to `false` improves CPU performance at the cost of some GPU performance.

## See also

### Specifying texture attributes
- [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturetype) — The dimension and arrangement of texture image data.
- [pixelFormat](https://developer.apple.com/documentation/metal/mtltexturedescriptor/pixelformat) — The size and bit layout of all pixels in the texture.
- [width](https://developer.apple.com/documentation/metal/mtltexturedescriptor/width) — The width of the texture image for the base level mipmap, in pixels.
- [height](https://developer.apple.com/documentation/metal/mtltexturedescriptor/height) — The height of the texture image for the base level mipmap, in pixels.
- [depth](https://developer.apple.com/documentation/metal/mtltexturedescriptor/depth) — The depth of the texture image for the base level mipmap, in pixels.
- [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) — The number of mipmap levels for this texture.
- [sampleCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/samplecount) — The number of samples in each fragment.
- [arrayLength](https://developer.apple.com/documentation/metal/mtltexturedescriptor/arraylength) — The number of array elements for this texture.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtltexturedescriptor/resourceoptions) — The behavior of a new memory allocation.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/cpucachemode) — The CPU cache mode used for the CPU mapping of the texture.
- [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) — The location and access permissions of the texture.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/hazardtrackingmode) — The texture’s hazard tracking mode.
- [usage](https://developer.apple.com/documentation/metal/mtltexturedescriptor/usage) — Options that determine how you can use the texture.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexturedescriptor/swizzle) — The pattern you want the GPU to apply to pixels when you read or sample pixels from the texture.
- [MTLTextureSwizzleChannels](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels) — A pattern that modifies the data read or sampled from a texture by rearranging or duplicating the elements of a vector.
