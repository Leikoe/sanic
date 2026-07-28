# MTLFunctionOptions

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionoptions>

Options that define how Metal compiles a GPU function.

## Declaration

```swift
struct MTLFunctionOptions
```

## Topics

### Function compilation options
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlfunctionoptions/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [compileToBinary](https://developer.apple.com/documentation/metal/mtlfunctionoptions/compiletobinary) — An option that instructs the compiler to generate a binary format for dynamic linking.
- [pipelineIndependent](https://developer.apple.com/documentation/metal/mtlfunctionoptions/pipelineindependent) — An option that generates the same function handle across all pipeline states that link a function, which lets you share function tables across pipeline states.
- [storeFunctionInMetalPipelinesScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalpipelinesscript) — An option that instructs the compiler to store function information for inspecting binary archives.
- [storeFunctionInMetalScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalscript) — An option that instructs the compiler to store function information for inspecting binary archives.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlfunctionoptions/init(rawvalue:)) — Creates a new function options structure from a raw value.

## See also

### Identifying shader functions
- [device](https://developer.apple.com/documentation/metal/mtlfunction/device) — The device object that created the shader function.
- [label](https://developer.apple.com/documentation/metal/mtlfunction/label) — A string that identifies the shader function.
- [functionType](https://developer.apple.com/documentation/metal/mtlfunction/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [MTLFunctionType](https://developer.apple.com/documentation/metal/mtlfunctiontype) — The type of a top-level Metal Shading Language (MSL) function.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
