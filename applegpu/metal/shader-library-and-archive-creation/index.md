# Shader library and archive creation

*API Collection*

<https://developer.apple.com/documentation/metal/shader-library-and-archive-creation>

Create static and dynamic shader libraries, and binary shader archives.

## Topics

### Creating shader libraries
- [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) — Creates a Metal library instance that contains the functions from your app’s default Metal library.
- [makeDefaultLibrary(bundle:)](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary(bundle:)) — Creates a Metal library instance that contains the functions in a bundle’s default Metal library.
- [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) — Creates a Metal library instance that contains the functions in the Metal library file at a URL.
- [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) — Synchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) — Asynchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) — Synchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(stitchedDescriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:completionhandler:)) — Asynchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)-7khmh) — Creates a Metal library instance that contains the functions in a precompiled Metal library.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) — Creates a Metal library instance from a dispatch-data instance that contains the functions in a precompiled Metal library.
- [MTLNewLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewlibrarycompletionhandler) — A completion handler signature a method calls when it finishes creating a Metal library.
- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:)) — Creates a Metal library instance that contains the functions in the Metal library file at a file path.

### Creating dynamic shader libraries
- [supportsDynamicLibraries](https://developer.apple.com/documentation/metal/mtldevice/supportsdynamiclibraries) — A Boolean value that indicates whether the GPU device can create and use dynamic libraries in compute pipelines.
- [supportsRenderDynamicLibraries](https://developer.apple.com/documentation/metal/mtldevice/supportsrenderdynamiclibraries) — A Boolean value that indicates whether the GPU device can create and use dynamic libraries in render pipelines.
- [makeDynamicLibrary(library:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(library:)) — Creates a Metal dynamic library instance from a Metal library instance.
- [makeDynamicLibrary(url:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(url:)) — Creates a Metal dynamic library instance that contains the functions in the Metal library file at a URL.
- [MTLDynamicLibraryError.Code](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/code) — Error codes that Metal can generate when creating dynamic libraries.
- [MTLDynamicLibraryDomain](https://developer.apple.com/documentation/metal/mtldynamiclibrarydomain) — The domain for Metal dynamic library errors.

### Creating binary shader archives
- [makeBinaryArchive(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makebinaryarchive(descriptor:)) — Creates a Metal binary archive instance.
- [MTLBinaryArchiveDescriptor](https://developer.apple.com/documentation/metal/mtlbinaryarchivedescriptor) — A description of a binary shader archive that you want to create.
- [MTLBinaryArchiveError.Code](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/code) — Error codes when creating binary archives of compiled shader code.
- [MTLBinaryArchiveDomain](https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain) — The domain for Metal binary archive errors.

## See also

### Working with GPU devices
- [Device inspection](https://developer.apple.com/documentation/metal/device-inspection) — Locate and identify a GPU and the features it supports, and sample its counters.
- [Work submission](https://developer.apple.com/documentation/metal/work-submission) — Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.
- [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) — Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.
- [Resource creation](https://developer.apple.com/documentation/metal/resource-creation) — Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.
