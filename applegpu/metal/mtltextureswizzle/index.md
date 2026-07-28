# MTLTextureSwizzle

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureswizzle>

A set of options to choose from when creating a texture swizzle pattern.

## Declaration

```swift
enum MTLTextureSwizzle
```

## Topics

### Specifying swizzle channels
- [MTLTextureSwizzle.alpha](https://developer.apple.com/documentation/metal/mtltextureswizzle/alpha) — The alpha channel of the source pixel is copied to the destination channel.
- [MTLTextureSwizzle.blue](https://developer.apple.com/documentation/metal/mtltextureswizzle/blue) — The blue channel of the source pixel is copied to the destination channel.
- [MTLTextureSwizzle.green](https://developer.apple.com/documentation/metal/mtltextureswizzle/green) — The green channel of the source pixel is copied to the destination channel.
- [MTLTextureSwizzle.red](https://developer.apple.com/documentation/metal/mtltextureswizzle/red) — The red channel of the source pixel is copied to the destination channel.
- [MTLTextureSwizzle.one](https://developer.apple.com/documentation/metal/mtltextureswizzle/one) — A value of `1.0` is copied to the destination channel.
- [MTLTextureSwizzle.zero](https://developer.apple.com/documentation/metal/mtltextureswizzle/zero) — A value of `0.0` is copied to the destination channel.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltextureswizzle/init(rawvalue:))

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
- [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexturedescriptor/allowgpuoptimizedcontents) — A Boolean value indicating whether the GPU is allowed to adjust the texture’s contents to improve GPU performance.
- [usage](https://developer.apple.com/documentation/metal/mtltexturedescriptor/usage) — Options that determine how you can use the texture.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexturedescriptor/swizzle) — The pattern you want the GPU to apply to pixels when you read or sample pixels from the texture.
