TODO
====
- [ ] Tests
- [ ] incomplete csv file plugins
- [ ] incomplete csv file snippets
- [ ] lua plugin for fwdt input files for validation
- [ ] ingest data from CAT or other processes into incomplete csvs
- [ ] Builds

Builds
=======

A temporary reminder to myself on how to build for different systems.

| Build     | command                                                                                               |
| --------- | ----------------------------------------------------------------------------------------------------- |
| `source`  | `cargo build`                                                                                         |
| `debian`  | `cargo deb`                                                                                           |
| `rpm `    | `alien --verbose --to-rpm ./target/debian/fwdt_*_amd64.deb & mv fwdt-0.1.1-2.x86_64.rpm ./target/rpm` |
| `windows` | `cross build `                                                                                        |
| `mac`     | `...`                                                                                                 |