# PropReader testing
PropReader is a small parsing library for properties files.  

## Examples
prop_reader can be used to parse properties files that are delimited with either ':' or '='.  It also has built-in support for ignoring full line comments as well as in-line comments.

```
# Properties file that is consider valid by prop_reader
first.property: "This is a property"
second.property: "another" # Here is a comment example
# This is a full-line comment example
third="has an equals"
```

## Usage
Add the following to your `cargo.toml`:

```
[dependencies]
prop_reader = "0.0.1"
```
From there, it can be used as simply as:
```
let properties: PropReader = PropReader::new("example.properties");
let server_ip = properties.get("server.ip");
let server_port = properties.get("server.port");
```

## License
prop_reader is licensed under either of:

* Apache License, 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
* MIT (https://opensource.org/license/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
