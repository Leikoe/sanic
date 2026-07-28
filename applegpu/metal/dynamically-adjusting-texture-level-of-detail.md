# Dynamically adjusting texture level of detail

*Article*

<https://developer.apple.com/documentation/metal/dynamically-adjusting-texture-level-of-detail>

Defer generating or loading larger mipmaps until that level of detail is needed.

## Overview

While the most common use for mipmaps is to improve sampling quality and performance, they have other uses, such as texture streaming. If you are loading texture data over a network or generating textures procedurally at runtime, it can be expensive to create a full set of mipmaps, especially if mipmap 0 is large. Further, if the objects being rendered are far away from the camera, the detail in mipmap 0 may never be needed. But because you aren’t required to provide mipmaps when you create the texture, you can bring in additional levels of detail only when needed.

### Provide data for smaller mipmaps

Start by creating the textures, as described in [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture). Remember that Metal allocates memory for all of the mipmaps when you create the texture. Instead of loading data for all mipmaps, pick a lower mipmap, and provide data for it and any mipmaps lower in the chain. For example, if you started at mipmap level 3, as shown in the figure below, you are loading only about 2% of the total texture data required for the entire mipmap chain.

![image](https://docs-assets.developer.apple.com/published/dafb29f75ae2b8c6a374ac3dcac03377/dynamically-adjusting-texture-level-of-detail-1%402x.png)

### Limit access to higher mipmaps

You need to keep track of the highest mipmap you’ve loaded and pass this information to your shader so that it samples only from mipmaps that contain data. You can do this by passing in an appropriately configured sampler, or, on some GPUs, by passing in the minimum level of detail (LOD) to your shader and using it as the minimum LOD when you sample the texture. See [Control mipmap selection when you sample the texture](https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps#Control-mipmap-selection-when-you-sample-the-texture).

### Determine when objects get closer to the camera

As the scene animates, some objects may get closer to the camera. Detect when this happens by asking the shader which mipmap it needs to access or by performing a calculation based on the rendered image size, as described in [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) and [Using function specialization to build pipeline variants](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants).

### Update the mipmaps

When it seems likely that an app needs more detailed textures, start preparing new mipmap data. Depending on what kind of system you are implementing, you might make a network request to your server or render a new mipmap procedurally on the device. When you have the data, copy it into the appropriate mipmaps, and update the range of mipmaps that your shaders can sample. For example, in the following diagram, two additional levels of mipmaps were loaded and copied into the texture.

![image](https://docs-assets.developer.apple.com/published/2dd86d163e634d814054ca79a316478b/dynamically-adjusting-texture-level-of-detail-2%402x.png)

## See also

### Texture mipmapping
- [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps) — Avoid texture-rendering artifacts and reduce the GPU’s workload by creating smaller versions of a texture.
- [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture) — Decide whether a texture that you’re creating needs mipmaps.
- [Copying data into or out of mipmaps](https://developer.apple.com/documentation/metal/copying-data-into-or-out-of-mipmaps) — Specify which mipmaps that the data transfer affects.
- [Generating mipmap data](https://developer.apple.com/documentation/metal/generating-mipmap-data) — Create your mipmaps either when you author content or at runtime.
- [Adding mipmap filtering to samplers](https://developer.apple.com/documentation/metal/adding-mipmap-filtering-to-samplers) — Specify how the GPU samples mipmaps in your textures.
- [Restricting access to specific mipmaps](https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps) — Set the range of mipmap levels that a sampler can access.
- [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) — Determine in advance which mipmap levels the GPU requires to sample a texture.
