# init(accelerationStructureSize:buildScratchBufferSize:refitScratchBufferSize:)

*Initializer · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/init(accelerationstructuresize:buildscratchbuffersize:refitscratchbuffersize:)>

Creates an acceleration sizes instance with specific values.

## Declaration

```swift
init(accelerationStructureSize: Int, buildScratchBufferSize: Int, refitScratchBufferSize: Int)
```

## Parameters

- **accelerationStructureSize** — The size of the acceleration structure, in bytes.
- **buildScratchBufferSize** — The amount of scratch memory, in bytes, the GPU devices needs to build the acceleration structure.
- **refitScratchBufferSize** — The amount of scratch memory, in bytes, the GPU device needs to refit the acceleration structure.

## See also

### Creating an acceleration size structure
- [init()](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/init()) — Creates an acceleration sizes instance with default values.
