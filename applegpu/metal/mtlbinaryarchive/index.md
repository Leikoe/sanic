# MTLBinaryArchive

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbinaryarchive>

A container for pipeline state descriptors and their associated compiled shader code.

## Declaration

```swift
protocol MTLBinaryArchive : NSObjectProtocol
```

## Topics

### Identifying the archive
- [device](https://developer.apple.com/documentation/metal/mtlbinaryarchive/device) — The Metal device object that created the binary archive.
- [label](https://developer.apple.com/documentation/metal/mtlbinaryarchive/label) — A string that identifies the library.

### Adding pipeline descriptors
- [addComputePipelineFunctions(descriptor:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addcomputepipelinefunctions(descriptor:)) — Adds a description of a compute pipeline to the archive.
- [addRenderPipelineFunctions(descriptor:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addrenderpipelinefunctions(descriptor:)) — Adds a description of a render pipeline to the archive.
- [addTileRenderPipelineFunctions(descriptor:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addtilerenderpipelinefunctions(descriptor:)) — Adds a description of a tile renderer pipeline to the archive.
- [addFunction(descriptor:library:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addfunction(descriptor:library:)) — Adds a description of a function to the archive.

### Serializing archives
- [serialize(to:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/serialize(to:)) — Writes the contents of the archive to a file.

### Instance Methods
- [addLibrary(descriptor:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addlibrary(descriptor:))
- [addMeshRenderPipelineFunctions(descriptor:)](https://developer.apple.com/documentation/metal/mtlbinaryarchive/addmeshrenderpipelinefunctions(descriptor:))

## See also

### Shader library management
- [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) — A collection of Metal shader functions.
- [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) — A dynamically linkable representation of compiled shader code for a specific Metal device object.
- [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) — Compilation settings for a Metal shader library.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
- [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) — The optimization options for the Metal compiler.
