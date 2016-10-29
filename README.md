# tiled-rs

This is a simple deserializer for tiled tile maps.

It uses `serde` and `serde_json` for the actual deserialization, and just
provides simple structs for the relevant data (it is not currently complete and
it probably never will, since I am implementing only the features I need in
other projects).

Currently it only supports the `json` format, but I would like to support the
`xml` one (`tmx`) in the future (this should be easy with serde, but I have
problems using `serde_xml`).

## Requirements

A nightly version of rust (because I use the custom derive provided by serde)

