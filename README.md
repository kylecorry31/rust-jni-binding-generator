# Rust JNI Binding Generator
A JNI binding generator for Rust.

## Config file
- `package-name`: The rust crate to generate bindings for.
- `members`: The list of public members to create JNI wrappers for.
  - `name`: The fully qualified name of the member.
  - `type`: The type of the member. Valid values are: `function`
  - `inputs`: Inputs for a function type.
    - `name`: The name of the input parameter.
    - `type`: The Rust type of the input parameter.
  - `output`: The Rust type of the output.

```json
{
  "put-your-crate-name-here": {
    "members": [
      {
        "name": "put_your_crate_name_here::fully::qualified::function_name",
        "type": "function",
        "inputs": [
          {
            "name": "a",
            "type": "f32"
          },
          {
            "name": "b",
            "type": "f32"
          }
        ],
        "output": "f32"
      }
    ]
  }
}
```

## Generate bindings
Run `cargo run <lib> <config> <java_package>` to generate the bindings.

- `lib`: The name of the cargo library to generate. This can be anything.
- `config`: The path to the config JSON file.
- `java_package`: The destination java package to generate the binding at.
