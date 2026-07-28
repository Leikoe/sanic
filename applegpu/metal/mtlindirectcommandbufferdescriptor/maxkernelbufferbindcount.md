# maxKernelBufferBindCount

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxkernelbufferbindcount>

The maximum number of buffers that you can set per command for the compute kernel.

## Declaration

```swift
var maxKernelBufferBindCount: Int { get set }
```

## Discussion

Metal ignores this property if [inheritBuffers](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritbuffers) is [true](https://developer.apple.com/documentation/Swift/true) or if you configured [commandTypes](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/commandtypes) for rendering commands. Metal needs to reserve enough memory in each command to store this many arguments. Use the smallest value that works for all commands you plan to encode into the indirect command buffer.

## See also

### Declaring the maximum number of argument buffers per command
- [maxVertexBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxvertexbufferbindcount) — The maximum number of buffers that you can set per command for the vertex stage.
- [maxFragmentBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxfragmentbufferbindcount) — The maximum number of buffers that you can set per command for the fragment stage.
