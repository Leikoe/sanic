# maxThreadsPerThreadgroup

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/maxthreadsperthreadgroup>

The maximum number of threads along each dimension of a threadgroup.

## Declaration

```swift
var maxThreadsPerThreadgroup: MTLSize { get }
```

## Discussion

This property reports the maximum thread group size the device can support for a trivial shader. This size isn’t guaranteed for all shaders. For the actual thread group size of a specific compute shader, see the [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/maxtotalthreadsperthreadgroup) property of your compute pipeline state.

For more information on the specific threadgroup limits of each GPU family, see the Metal feature set tables:

- [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf)

- [Metal feature set tables (Numbers)](https://developer.apple.com/metal/metal-feature-set-tables.zip)

## See also

### Checking compute support
- [maxThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtldevice/maxthreadgroupmemorylength) — The maximum threadgroup memory available to a compute kernel, in bytes.
