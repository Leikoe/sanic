# maxVertexBufferBindCount

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxvertexbufferbindcount>

The maximum number of buffers that you can set per command for the vertex stage.

## Declaration

```swift
var maxVertexBufferBindCount: Int { get set }
```

## Discussion

Metal ignores this property if [inheritBuffers](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritbuffers) is [true](https://developer.apple.com/documentation/Swift/true) or if you configured [commandTypes](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/commandtypes) for compute commands. Metal needs to reserve enough memory in each command to store this many arguments. Use the smallest value that works for all commands you plan to encode into the indirect command buffer.

## See also

### Declaring the maximum number of argument buffers per command
- [maxFragmentBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxfragmentbufferbindcount) — The maximum number of buffers that you can set per command for the fragment stage.
- [maxKernelBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxkernelbufferbindcount) — The maximum number of buffers that you can set per command for the compute kernel.
