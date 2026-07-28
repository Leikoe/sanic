# setConstantValue(_:type:withName:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:withname:)>

Sets a value for a function constant with a specific name.

## Declaration

```swift
func setConstantValue(_ value: UnsafeRawPointer, type: MTLDataType, withName name: String)
```

## Parameters

- **value** — A pointer to the constant value.
- **type** — The data type of the function constant.
- **name** — The name of the function constant.

## Discussion

The first example declares a single function constant in a Metal Shading Language file.

```metal
constant bool a [[ function_constant(0) ]];
```

The next example sets that Boolean value by providing its specific name.

```swift
var a = true
let constantValues = MTLFunctionConstantValues()
constantValues.setConstantValue(&a, type: .bool, withName: "a")
```

```objective-c
const bool a = true;
MTLFunctionConstantValues* constantValues = [MTLFunctionConstantValues new];
[constantValues setConstantValue:&a type:MTLDataTypeBool withName:@"a"];
```

## See also

### Setting constant values
- [setConstantValue(_:type:index:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:index:)) — Sets a value for a function constant at a specific index.
- [setConstantValues(_:type:range:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalues(_:type:range:)) — Sets values for a group of function constants within a specific index range.
