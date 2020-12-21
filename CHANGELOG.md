# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2020-12-21
### Added
- Addons from Tukui can now be handled by specifying their names in a `tukui`
section in the config file.
- The application now logs status messages to the console and to a log file.
- The application now reads command line arguments.
- Command line argument `config`. Used to specify a different path for the
config file.
- Command line argument `verbose`. Used to make the application show all output
messages instead of just warnings and errors.
- Command line argument `force`. This does not do anything yet.

## [0.1.0] - 2020-12-19
### Added
- The application reads a list of addons from a config file.
- The application downloads requested addons from GitHub and CurseForge.
- The application installs downloaded addons to a specified addon folder.
