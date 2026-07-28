# Generating mipmap data

*Article*

<https://developer.apple.com/documentation/metal/generating-mipmap-data>

Create your mipmaps either when you author content or at runtime.

## Overview

You create mipmaps for texture sampling by applying a filter to the original image. Different filter algorithms vary in processing time and output quality. You need to determine the right tradeoff for your content by considering file size, quality, and runtime performance.

These are the options for creating mipmaps:

**Have the device object generate them for you at runtime.** This approach is the simplest way to create mipmaps for color images. After initializing mipmap `0` with data, create a blit command encoder and encode a command to generate the other mipmaps using the [generateMipmaps(for:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/generatemipmaps(for:)) method.

```swift
if let encoder = commandBuffer.makeBlitCommandEncoder() {
    encoder.generateMipmaps(for: texture)
    encoder.endEncoding()
}
```

```objective-c
id <MTLBlitCommandEncoder> encoder = [commandBuffer blitCommandEncoder];
[encoder generateMipmapsForTexture: myTexture];
[encoder endEncoding];
```

As with any other GPU commands, the GPU creates the mipmaps asynchronously, at some point after the command buffer is committed to a command queue. The filtering that the device object uses to generate the mipmaps is implementation-dependent and may vary from one GPU to another.

**Generate high-quality mipmaps from your source texture.** Many tools are capable of generating high-quality mipmaps from your source texture. In this case, you store all of the mipmaps in your source data and load them at runtime. This approach lets you use higher-quality filters and tools to build your mipmaps, but increases the size of your files and thus the distribution size of your app.

**Use custom filters or Metal Performance Shaders to generate better mipmaps.** You can also create your own tools, using custom filters or Metal Performance Shaders to generate better mipmaps. Depending on exactly what solution you use for your own tools, you might either create your mipmap data at runtime or as an offline process that runs when you create your content.

## See also

### Texture mipmapping
- [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps) — Avoid texture-rendering artifacts and reduce the GPU’s workload by creating smaller versions of a texture.
- [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture) — Decide whether a texture that you’re creating needs mipmaps.
- [Copying data into or out of mipmaps](https://developer.apple.com/documentation/metal/copying-data-into-or-out-of-mipmaps) — Specify which mipmaps that the data transfer affects.
- [Adding mipmap filtering to samplers](https://developer.apple.com/documentation/metal/adding-mipmap-filtering-to-samplers) — Specify how the GPU samples mipmaps in your textures.
- [Restricting access to specific mipmaps](https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps) — Set the range of mipmap levels that a sampler can access.
- [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) — Determine in advance which mipmap levels the GPU requires to sample a texture.
- [Dynamically adjusting texture level of detail](https://developer.apple.com/documentation/metal/dynamically-adjusting-texture-level-of-detail) — Defer generating or loading larger mipmaps until that level of detail is needed.
