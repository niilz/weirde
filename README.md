# POC Rust - library to De-Serialize Protobuf-Messages

## Supported Wire-Types

- Varint for u64
- Len for String

## About this minimal (non productional) lib

It has two lib modules

- `deserialize` (convert from binary hex value to on of the supported variants, which will have contain the field-number and the value)
- `serialize` (convert a type that implements `Proto` into binary protobuf message hex-value)

For more information about the _Encoding_ rules refer to the (official documentation)[https://protobuf.dev/programming-guides/encoding/]

## Verify values with `protoscope`

For installation and usage documentation of `protoscope` refer the (protoscope github repo)[https://github.com/protocolbuffers/protoscope/]
(As an alternative, in-browser tool see (protobufpal)[https://www.protobufpal.com/])

- write a pseudo message into file
  ```
  echo '1: 42' > msg.txt
  echo '2: {"Foo"}' >> msg.txt
  ```
- convert to binary
  ```
  protoscope -s msg.txt > msg.bin
  # look at hex value
  xxd msg.bin
  ```
- to go from bin to message

  ```
  protoscope msg.bin

  ```

## Next Steps (low hanging fruit)

- Support other wire types (other `VARINT`s like `bool`, signed numbers, repeated types, nexted types etc.)
- Implement derive-macro for `Proto` (at least on structs), which internally converts every field to it's `WireType` equivalent and calls `serialize` on it.
