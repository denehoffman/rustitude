# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.9...rustitude-gluex-v0.5.0) - 2024-07-30

### Added
- [**breaking**] add `Decay` enum to basically every amplitude to avoid hardcoding particle indices
- Add `Decay` enum to handle particle assignment (expandable in future) and make other enums into pyclasses

## [0.4.8](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.7...rustitude-gluex-v0.4.8) - 2024-07-23

### Other
- update benchmarks and tests to reflect new breaking syntax
- update amplitudes to match rustitude-cores latest updates by adding generics

## [0.4.6](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.5...rustitude-gluex-v0.4.6) - 2024-06-27

### Added
- remove RwLock from Dataset and adopt indexing methods for splitting data

### Fixed
- change FourMomentum sum implementation to take owned copies

### Other
- update all README.mds

## [0.4.5](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.4...rustitude-gluex-v0.4.5) - 2024-06-21

### Other
- update Cargo.toml dependencies

## [0.4.4](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.3...rustitude-gluex-v0.4.4) - 2024-06-20

### Added
- re-export nalgebra::Vector3 in rustitude_core::prelude since it gets used so often

### Other
- Merge pull request [#11](https://github.com/denehoffman/rustitude/pull/11) from denehoffman/development
- add tests for some GlueX amplitudes as well as some of the main crate functionality

## [0.4.3](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.2...rustitude-gluex-v0.4.3) - 2024-06-19

### Added
- add pole_product scaling to reduce numerical error near mass poles

### Fixed
- corrected P_gamma multiplicative factor in Zlm and OnePS
- corrected frame calculation in all HX/GJ-related amplitudes
- add more digits to channel masses in all K-matrix amplitudes
- corrected transposed g-matrix as well as a typo in one of the terms (channel 2 resonance 0)
- corrected a flipped minus sign in Chew-Mandelstam matrix calculation

### Other
- get rid of warning in pole_product (temporary)
- moved C-matrix to a chronologically sensible place
- move postfix operator to prefix for clarity

## [0.4.2](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.1...rustitude-gluex-v0.4.2) - 2024-06-17

### Other
- update Cargo.toml dependencies

## [0.4.1](https://github.com/denehoffman/rustitude/compare/rustitude-gluex-v0.4.0...rustitude-gluex-v0.4.1) - 2024-06-10

### Added
- add par_ versions for all compute and norm_int methods and refactor python accordingly. Also remove RwLocks and extra allocations in the Amplitude struct, which is a huge speedup

### Other
- bump versions because smart-release just doesn't know how to do anything I guess

## v0.3.0 (2024-05-24)

### Bug Fixes

 - <csr-id-405ed0cb2ad417ccf0041a49ce6fbd6b6693539c/> bump versions

### Documentation

 - <csr-id-45107c4c719503310ac171186b016212bf4d5370/> Update README.md

### New Features

 - <csr-id-6955773a3e96890efa4573d5c7bc355bb23a07e6/> add RustitudeError and remove unwraps, error handling should work in python as well

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 8 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump versions ([`405ed0c`](https://github.com/denehoffman/rustitude/commit/405ed0cb2ad417ccf0041a49ce6fbd6b6693539c))
    - Update README.md ([`45107c4`](https://github.com/denehoffman/rustitude/commit/45107c4c719503310ac171186b016212bf4d5370))
    - Merge pull request #3 from denehoffman/python_reorg ([`467caac`](https://github.com/denehoffman/rustitude/commit/467caacc688f94c074d28be1ec54a422d8d9ebc0))
    - Add RustitudeError and remove unwraps, error handling should work in python as well ([`6955773`](https://github.com/denehoffman/rustitude/commit/6955773a3e96890efa4573d5c7bc355bb23a07e6))
    - Move all pyo3 code to the py-rustitude crate ([`9bcdb46`](https://github.com/denehoffman/rustitude/commit/9bcdb4615fdb4df5b4566673fbed955930926b7c))
</details>

## v0.2.2 (2024-05-16)

<csr-id-0e94ec45850cb6129924b2be27793a17c51b03c2/>

### Bug Fixes

 - <csr-id-77054e334d90077decd54d4f970400efa1a31f47/> update amplitudes to account for changes in core

### Style

 - <csr-id-0e94ec45850cb6129924b2be27793a17c51b03c2/> get rid of extra git-files and media

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude-core v1.0.1, rustitude-gluex v0.2.2, rustitude v0.4.3 ([`fe0c603`](https://github.com/denehoffman/rustitude/commit/fe0c6036d4816587b1d65ae2697f3b241cfe579c))
    - Release rustitude-gluex v0.2.2, rustitude v0.4.3 ([`0062542`](https://github.com/denehoffman/rustitude/commit/006254211c6dda1924cede7818c94ab4dcf1f49a))
    - Get rid of extra git-files and media ([`0e94ec4`](https://github.com/denehoffman/rustitude/commit/0e94ec45850cb6129924b2be27793a17c51b03c2))
    - Update amplitudes to account for changes in core ([`77054e3`](https://github.com/denehoffman/rustitude/commit/77054e334d90077decd54d4f970400efa1a31f47))
</details>

## v0.2.1 (2024-05-15)

<csr-id-9089c84e481124ff764b24f42507ab14913fef07/>

### Refactor

 - <csr-id-9089c84e481124ff764b24f42507ab14913fef07/> major move required to have rustitude function properly as a Rust crate. I set this up very wrong the first time

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude-gluex v0.2.1, rustitude v0.4.2 ([`3a45077`](https://github.com/denehoffman/rustitude/commit/3a45077dcd7413ac50e5ec45dc98826b11d789fb))
    - Major move required to have rustitude function properly as a Rust crate. I set this up very wrong the first time ([`9089c84`](https://github.com/denehoffman/rustitude/commit/9089c84e481124ff764b24f42507ab14913fef07))
</details>

## v0.2.0 (2024-05-15)

<csr-id-9617a27322460b378fb022ef28561f31197fc86f/>

### Documentation

 - <csr-id-93dc0d6cfeb57f655c81f30c1e55b1c6b0460ccc/> update links for rustitude-gluex readme
 - <csr-id-95f85ed6b16400c882e7535c7fa113ead9876353/> update links in readmes

### Other

 - <csr-id-9617a27322460b378fb022ef28561f31197fc86f/> merge rustitude-core to crates subdirectory

### Bug Fixes

 - <csr-id-2495107c77f483b84a926090cd868ccec8296052/> need to set some dependency versions...
 - <csr-id-ee48e4039e40bbf2c5b23d230bbcd0213c41e888/> need to set some dependency versions...properly

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude-gluex v0.2.0, rustitude v0.4.0 ([`bc38f3a`](https://github.com/denehoffman/rustitude/commit/bc38f3af06807d8e6e1c3ca3a38581461ef934b8))
    - Need to set some dependency versions...properly ([`ee48e40`](https://github.com/denehoffman/rustitude/commit/ee48e4039e40bbf2c5b23d230bbcd0213c41e888))
    - Release rustitude-gluex v0.2.0, rustitude v0.4.0 ([`1369408`](https://github.com/denehoffman/rustitude/commit/1369408a7352be0e5b5fb675b0127e8e69a10c59))
    - Need to set some dependency versions... ([`2495107`](https://github.com/denehoffman/rustitude/commit/2495107c77f483b84a926090cd868ccec8296052))
    - Release rustitude-core v1.0.0, rustitude-gluex v0.2.0, rustitude v0.4.0, safety bump 2 crates ([`23a8807`](https://github.com/denehoffman/rustitude/commit/23a880783702368ee873ce4839310f4b392c6862))
    - Update links for rustitude-gluex readme ([`93dc0d6`](https://github.com/denehoffman/rustitude/commit/93dc0d6cfeb57f655c81f30c1e55b1c6b0460ccc))
    - Update links in readmes ([`95f85ed`](https://github.com/denehoffman/rustitude/commit/95f85ed6b16400c882e7535c7fa113ead9876353))
    - Merge rustitude-core to crates subdirectory ([`9617a27`](https://github.com/denehoffman/rustitude/commit/9617a27322460b378fb022ef28561f31197fc86f))
</details>

## v0.1.5 (2024-05-06)

<csr-id-78c524baa32d27ed65657d28ce1ef0eacde3ed0b/>

### Documentation

 - <csr-id-c8c587aa724423bd901b2b194b40446105dca3d0/> Update README.md

### Bug Fixes

 - <csr-id-9d41bce7308c9839e1169c07365229c17be557d8/> minor updates to text format
 - <csr-id-cc42f164db96559dda08a3b15d5074475625295c/> let the rustitude crate handle module inits
 - <csr-id-e5a4a49fb9b945661d8c4ab9aeb72b597f2647a4/> update to include submodule initialization

### Other

 - <csr-id-78c524baa32d27ed65657d28ce1ef0eacde3ed0b/> update Cargo.lock

## v0.1.4 (2024-05-03)

<csr-id-fb1e8f512409dd34cc197a6e76a5376d3781eac9/>

### Other

 - <csr-id-fb1e8f512409dd34cc197a6e76a5376d3781eac9/> remove unused library that was causing compilation errors further on up

## v0.1.3 (2024-05-02)

<csr-id-c91284ad855463940c849c8c11f7c43d67c35288/>
<csr-id-de2a33d7f496862d82ebf488b43e5750548afd94/>

### Chore

 - <csr-id-c91284ad855463940c849c8c11f7c43d67c35288/> update rustitude-core version

### New Features

 - <csr-id-3e3619b4736c93ecd5094c4bbd150221afb17bd6/> add pyo3 bindings for all current modules and amplitudes
 - <csr-id-a94baf2dc37165d5911bb72c336fdae0355dff92/> add parsers for current util enums

### Other

 - <csr-id-de2a33d7f496862d82ebf488b43e5750548afd94/> update Cargo.toml

## v0.1.2 (2024-04-29)

<csr-id-066062486c30e0df1760aa67e7df2ff14e082e52/>
<csr-id-685288f07f38eb5b3b7d773bf989bb6a2bc034ee/>

### Chore

 - <csr-id-066062486c30e0df1760aa67e7df2ff14e082e52/> Create LICENSE

### Documentation

 - <csr-id-33b8eaf2481fb0e013fb683f3f75d64a813f6d60/> fix README.md link to documentation
 - <csr-id-45e4cf8e192492f400fa1394ca3be5fd2e8ab1ea/> fix badge links in README.md

### New Features

 - <csr-id-ab6c8883b3676c837d168cb91053d6ea9b2b1330/> implement ThreePiAnglesSchilling.cc as sdmes::ThreePiSDME
 - <csr-id-b857ee844d4c12b242d4d8239c02c9c88ea8c1b8/> implement TwoPiAngles.cc as sdmes::TwoPiSDME
 - <csr-id-83d46cfdb1fb4b4ef65f5fa3b5c2be3b30ad4532/> refactor harmonics::{ReZlm, ImZlm} and add OnePS and TwoPS
   Added Frame and Part enums to be used with all of these amplitudes to simplify requests for just the real or imaginary parts of amplitudes. Additionally, we can now specify the Frame (Helicity/GJ). Therefore, we only need a Zlm struct rather than the split structs before.
 - <csr-id-f6b4e0d0408e6be431c3947098512b51ad5a253a/> add helper methods for Wigner D-matrices

### Bug Fixes

 - <csr-id-7175f39998c4156988799cf5e599bf840aca6d8d/> switch from rustitude to rustitude_core dependency
 - <csr-id-77dec26ac644811614ff735c229c2016882c392f/> modify the way Frames work with daughter vector inputs
   This is in preparation for the ThreePiAnglesSchilling.cc implementation.
 - <csr-id-5caca39cac184296ed6eb1a3de2258fc6cf51d36/> move useful enums into utils
 - <csr-id-87b83e8bb22fb1135862cf4138f26388a2ee17bc/> make OmegaDalitz public

### Other

 - <csr-id-685288f07f38eb5b3b7d773bf989bb6a2bc034ee/> remove local dependency

## v0.1.1 (2024-04-10)

<csr-id-9899faf055e30e68db3a88e09a5064c4767f8882/>
<csr-id-c53ab0d05b5adf9a241f74626361dba127f631bc/>
<csr-id-068054ec1ced218698e2606ac513c08219f1c958/>
<csr-id-d66f984bb5a6c1fd7144655d7f2e8dffc3bb6554/>

### Chore

 - <csr-id-9899faf055e30e68db3a88e09a5064c4767f8882/> update rustitude version

### Documentation

 - <csr-id-2d97b61e6f66d215585789793e94707b85454e73/> Update README.md

### New Features

 - <csr-id-bc6efb37bd59699cde73c18427c998080dc85791/> reorganize modules and add some new Amplitudes
   Add Rho K-Matrix, Omega-Dalitz, and Breit-Wigner, move some K-Matrix functions to utils, add placeholder module for SDMEs

### Bug Fixes

 - <csr-id-042cfeb1d371903a3a366ea4859ba069ad3c7f0f/> reorganization and local versioning on rustitude

### Other

 - <csr-id-c53ab0d05b5adf9a241f74626361dba127f631bc/> Create rust.yml

### Style

 - <csr-id-068054ec1ced218698e2606ac513c08219f1c958/> update logo
 - <csr-id-d66f984bb5a6c1fd7144655d7f2e8dffc3bb6554/> remove now-redundant imports of Complex64

