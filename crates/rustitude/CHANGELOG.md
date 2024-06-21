# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.4](https://github.com/denehoffman/rustitude/compare/rustitude-v0.7.3...rustitude-v0.7.4) - 2024-06-21

### Other
- Merge branch 'main' into development
- reduce number of samples in read_dataset benchmark

## [0.7.2](https://github.com/denehoffman/rustitude/compare/rustitude-v0.7.1...rustitude-v0.7.2) - 2024-06-19

### Other
- deprecate norm_int methods

## [0.7.1](https://github.com/denehoffman/rustitude/compare/rustitude-v0.7.0...rustitude-v0.7.1) - 2024-06-17

### Added
- add basic dataset read benchmark

## [0.7.0](https://github.com/denehoffman/rustitude/compare/rustitude-v0.6.0...rustitude-v0.7.0) - 2024-06-10

### Added
- [**breaking**] Restructures AmpOp into concrete types
- add par_ versions for all compute and norm_int methods and refactor python accordingly. Also remove RwLocks and extra allocations in the Amplitude struct, which is a huge speedup

### Other
- bump versions because smart-release just doesn't know how to do anything I guess

## 0.5.0 (2024-05-24)

<csr-id-a306fc4c47acc701aae32104ea2e017d2a4f97cc/>

### New Features

 - <csr-id-6955773a3e96890efa4573d5c7bc355bb23a07e6/> add RustitudeError and remove unwraps, error handling should work in python as well
 - <csr-id-b495892674a3fad4937de6e830241230b52880c3/> update benchmark and itertools dep, which deprecated group_by -> chunk_by
 - <csr-id-8b1eee79d545caaca39b8b38be1f00f7762adcda/> make Model::new public to allow for proper benchmarking, and add divan benchmark for basic kmatrix

### Bug Fixes

 - <csr-id-6a67ab1c8b98ea21a00a3098dfcf150ae6baa228/> switch from divan to criterion because I don't trust the threaded benchmarks
 - <csr-id-36236e54eb26dbe0015482feb60e65ce4e0cb698/> didn't need an extra pyproject.toml
 - <csr-id-405ed0cb2ad417ccf0041a49ce6fbd6b6693539c/> bump versions

