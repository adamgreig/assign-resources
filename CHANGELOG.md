# Changelog

## Unreleased

## v0.5.0 - 2024-01-14

* Overhaul to attribute macro
  * resources are now struct definitions
  * each resource is explicitly extracted
  * `#[cfg(...)]` along with other attribute macros work fine now
  * add tests

## v0.4.0 - 2023-12-02

* Rename generated struct to `AssignedResources` and make it pub
* Make type alises pub

## v0.3.0 - 2023-11-25

* Fix bug in type aliases

## v0.2.0 - 2023-11-25

* Add optional type aliases
* Mark struct fields `pub`

## v0.1.0 - 2023-11-22

* Initial release
