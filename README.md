<div align="center">

  # Xshe – Cross-Shell Environment Vars

  [![Documentation][icon-docs]][docs]
  [![View on Crates.io][icon-link-crates]][crates]
  [![Fork me on GitHub][icon-fork]][fork]
  [![Leave a GitHub Repo Star][icon-star]][repo]
  [![Open an Issue][icon-issue]][new issue]

  [![GitHub Release Status][icon-release]][release workflows]
  [![Libraries.io dependency status][icon-depend]][libraries.io tree]
  [![Latest Crates.io Release][icon-crates]][crates]
  [![Latest GitHub Release][icon-gh-release]][gh release]
  [![Crates.io downloads][icon-crates-downloads]][crates]

</div>


**Xshe** allows for setting <u>Sh</u>ell <u>E</u>nvironment Variables across multiple shells with a single TOML
configuration file.

Instead of writing multiple similar files for each shell you use,
you can instead create one file and use it for every shell with **xshe**!

All you have to do is [add a single line](docs/cli.md#sourcing-the-xshetoml-file) to all of your shells' startup scripts,
and `xshe` will set your environment variable across all of them.

To use **xshe**, you write lines in a `xshe.toml` file like this _(in [TOML] format)_:

```toml
CARGO_HOME = "~/.cargo"
EDITOR = "$(which nano)"
```

Create a file like this once and use it everywhere, for every shell!
`xshe` can convert this into the format for every supported shell.

<!--When updating this list, update the icon *AND* the alt text -->
[![Shells - bash | zsh | fish][icon-shells]](#)
[![Coming Soon - elvish | dash | xonsh | tcsh][icon-future-shells]][future shells]

## Usage and Documentation

View the documentation for `xshe` online at [xshe.superatomic.dev][docs]
or [locally by opening the docs](docs/README.md).

[docs]: https://xshe.superatomic.dev

## Quick install
* [With Cargo](docs/install#with-cargo)
* [With Homebrew](docs/install#with-homebrew)
* [As a File Download](docs/install#as-a-file-download)
* [Build from Source](docs/install#build-from-source)

<div align="center">

  ---

  The branch `main` is ahead of the current release.
  If you are looking for the documentation for the latest released version,
  [switch to the `0.4.2` release branch](https://github.com/superatomic/xshe/tree/v0.4.2),
  or view the documentation on [Lib.rs][lib.rs] or [Crates.io][crates].
  
  ![GitHub commits since latest release (by date)](https://img.shields.io/github/commits-since/superatomic/xshe/latest/main)

  ---

</div>

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.txt) or [www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

---

<div align=center>
  
  [![GitHub.com][icon-link-github]][repo]
  [![Crates.io][icon-link-crates]][crates]
  [![Lib.rs][icon-link-lib.rs]][lib.rs]
  [![Libraries.io][icon-link-libraries]][libraries.io]

</div>

<!-- Icons from https://primer.github.io/octicons/ -->
<!-- Thanks to https://base64.guru/converter/encode/image/svg for generating base64 representations of the icons -->

[icon-link-github]: https://img.shields.io/badge/-GitHub.com-2ea44f?style=flat&logoColor=white&logo=github
[icon-link-crates]: https://img.shields.io/badge/-Crates.io-ffc832?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojMDAwIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik04Ljg3OC4zOTJhMS43NSAxLjc1IDAgMDAtMS43NTYgMGwtNS4yNSAzLjA0NUExLjc1IDEuNzUgMCAwMDEgNC45NTF2Ni4wOThjMCAuNjI0LjMzMiAxLjIuODcyIDEuNTE0bDUuMjUgMy4wNDVhMS43NSAxLjc1IDAgMDAxLjc1NiAwbDUuMjUtMy4wNDVjLjU0LS4zMTMuODcyLS44OS44NzItMS41MTRWNC45NTFjMC0uNjI0LS4zMzItMS4yLS44NzItMS41MTRMOC44NzguMzkyek03Ljg3NSAxLjY5YS4yNS4yNSAwIDAxLjI1IDBsNC42MyAyLjY4NUw4IDcuMTMzIDMuMjQ1IDQuMzc1bDQuNjMtMi42ODV6TTIuNSA1LjY3N3Y1LjM3MmMwIC4wOS4wNDcuMTcxLjEyNS4yMTZsNC42MjUgMi42ODNWOC40MzJMMi41IDUuNjc3em02LjI1IDguMjcxbDQuNjI1LTIuNjgzYS4yNS4yNSAwIDAwLjEyNS0uMjE2VjUuNjc3TDguNzUgOC40MzJ2NS41MTZ6Ij48L3BhdGg+PC9zdmc+
[icon-link-lib.rs]: https://img.shields.io/badge/-Lib.rs-bb44ee?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDEuNzVBLjc1Ljc1IDAgMDEuNzUgMWg0LjI1M2MxLjIyNyAwIDIuMzE3LjU5IDMgMS41MDFBMy43NDQgMy43NDQgMCAwMTExLjAwNiAxaDQuMjQ1YS43NS43NSAwIDAxLjc1Ljc1djEwLjVhLjc1Ljc1IDAgMDEtLjc1Ljc1aC00LjUwN2EyLjI1IDIuMjUgMCAwMC0xLjU5MS42NTlsLS42MjIuNjIxYS43NS43NSAwIDAxLTEuMDYgMGwtLjYyMi0uNjIxQTIuMjUgMi4yNSAwIDAwNS4yNTggMTNILjc1YS43NS43NSAwIDAxLS43NS0uNzVWMS43NXptOC43NTUgM2EyLjI1IDIuMjUgMCAwMTIuMjUtMi4yNUgxNC41djloLTMuNzU3Yy0uNzEgMC0xLjQuMjAxLTEuOTkyLjU3MmwuMDA0LTcuMzIyem0tMS41MDQgNy4zMjRsLjAwNC01LjA3My0uMDAyLTIuMjUzQTIuMjUgMi4yNSAwIDAwNS4wMDMgMi41SDEuNXY5aDMuNzU3YTMuNzUgMy43NSAwIDAxMS45OTQuNTc0eiI+PC9wYXRoPjwvc3ZnPg==
[icon-link-libraries]: https://img.shields.io/badge/-Libraries.io-337ab7?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBkPSJNOC40NyA0Ljk3YS43NS43NSAwIDAwMCAxLjA2TDkuOTQgNy41IDguNDcgOC45N2EuNzUuNzUgMCAxMDEuMDYgMS4wNmwyLTJhLjc1Ljc1IDAgMDAwLTEuMDZsLTItMmEuNzUuNzUgMCAwMC0xLjA2IDB6TTYuNTMgNi4wM2EuNzUuNzUgMCAwMC0xLjA2LTEuMDZsLTIgMmEuNzUuNzUgMCAwMDAgMS4wNmwyIDJhLjc1Ljc1IDAgMTAxLjA2LTEuMDZMNS4wNiA3LjVsMS40Ny0xLjQ3eiI+PC9wYXRoPjxwYXRoIHN0eWxlPSJmaWxsOiNmZmYiIGZpbGwtcnVsZT0iZXZlbm9kZCIgZD0iTTEyLjI0NiAxMy4zMDdhNy41IDcuNSAwIDExMS4wNi0xLjA2bDIuNDc0IDIuNDczYS43NS43NSAwIDExLTEuMDYgMS4wNmwtMi40NzQtMi40NzN6TTEuNSA3LjVhNiA2IDAgMTExMC4zODYgNC4wOTQuNzUuNzUgMCAwMC0uMjkyLjI5M0E2IDYgMCAwMTEuNSA3LjV6Ij48L3BhdGg+PC9zdmc+

[icon-fork]:  https://img.shields.io/badge/-Fork%20me%20on%20Github-teal?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik01IDMuMjVhLjc1Ljc1IDAgMTEtMS41IDAgLjc1Ljc1IDAgMDExLjUgMHptMCAyLjEyMmEyLjI1IDIuMjUgMCAxMC0xLjUgMHYuODc4QTIuMjUgMi4yNSAwIDAwNS43NSA4LjVoMS41djIuMTI4YTIuMjUxIDIuMjUxIDAgMTAxLjUgMFY4LjVoMS41YTIuMjUgMi4yNSAwIDAwMi4yNS0yLjI1di0uODc4YTIuMjUgMi4yNSAwIDEwLTEuNSAwdi44NzhhLjc1Ljc1IDAgMDEtLjc1Ljc1aC00LjVBLjc1Ljc1IDAgMDE1IDYuMjV2LS44Nzh6bTMuNzUgNy4zNzhhLjc1Ljc1IDAgMTEtMS41IDAgLjc1Ljc1IDAgMDExLjUgMHptMy04Ljc1YS43NS43NSAwIDEwMC0xLjUuNzUuNzUgMCAwMDAgMS41eiI+PC9wYXRoPjwvc3ZnPg==
[icon-docs]:  https://img.shields.io/badge/-Documentation-9cf?style=flat&logoColor=black&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojMDAwIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDEuNzVBLjc1Ljc1IDAgMDEuNzUgMWg0LjI1M2MxLjIyNyAwIDIuMzE3LjU5IDMgMS41MDFBMy43NDQgMy43NDQgMCAwMTExLjAwNiAxaDQuMjQ1YS43NS43NSAwIDAxLjc1Ljc1djEwLjVhLjc1Ljc1IDAgMDEtLjc1Ljc1aC00LjUwN2EyLjI1IDIuMjUgMCAwMC0xLjU5MS42NTlsLS42MjIuNjIxYS43NS43NSAwIDAxLTEuMDYgMGwtLjYyMi0uNjIxQTIuMjUgMi4yNSAwIDAwNS4yNTggMTNILjc1YS43NS43NSAwIDAxLS43NS0uNzVWMS43NXptOC43NTUgM2EyLjI1IDIuMjUgMCAwMTIuMjUtMi4yNUgxNC41djloLTMuNzU3Yy0uNzEgMC0xLjQuMjAxLTEuOTkyLjU3MmwuMDA0LTcuMzIyem0tMS41MDQgNy4zMjRsLjAwNC01LjA3My0uMDAyLTIuMjUzQTIuMjUgMi4yNSAwIDAwNS4wMDMgMi41SDEuNXY5aDMuNzU3YTMuNzUgMy43NSAwIDAxMS45OTQuNTc0eiI+PC9wYXRoPjwvc3ZnPg==
[icon-star]:  https://img.shields.io/badge/-Star%20Repo-action?style=flat&logoColor=white&color=F25278&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik04IC4yNWEuNzUuNzUgMCAwMS42NzMuNDE4bDEuODgyIDMuODE1IDQuMjEuNjEyYS43NS43NSAwIDAxLjQxNiAxLjI3OWwtMy4wNDYgMi45Ny43MTkgNC4xOTJhLjc1Ljc1IDAgMDEtMS4wODguNzkxTDggMTIuMzQ3bC0zLjc2NiAxLjk4YS43NS43NSAwIDAxLTEuMDg4LS43OWwuNzItNC4xOTRMLjgxOCA2LjM3NGEuNzUuNzUgMCAwMS40MTYtMS4yOGw0LjIxLS42MTFMNy4zMjcuNjY4QS43NS43NSAwIDAxOCAuMjV6bTAgMi40NDVMNi42MTUgNS41YS43NS43NSAwIDAxLS41NjQuNDFsLTMuMDk3LjQ1IDIuMjQgMi4xODRhLjc1Ljc1IDAgMDEuMjE2LjY2NGwtLjUyOCAzLjA4NCAyLjc2OS0xLjQ1NmEuNzUuNzUgMCAwMS42OTggMGwyLjc3IDEuNDU2LS41My0zLjA4NGEuNzUuNzUgMCAwMS4yMTYtLjY2NGwyLjI0LTIuMTgzLTMuMDk2LS40NWEuNzUuNzUgMCAwMS0uNTY0LS40MUw4IDIuNjk0di4wMDF6Ij48L3BhdGg+PC9zdmc+
[icon-issue]: https://img.shields.io/badge/-Open%20an%20Issue-palegreen?style=flat&logoColor=black&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojMDAwIiBkPSJNOCA5LjVhMS41IDEuNSAwIDEwMC0zIDEuNSAxLjUgMCAwMDAgM3oiPjwvcGF0aD48cGF0aCBzdHlsZT0iZmlsbDojMDAwIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik04IDBhOCA4IDAgMTAwIDE2QTggOCAwIDAwOCAwek0xLjUgOGE2LjUgNi41IDAgMTExMyAwIDYuNSA2LjUgMCAwMS0xMyAweiI+PC9wYXRoPjwvc3ZnPg==

[icon-release]: https://img.shields.io/github/workflow/status/superatomic/xshe/release?label=release%20build&style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDEuNzVDMCAuNzg0Ljc4NCAwIDEuNzUgMGgzLjVDNi4yMTYgMCA3IC43ODQgNyAxLjc1djMuNUExLjc1IDEuNzUgMCAwMTUuMjUgN0g0djRhMSAxIDAgMDAxIDFoNHYtMS4yNUM5IDkuNzg0IDkuNzg0IDkgMTAuNzUgOWgzLjVjLjk2NiAwIDEuNzUuNzg0IDEuNzUgMS43NXYzLjVBMS43NSAxLjc1IDAgMDExNC4yNSAxNmgtMy41QTEuNzUgMS43NSAwIDAxOSAxNC4yNXYtLjc1SDVBMi41IDIuNSAwIDAxMi41IDExVjdoLS43NUExLjc1IDEuNzUgMCAwMTAgNS4yNXYtMy41em0xLjc1LS4yNWEuMjUuMjUgMCAwMC0uMjUuMjV2My41YzAgLjEzOC4xMTIuMjUuMjUuMjVoMy41YS4yNS4yNSAwIDAwLjI1LS4yNXYtMy41YS4yNS4yNSAwIDAwLS4yNS0uMjVoLTMuNXptOSA5YS4yNS4yNSAwIDAwLS4yNS4yNXYzLjVjMCAuMTM4LjExMi4yNS4yNS4yNWgzLjVhLjI1LjI1IDAgMDAuMjUtLjI1di0zLjVhLjI1LjI1IDAgMDAtLjI1LS4yNWgtMy41eiI+PC9wYXRoPjwvc3ZnPg==
[icon-depend]: https://img.shields.io/librariesio/release/cargo/xshe?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik02LjEyMi4zOTJhMS43NSAxLjc1IDAgMDExLjc1NiAwbDUuMjUgMy4wNDVjLjU0LjMxMy44NzIuODkuODcyIDEuNTE0VjcuMjVhLjc1Ljc1IDAgMDEtMS41IDBWNS42NzdMNy43NSA4LjQzMnY2LjM4NGExIDEgMCAwMS0xLjUwMi44NjVMLjg3MiAxMi41NjNBMS43NSAxLjc1IDAgMDEwIDExLjA0OVY0Ljk1MWMwLS42MjQuMzMyLTEuMi44NzItMS41MTRMNi4xMjIuMzkyek03LjEyNSAxLjY5bDQuNjMgMi42ODVMNyA3LjEzMyAyLjI0NSA0LjM3NWw0LjYzLTIuNjg1YS4yNS4yNSAwIDAxLjI1IDB6TTEuNSAxMS4wNDlWNS42NzdsNC43NSAyLjc1NXY1LjUxNmwtNC42MjUtMi42ODNhLjI1LjI1IDAgMDEtLjEyNS0uMjE2em0xMS42NzItLjI4MmEuNzUuNzUgMCAxMC0xLjA4Ny0xLjAzNGwtMi4zNzggMi41YS43NS43NSAwIDAwMCAxLjAzNGwyLjM3OCAyLjVhLjc1Ljc1IDAgMTAxLjA4Ny0xLjAzNEwxMS45OTkgMTMuNWgzLjI1MWEuNzUuNzUgMCAwMDAtMS41aC0zLjI1MWwxLjE3My0xLjIzM3oiPjwvcGF0aD48L3N2Zz4=
[icon-crates]: https://img.shields.io/crates/v/xshe?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik04Ljg3OC4zOTJhMS43NSAxLjc1IDAgMDAtMS43NTYgMGwtNS4yNSAzLjA0NUExLjc1IDEuNzUgMCAwMDEgNC45NTF2Ni4wOThjMCAuNjI0LjMzMiAxLjIuODcyIDEuNTE0bDUuMjUgMy4wNDVhMS43NSAxLjc1IDAgMDAxLjc1NiAwbDUuMjUtMy4wNDVjLjU0LS4zMTMuODcyLS44OS44NzItMS41MTRWNC45NTFjMC0uNjI0LS4zMzItMS4yLS44NzItMS41MTRMOC44NzguMzkyek03Ljg3NSAxLjY5YS4yNS4yNSAwIDAxLjI1IDBsNC42MyAyLjY4NUw4IDcuMTMzIDMuMjQ1IDQuMzc1bDQuNjMtMi42ODV6TTIuNSA1LjY3N3Y1LjM3MmMwIC4wOS4wNDcuMTcxLjEyNS4yMTZsNC42MjUgMi42ODNWOC40MzJMMi41IDUuNjc3em02LjI1IDguMjcxbDQuNjI1LTIuNjgzYS4yNS4yNSAwIDAwLjEyNS0uMjE2VjUuNjc3TDguNzUgOC40MzJ2NS41MTZ6Ij48L3BhdGg+PC9zdmc+
[icon-gh-release]: https://img.shields.io/github/v/release/superatomic/xshe?include_prereleases&style=flat&logo=github
[icon-crates-downloads]: https://img.shields.io/crates/d/xshe?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik03LjQ3IDEwLjc4YS43NS43NSAwIDAwMS4wNiAwbDMuNzUtMy43NWEuNzUuNzUgMCAwMC0xLjA2LTEuMDZMOC43NSA4LjQ0VjEuNzVhLjc1Ljc1IDAgMDAtMS41IDB2Ni42OUw0Ljc4IDUuOTdhLjc1Ljc1IDAgMDAtMS4wNiAxLjA2bDMuNzUgMy43NXpNMy43NSAxM2EuNzUuNzUgMCAwMDAgMS41aDguNWEuNzUuNzUgMCAwMDAtMS41aC04LjV6Ij48L3BhdGg+PC9zdmc+

[icon-shells]: https://img.shields.io/badge/Shells-bash_|_zsh_|_fish-2ea44f?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDIuNzVDMCAxLjc4NC43ODQgMSAxLjc1IDFoMTIuNWMuOTY2IDAgMS43NS43ODQgMS43NSAxLjc1djEwLjVBMS43NSAxLjc1IDAgMDExNC4yNSAxNUgxLjc1QTEuNzUgMS43NSAwIDAxMCAxMy4yNVYyLjc1em0xLjc1LS4yNWEuMjUuMjUgMCAwMC0uMjUuMjV2MTAuNWMwIC4xMzguMTEyLjI1LjI1LjI1aDEyLjVhLjI1LjI1IDAgMDAuMjUtLjI1VjIuNzVhLjI1LjI1IDAgMDAtLjI1LS4yNUgxLjc1ek03LjI1IDhhLjc1Ljc1IDAgMDEtLjIyLjUzbC0yLjI1IDIuMjVhLjc1Ljc1IDAgMTEtMS4wNi0xLjA2TDUuNDQgOCAzLjcyIDYuMjhhLjc1Ljc1IDAgMTExLjA2LTEuMDZsMi4yNSAyLjI1Yy4xNDEuMTQuMjIuMzMxLjIyLjUzem0xLjUgMS41YS43NS43NSAwIDAwMCAxLjVoM2EuNzUuNzUgMCAwMDAtMS41aC0zeiI+PC9wYXRoPjwvc3ZnPg==
[icon-future-shells]: https://img.shields.io/badge/Coming_Soon-elvish_|_dash_|_xonsh_|_tcsh-yellow?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0yLjUgMS43NWEuMjUuMjUgMCAwMS4yNS0uMjVoOC41YS4yNS4yNSAwIDAxLjI1LjI1djcuNzM2YS43NS43NSAwIDEwMS41IDBWMS43NUExLjc1IDEuNzUgMCAwMDExLjI1IDBoLTguNUExLjc1IDEuNzUgMCAwMDEgMS43NXYxMS41YzAgLjk2Ni43ODQgMS43NSAxLjc1IDEuNzVoMy4xN2EuNzUuNzUgMCAwMDAtMS41SDIuNzVhLjI1LjI1IDAgMDEtLjI1LS4yNVYxLjc1ek00Ljc1IDRhLjc1Ljc1IDAgMDAwIDEuNWg0LjVhLjc1Ljc1IDAgMDAwLTEuNWgtNC41ek00IDcuNzVBLjc1Ljc1IDAgMDE0Ljc1IDdoMmEuNzUuNzUgMCAwMTAgMS41aC0yQS43NS43NSAwIDAxNCA3Ljc1em0xMS43NzQgMy41MzdhLjc1Ljc1IDAgMDAtMS4wNDgtMS4wNzRMMTAuNyAxNC4xNDUgOS4yODEgMTIuNzJhLjc1Ljc1IDAgMDAtMS4wNjIgMS4wNThsMS45NDMgMS45NWEuNzUuNzUgMCAwMDEuMDU1LjAwOGw0LjU1Ny00LjQ1eiI+PC9wYXRoPjwvc3ZnPg==

[fork]: https://github.com/superatomic/xshe/fork
[new issue]: https://github.com/superatomic/xshe/issues/new/choose
[repo]: https://github.com/superatomic/xshe/
[lib.rs]: https://lib.rs/crates/xshe
[libraries.io]: https://libraries.io/cargo/xshe
[crates]: https://crates.io/crates/xshe

[future shells]: https://github.com/users/superatomic/projects/1

[libraries.io tree]: https://libraries.io/cargo/xshe/tree?kind=normal

[gh release]: https://github.com/superatomic/xshe/releases/
[release workflows]: https://github.com/superatomic/xshe/actions/workflows/release.yml

[toml]: https://toml.io/en/