### Test

 - <csr-id-a306fc4c47acc701aae32104ea2e017d2a4f97cc/> add convert script and update benchmark to use a standardized test file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 8 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump versions ([`405ed0c`](https://github.com/denehoffman/rustitude/commit/405ed0cb2ad417ccf0041a49ce6fbd6b6693539c))
    - Merge pull request #3 from denehoffman/python_reorg ([`467caac`](https://github.com/denehoffman/rustitude/commit/467caacc688f94c074d28be1ec54a422d8d9ebc0))
    - Add RustitudeError and remove unwraps, error handling should work in python as well ([`6955773`](https://github.com/denehoffman/rustitude/commit/6955773a3e96890efa4573d5c7bc355bb23a07e6))
    - Add convert script and update benchmark to use a standardized test file ([`a306fc4`](https://github.com/denehoffman/rustitude/commit/a306fc4c47acc701aae32104ea2e017d2a4f97cc))
    - Merge pull request #2 from denehoffman/create_benchmarks ([`1297b92`](https://github.com/denehoffman/rustitude/commit/1297b9275dfac49f2591573b46438ff588a77c51))
    - Switch from divan to criterion because I don't trust the threaded benchmarks ([`6a67ab1`](https://github.com/denehoffman/rustitude/commit/6a67ab1c8b98ea21a00a3098dfcf150ae6baa228))
    - Didn't need an extra pyproject.toml ([`36236e5`](https://github.com/denehoffman/rustitude/commit/36236e54eb26dbe0015482feb60e65ce4e0cb698))
    - Update benchmark and itertools dep, which deprecated group_by -> chunk_by ([`b495892`](https://github.com/denehoffman/rustitude/commit/b495892674a3fad4937de6e830241230b52880c3))
    - Make Model::new public to allow for proper benchmarking, and add divan benchmark for basic kmatrix ([`8b1eee7`](https://github.com/denehoffman/rustitude/commit/8b1eee79d545caaca39b8b38be1f00f7762adcda))
</details>

## 0.4.3 (2024-05-16)

<csr-id-e4399efe07ea8e66d9769a067c55c72d9dc09c69/>
<csr-id-0e94ec45850cb6129924b2be27793a17c51b03c2/>

### Refactor

 - <csr-id-e4399efe07ea8e66d9769a067c55c72d9dc09c69/> this is no longer a python crate

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
    - This is no longer a python crate ([`e4399ef`](https://github.com/denehoffman/rustitude/commit/e4399efe07ea8e66d9769a067c55c72d9dc09c69))
</details>

## 0.4.2 (2024-05-15)

<csr-id-9089c84e481124ff764b24f42507ab14913fef07/>

### Refactor

 - <csr-id-9089c84e481124ff764b24f42507ab14913fef07/> major move required to have rustitude function properly as a Rust crate. I set this up very wrong the first time

### Bug Fixes

 - <csr-id-174a6f540fa8f2b0292a9657e87acbe65edcaf71/> readmes and licenses suck

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude v0.4.2 ([`6fde79a`](https://github.com/denehoffman/rustitude/commit/6fde79a5b11d20069c36d47a272aee9bed9fd3a9))
    - Release rustitude v0.4.2 ([`f0d5b95`](https://github.com/denehoffman/rustitude/commit/f0d5b954f659e5410fc504b460e980a1fb270936))
    - Readmes and licenses suck ([`174a6f5`](https://github.com/denehoffman/rustitude/commit/174a6f540fa8f2b0292a9657e87acbe65edcaf71))
    - Release rustitude-gluex v0.2.1, rustitude v0.4.2 ([`3a45077`](https://github.com/denehoffman/rustitude/commit/3a45077dcd7413ac50e5ec45dc98826b11d789fb))
    - Major move required to have rustitude function properly as a Rust crate. I set this up very wrong the first time ([`9089c84`](https://github.com/denehoffman/rustitude/commit/9089c84e481124ff764b24f42507ab14913fef07))
</details>

## 0.4.1 (2024-05-15)

<csr-id-9617a27322460b378fb022ef28561f31197fc86f/>
<csr-id-64ec5097cc99eb9bb6d73376e6d3b2788f637d9d/>
<csr-id-78b96b94097670af64886abb84ed263048e91e62/>
<csr-id-8f2f28c972c20c0b8cef2869ab08fc4abaec5cf7/>
<csr-id-097311224630f5a4d98381a11d2917ca6378ad46/>

### Bug Fixes

 - <csr-id-740a0186ae22bdab87f514a5e035f3917a531c86/> add package info into workspace

### Other

 - <csr-id-9617a27322460b378fb022ef28561f31197fc86f/> merge rustitude-core to crates subdirectory
 - <csr-id-64ec5097cc99eb9bb6d73376e6d3b2788f637d9d/> more Cargo.lock and readme updates
 - <csr-id-78b96b94097670af64886abb84ed263048e91e62/> update Cargo.tomls
 - <csr-id-8f2f28c972c20c0b8cef2869ab08fc4abaec5cf7/> move rustitude to crates subdirectory

### Refactor

 - <csr-id-097311224630f5a4d98381a11d2917ca6378ad46/> move rustitude into the crates directory and add to workspace

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude v0.4.1 ([`9f6b0c5`](https://github.com/denehoffman/rustitude/commit/9f6b0c5570050312cc7f5ed1609d37e0f6f0aaca))
    - Add package info into workspace ([`740a018`](https://github.com/denehoffman/rustitude/commit/740a0186ae22bdab87f514a5e035f3917a531c86))
    - Move rustitude into the crates directory and add to workspace ([`0973112`](https://github.com/denehoffman/rustitude/commit/097311224630f5a4d98381a11d2917ca6378ad46))
    - Merge rustitude-core to crates subdirectory ([`9617a27`](https://github.com/denehoffman/rustitude/commit/9617a27322460b378fb022ef28561f31197fc86f))
    - More Cargo.lock and readme updates ([`64ec509`](https://github.com/denehoffman/rustitude/commit/64ec5097cc99eb9bb6d73376e6d3b2788f637d9d))
    - Update Cargo.tomls ([`78b96b9`](https://github.com/denehoffman/rustitude/commit/78b96b94097670af64886abb84ed263048e91e62))
    - Move rustitude to crates subdirectory ([`8f2f28c`](https://github.com/denehoffman/rustitude/commit/8f2f28c972c20c0b8cef2869ab08fc4abaec5cf7))
</details>

## 0.4.0 (2024-05-15)

<csr-id-f39aab03b7160ba3817614170d67bfcfdb22642b/>

### Bug Fixes

 - <csr-id-b71f07c33445f310969e445e7b158bdeef726a8d/> make add_submodule public

### Other

 - <csr-id-f39aab03b7160ba3817614170d67bfcfdb22642b/> reorganize crate structure

## 0.3.4 (2024-05-06)

<csr-id-475682fc30d7dc6a817030dd754cc4fb7dd295cc/>
<csr-id-f64c86d2e21c17fe6bc5638240293a774185159a/>
<csr-id-4a88e2b13fb01de2812f91ef4d55eea6b37fe7b2/>
<csr-id-1747d5dc4a63bf47f2f5cbc479f879459e900c4c/>
<csr-id-42f29669736bb72ed4d85f4669df1a48288a2db8/>
<csr-id-d94179156007fd86b69b8efbfd2f1799d0bb71b8/>

### Chore

 - <csr-id-475682fc30d7dc6a817030dd754cc4fb7dd295cc/> bump python library version

### Documentation

 - <csr-id-6d613a1c49ce065ffa4a50df09380f41359218be/> Update README.md
 - <csr-id-48a73623f151336f198009097c028d000d3e43c5/> Update README.md
 - <csr-id-2d548c4865b90043005b2aed7977612a28dad409/> fixed some links
 - <csr-id-3dc627598be4f79ffc230f8bff813423d57491f2/> Update README.md

### New Features

 - <csr-id-6fc5c77477602403f3b892b240064de9f717d406/> add type checking and re-export rustitude-core and rustitude-gluex as their own submodules

### Bug Fixes

 - <csr-id-9735d57d039946cc7b5de57a9965c4fd01c2964f/> correct spelling

### Other

 - <csr-id-f64c86d2e21c17fe6bc5638240293a774185159a/> update Cargo.lock
 - <csr-id-4a88e2b13fb01de2812f91ef4d55eea6b37fe7b2/> re-enable tag check
 - <csr-id-1747d5dc4a63bf47f2f5cbc479f879459e900c4c/> temporarily disable tag check so we can push to pypi through an action
 - <csr-id-42f29669736bb72ed4d85f4669df1a48288a2db8/> re-enable on-push and on-PR workflow conditions
   A release only happens with a new tag
 - <csr-id-d94179156007fd86b69b8efbfd2f1799d0bb71b8/> update pyproject.toml version

## 0.3.3 (2024-05-03)

<csr-id-e0c32f773b601e2703a0803849a1a2db130e2ffc/>
<csr-id-158ddc248dc8463e08eb14b7f633952fc28abcd6/>
<csr-id-f8370544ef666930bfd5f3a1b555e34e53525b6f/>
<csr-id-db8b1a32563700203f896c0d357ed96d1144d202/>

### Chore

 - <csr-id-e0c32f773b601e2703a0803849a1a2db130e2ffc/> update pyproject.toml info

### Other

 - <csr-id-158ddc248dc8463e08eb14b7f633952fc28abcd6/> remove unused library in rustitude-gluex which prevented cross-compilation
 - <csr-id-f8370544ef666930bfd5f3a1b555e34e53525b6f/> Update maturin.yml
   don't publish on every push, need to set up more for that
 - <csr-id-db8b1a32563700203f896c0d357ed96d1144d202/> add maturin github actions

## 0.3.3-pypi (2024-05-03)

<csr-id-e0c32f773b601e2703a0803849a1a2db130e2ffc/>
<csr-id-158ddc248dc8463e08eb14b7f633952fc28abcd6/>
<csr-id-f8370544ef666930bfd5f3a1b555e34e53525b6f/>
<csr-id-db8b1a32563700203f896c0d357ed96d1144d202/>

### Chore

 - <csr-id-e0c32f773b601e2703a0803849a1a2db130e2ffc/> update pyproject.toml info

### Other

 - <csr-id-158ddc248dc8463e08eb14b7f633952fc28abcd6/> remove unused library in rustitude-gluex which prevented cross-compilation
 - <csr-id-f8370544ef666930bfd5f3a1b555e34e53525b6f/> Update maturin.yml
   don't publish on every push, need to set up more for that
 - <csr-id-db8b1a32563700203f896c0d357ed96d1144d202/> add maturin github actions

## 0.3.2 (2024-05-02)

<csr-id-023cfea357b0d6e5c12f724df32d4ed30c9f24c7/>
<csr-id-7646b89c792c0f55b8898832abe6a743a052fc7a/>
<csr-id-8bd07de2425e48f4489a59ce4c168eaa9df9cc42/>
<csr-id-aaa07cd742e03461449269e8261e7f326600b2a0/>
<csr-id-e2cb6e54946744299506a39a5d3559ee099378fb/>
<csr-id-310f89c2a2da584beabad3e208484be186e8f7fd/>

### Chore

 - <csr-id-023cfea357b0d6e5c12f724df32d4ed30c9f24c7/> bump rustitude dependency versions

### New Features

 - <csr-id-157c8648dbcc1a6111d8c262a31139990ab09f3b/> initial commit to rustitude meta-crate
   Includes some preliminary PyO3 bindings for the rustitude-core crate as well as rustitude-gluex bindings for testing. The gluex submodule will eventually be behind a feature gate!

### Other

 - <csr-id-7646b89c792c0f55b8898832abe6a743a052fc7a/> Create rust.yml
 - <csr-id-8bd07de2425e48f4489a59ce4c168eaa9df9cc42/> add README and CHANGELOG

### Reverted

 - <csr-id-3126b7a26b835ee24d112883ca540c172d97dd82/> opt for implementing all pyo3 bindings in their own submodules - this package will be very lightweight!

### Style

 - <csr-id-aaa07cd742e03461449269e8261e7f326600b2a0/> add local notes
 - <csr-id-e2cb6e54946744299506a39a5d3559ee099378fb/> Create LICENSE
 - <csr-id-310f89c2a2da584beabad3e208484be186e8f7fd/> update .gitignore

## 0.3.1 (2024-04-10)

<csr-id-35fd81ade394522801d288e4a2d084b581d5e5a5/>
<csr-id-1d91ee9b2928b9761b0568104ea5e8b7841bf24c/>
<csr-id-73067f2c6f657ba3b35a197a0ab2ea8029e359e6/>
<csr-id-55443f6454e0072abc3cfc41d38e5fa297cdb9cd/>
<csr-id-aa9d91971816ff3d99a47e928be5bfb2360c0694/>
<csr-id-042cee3a55e95567058401a72260911fcffccc0b/>

### Chore

 - <csr-id-35fd81ade394522801d288e4a2d084b581d5e5a5/> update Cargo.lock

### Documentation

 - <csr-id-32fb8351ee7fa4d1b8883391f3360a63a643497d/> update README.md
 - <csr-id-b9c973034a53153de41370591b64d7f8817e87a3/> update README.md
 - <csr-id-e0509c98ab2389991b78980b7df01d4bf2ee5369/> document ParameterType
 - <csr-id-fcd1fc0584eeccdac26571b1422212e2ef2dee4e/> document Scalar and ComplexScalar
 - <csr-id-2793683c3820e127efa817fb324ec78353ba7064/> document Amplitude
 - <csr-id-182ac516a990ee2c9fccebcc253397c0bcf6db8e/> document Node
 - <csr-id-54197fc39b455db3b10f820ac09185f11eb12588/> document macros

### New Features

 - <csr-id-bf655e45feab393115b52ac711d4e9d3d487e799/> add pretty printing to Parameter struct
 - <csr-id-060bd61296a4311ab3fd57f91ddf9a66187a9e7f/> formalize Parameter struct
 - <csr-id-15f3afe9a97a1715a67095d5dcd2b8ea5fbd8e07/> rework loading from parquet and different ways to load EPS
 - <csr-id-ae0f81f9e12236ecfb0755522773b2c18d2874ba/> modify output of Manage::parameters to include other parameter fields
 - <csr-id-8c9cf902b85584098e754f02381ac3f3735170e6/> re-export num_complex::Complex64

### Bug Fixes

 - <csr-id-f5d1b75e8c0d97f9c2090afb6778f31bd0d61804/> make Manager::compute public again
 - <csr-id-69b3dce41f3aedcb07c6f5f56e4529086bba163d/> fix tests
 - <csr-id-5307dc79d26eaf38692e558f72a5f9185d78db68/> reorganize amplitude module into amplitude and manager restructure manager via Manage trait
 - <csr-id-6cb74a26eccd5311bd9c42461b4a8340e55acf0c/> remove nightly-only feature which was unused
 - <csr-id-db41c39e93443701f2a45bda806bfd438a6ba141/> doctest

### Other

 - <csr-id-1d91ee9b2928b9761b0568104ea5e8b7841bf24c/> update .gitignore
 - <csr-id-73067f2c6f657ba3b35a197a0ab2ea8029e359e6/> get rid of expect, gluex
 - <csr-id-55443f6454e0072abc3cfc41d38e5fa297cdb9cd/> some updates to bacon and other configs
 - <csr-id-aa9d91971816ff3d99a47e928be5bfb2360c0694/> first commit of functional code
   This really doesn't work well at all, it is super slow and hoards memory

### Style

 - <csr-id-042cee3a55e95567058401a72260911fcffccc0b/> remove line from docs-header.html

## 0.3.0 (2024-04-05)

## 0.2.0-alpha (2024-02-02)

<csr-id-be408f129f003ec8ec273cc2a7e89480c743b525/>
<csr-id-c57b8ccb558f61bae4739b0e22604856807d741a/>
<csr-id-ffb02531cc77f25bc9bdf02b19df97eb7b82b28b/>
<csr-id-bf6d0714c8969e3e82f3479778ae033b7a1cfd8b/>
<csr-id-45274b24fb5cd7d596585909c868ae6223af3824/>
<csr-id-7e561e9a5bfebcf901c0f460448f9f68c286757f/>
<csr-id-e64d69a70e00537cd059b0e8cddfbe6c9ebc656b/>

### Chore

 - <csr-id-be408f129f003ec8ec273cc2a7e89480c743b525/> Release rustitude version 0.2.0-alpha

### New Features

 - <csr-id-67279a800722f3bf90754502126f5a387f0dabd8/> re-implement dependency resolution

### Other

 - <csr-id-c57b8ccb558f61bae4739b0e22604856807d741a/> add Resolver trait and some other minor organizational things

### Refactor

 - <csr-id-ffb02531cc77f25bc9bdf02b19df97eb7b82b28b/> change par_evaluate_on to evaluate_on_par
   This matches other methods and makes the API more consistent. It also
   makes it easier to search for the parallel version if you already know
   the non-parallel version by typing completion.
 - <csr-id-bf6d0714c8969e3e82f3479778ae033b7a1cfd8b/> changed resolve to function to match amplitude
   plus other aesthetic changes
 - <csr-id-45274b24fb5cd7d596585909c868ae6223af3824/> major rewrite of Dataset and Variable types
   While this is technically a breaking change, I'm still working in 0.1.* here. The core of this is that Dataset entries are no longer enums, which have the size of their largest variant (in our case, complex matrices probably). This is bad, as it wastes memory and takes longer to move around. While I still haven't tested this, I believe my solution is better, and as far as I can tell, it makes a lot of the code cleaner. It did require a rewrite of Variable, since the return type is no longer an enum, so there are now multiple Variable variants, each with Builders. I also added a Builder to the Amplitude struct, and renamed the traits Amplitude(Variable)Builder to IntoAmplitude(Variable) to avoid confusion. I disabled most of the gluex code while I'm working through the changes, but Ylm is implemented and might work, although I haven't tested it at time of writing. Assume tests will break here.

### Style

 - <csr-id-7e561e9a5bfebcf901c0f460448f9f68c286757f/> give logo a white background
 - <csr-id-e64d69a70e00537cd059b0e8cddfbe6c9ebc656b/> add logo

## 0.1.3 (2023-12-29)

<csr-id-2c0a933b1e2861987b172cdfc81a479ced68792c/>
<csr-id-3442057a56351b0fb3a5c53fb89df242c36a4c66/>
<csr-id-e85e1ca1fb81476ac90249b57bcdc60e22881d9a/>
<csr-id-f0b655bac628665bf7f1a479f3e941d0280407ae/>

### New Features

 - <csr-id-545f263a714e9d8ed7fc91c7250af97275fe9738/> add Pow trait and pow function to Amplitude
 - <csr-id-0dff19617e8264c61a9c1569b06a56797c4f55d3/> allow for different amplitudes for data and MC
   This ensures we can assign weights as part of the amplitude, but users
   can choose whether they want weighted MC or not. Also makes it easy if
   your branch names differ between the two files, you only have to
   re-implement some things.
 - <csr-id-676daf37764d153e4d7c4898c9784b3243814f2b/> add Branch struct
   Branch is a convenience wrapper for getting data from the Dataset
   without duplicating or copying anything into a new variable dependency.

### Bug Fixes

 - <csr-id-2f5de7f864a35d38f9c6d612a4e3db5354b4c2fe/> doctests were taking Strings after I changed to &str

### Refactor

 - <csr-id-2c0a933b1e2861987b172cdfc81a479ced68792c/> move data extraction into dataset and propogate errors
 - <csr-id-3442057a56351b0fb3a5c53fb89df242c36a4c66/> change inputs to functions to &str

### Style

 - <csr-id-e85e1ca1fb81476ac90249b57bcdc60e22881d9a/> try out logging fit steps
 - <csr-id-f0b655bac628665bf7f1a479f3e941d0280407ae/> remove some commented-out code

## 0.1.2-beta.2 (2023-12-29)

<csr-id-8d596bf94049e0cd4327902bf63b9ae240c51a13/>

### Chore

 - <csr-id-8d596bf94049e0cd4327902bf63b9ae240c51a13/> release

## 0.1.2-beta.1 (2023-12-29)

<csr-id-0b5ace3f3d7f9c2549e2011a488bbd35d55290d9/>

### Chore

 - <csr-id-0b5ace3f3d7f9c2549e2011a488bbd35d55290d9/> Release rustitude version 0.1.2-beta.1

