# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.5.0] - 2025-04-26

### Changed
* upgrade to bevy `0.16`

## [0.4.0] - 2024-12-02

### Changed
* upgrade to bevy `0.15`

## [0.3.1] - 2024-11-04

### Changed
* upgrade `bevy_channel_trigger` to version that is backed by crossbeam for smaller wasm build

## [0.3.0] - 2024-11-04

### Changed
* use `bevy_channel_trigger` instead of `bevy_crossbeam_event`, this changes how you consume the events we sent to you, see readme for example
