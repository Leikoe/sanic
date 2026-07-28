# setConstantValue(_:type:index:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:index:)>

Sets a value for a function constant at a specific index.

## Declaration

```swift
func setConstantValue(_ value: UnsafeRawPointer, type: MTLDataType, index: Int)
```

## Parameters

- **value** — A pointer to the constant value.
- **type** — The data type of the function constant.
- **index** — The index of the function constant.

## Discussion

Declare a single function constant in Metal Shading Language (MSL).

```metal
constant bool a [[ function_constant(0) ]];
```

Set its value by assigning with a specific index.

```swift
var a = true
let constantValues = MTLFunctionConstantValues()
constantValues.setConstantValue(&a, type: .bool, at: 0)
```

```objective-c
const bool a = true;
MTLFunctionConstantValues* constantValues = [MTLFunctionConstantValues new];
[constantValues setConstantValue:&a type:MTLDataTypeBool atIndex:0];
```

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Setting constant values
- [setConstantValue(_:type:withName:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:withname:)) — Sets a value for a function constant with a specific name.
- [setConstantValues(_:type:range:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalues(_:type:range:)) — Sets values for a group of function constants within a specific index range.
