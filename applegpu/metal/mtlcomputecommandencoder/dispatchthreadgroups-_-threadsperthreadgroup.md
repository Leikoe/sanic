# dispatchThreadgroups(_:threadsPerThreadgroup:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(_:threadsperthreadgroup:)>

Encodes a compute dispatch command using a grid aligned to threadgroup boundaries.

## Declaration

```swift
func dispatchThreadgroups(_ threadgroupsPerGrid: MTLSize, threadsPerThreadgroup: MTLSize)
```

## Parameters

- **threadgroupsPerGrid** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads for each grid dimension.
- **threadsPerThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in a threadgroup.

## Discussion

> **Tip:**
>  Prefer using dispatchThreads for your kernel calls on `Apple4` and later Apple GPUs. See [Metal Feature Set Tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for information on hardware support.

Metal calculates the number of threads in a grid by multiplying `threadsPerThreadgroup` by `threadgroupsPerGrid`.

If the size of your data doesn’t match the size of the grid, perform boundary checks in your compute function to avoid accessing data out of bounds. See [Calculating threadgroup and grid sizes](https://developer.apple.com/documentation/metal/calculating-threadgroup-and-grid-sizes) for an example.

## See also

### Dispatching kernel calls directly
- [dispatchThreads(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreads(_:threadsperthreadgroup:)) — Encodes a compute command using an arbitrarily sized grid.
