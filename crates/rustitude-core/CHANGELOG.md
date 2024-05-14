# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.4 (2024-05-06)

### Documentation

 - <csr-id-54a9908a5fb203b828787ae8ca21d628a8da485b/> Update README.md
 - <csr-id-3d22a8634bc81d6cb21049d60ada5836a7d03678/> Update README.md

### New Features

 - <csr-id-f05877659fada6fa62d5307a352b4c8af781460e/> add Piecewise to prelude
 - <csr-id-effffcfa708cdd7267c6962bebc769064a979b82/> add Piecewise amplitude
   Additionally adds a method to use the piecewise amplitude over resonance mass: PiecewiseM
 - <csr-id-8675299aa0bc0dc0ea491f05ab8cc6ad22d8bf38/> add assert for compute to check if the proper number of parameters is inputed
 - <csr-id-1ae2cbcc4aa3a9a73572a0bc1e07065c198641dc/> add fixed and constrained flags to parameters
 - <csr-id-da32d6311d37c3f4ffbd0a02726e83efec8f25cc/> move Dataset constructors into pymethods, add options for no weights or eps in from_dict

### Bug Fixes

 - <csr-id-d793c7cb84c7e586cef301a9310c8d931827986e/> correct spelling
 - <csr-id-6ec195da90bb9b871b3bf3c4048b88aac0175eac/> add list of used indices to hopefully ensure constrained parameters are only returned by parameters if the flag is true
 - <csr-id-ea15b3280e198ef38b68a53d8499618cfd43850d/> set indices rather than setting parameters, this fixes a kind of major bug where constraining just made duplicates
 - <csr-id-1b2765a7e3c85744bab7472555b9b0348d2a7e3d/> change module implementation
 - <csr-id-1f0ad57bbf5199d6923755f811744472947340b1/> make sure submodules are called
 - <csr-id-5c47e5cf301a5d6c58a359d0b4afaead678de2b5/> make amplitude, four_momentum, and manager #[pymodule]s
 - <csr-id-b9c6d51bb54d8f962a6d8e7507e42c9676f84a99/> make dataset a #[pymodule], hopefully this resolves import issues

### Other

 - <csr-id-2137d6ed155df261e70f9905c6400a9fdcfade35/> update Cargo.lock

### Style

 - <csr-id-ce7f083ef0d38e53d6b2ebfe8382c9ed52146713/> change nbins to bins to match numpy and matplotlib

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update Cargo.lock ([`2137d6e`](https://github.com/denehoffman/rustitude-core/commit/2137d6ed155df261e70f9905c6400a9fdcfade35))
    - Add Piecewise to prelude ([`f058776`](https://github.com/denehoffman/rustitude-core/commit/f05877659fada6fa62d5307a352b4c8af781460e))
    - Correct spelling ([`d793c7c`](https://github.com/denehoffman/rustitude-core/commit/d793c7cb84c7e586cef301a9310c8d931827986e))
    - Add Piecewise amplitude ([`effffcf`](https://github.com/denehoffman/rustitude-core/commit/effffcfa708cdd7267c6962bebc769064a979b82))
    - Update README.md ([`54a9908`](https://github.com/denehoffman/rustitude-core/commit/54a9908a5fb203b828787ae8ca21d628a8da485b))
    - Update README.md ([`3d22a86`](https://github.com/denehoffman/rustitude-core/commit/3d22a8634bc81d6cb21049d60ada5836a7d03678))
    - Add assert for compute to check if the proper number of parameters is inputed ([`8675299`](https://github.com/denehoffman/rustitude-core/commit/8675299aa0bc0dc0ea491f05ab8cc6ad22d8bf38))
    - Add list of used indices to hopefully ensure constrained parameters are only returned by parameters if the flag is true ([`6ec195d`](https://github.com/denehoffman/rustitude-core/commit/6ec195da90bb9b871b3bf3c4048b88aac0175eac))
    - Add fixed and constrained flags to parameters ([`1ae2cbc`](https://github.com/denehoffman/rustitude-core/commit/1ae2cbcc4aa3a9a73572a0bc1e07065c198641dc))
    - Set indices rather than setting parameters, this fixes a kind of major bug where constraining just made duplicates ([`ea15b32`](https://github.com/denehoffman/rustitude-core/commit/ea15b3280e198ef38b68a53d8499618cfd43850d))
    - Change nbins to bins to match numpy and matplotlib ([`ce7f083`](https://github.com/denehoffman/rustitude-core/commit/ce7f083ef0d38e53d6b2ebfe8382c9ed52146713))
    - Move Dataset constructors into pymethods, add options for no weights or eps in from_dict ([`da32d63`](https://github.com/denehoffman/rustitude-core/commit/da32d6311d37c3f4ffbd0a02726e83efec8f25cc))
    - Change module implementation ([`1b2765a`](https://github.com/denehoffman/rustitude-core/commit/1b2765a7e3c85744bab7472555b9b0348d2a7e3d))
    - Make sure submodules are called ([`1f0ad57`](https://github.com/denehoffman/rustitude-core/commit/1f0ad57bbf5199d6923755f811744472947340b1))
    - Make amplitude, four_momentum, and manager #[pymodule]s ([`5c47e5c`](https://github.com/denehoffman/rustitude-core/commit/5c47e5cf301a5d6c58a359d0b4afaead678de2b5))
    - Make dataset a #[pymodule], hopefully this resolves import issues ([`b9c6d51`](https://github.com/denehoffman/rustitude-core/commit/b9c6d51bb54d8f962a6d8e7507e42c9676f84a99))
</details>

## 0.3.3 (2024-05-02)

<csr-id-a257aace868fb14b062eb9ac2b502a1416c7cab6/>

### Chore

 - <csr-id-a257aace868fb14b062eb9ac2b502a1416c7cab6/> Update README.md

### New Features

 - <csr-id-baf71d09652b2140400013f821cfe40bfa7ef73d/> add pyo3 bindings to all main modules
 - <csr-id-ceac972d1235d434d5f36ef83402e772f3e9fbc5/> implement Clone for Event
 - <csr-id-fa7eea606e211784de37d6f43f6d24fac21cae35/> some Clone derives for Manager and additional API access for Parameter

### Bug Fixes

 - <csr-id-be2c1f1bc9928d1674120260ced3cb2d4b107370/> correct the module name for amplitude from dataset (typo)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude-core v0.3.3 ([`f35d785`](https://github.com/denehoffman/rustitude-core/commit/f35d785ac58b4266a308b8c9ea85f57ce476bf43))
    - Correct the module name for amplitude from dataset (typo) ([`be2c1f1`](https://github.com/denehoffman/rustitude-core/commit/be2c1f1bc9928d1674120260ced3cb2d4b107370))
    - Add pyo3 bindings to all main modules ([`baf71d0`](https://github.com/denehoffman/rustitude-core/commit/baf71d09652b2140400013f821cfe40bfa7ef73d))
    - Implement Clone for Event ([`ceac972`](https://github.com/denehoffman/rustitude-core/commit/ceac972d1235d434d5f36ef83402e772f3e9fbc5))
    - Merge branch 'main' of github.com:denehoffman/rustitude-core ([`4eacf02`](https://github.com/denehoffman/rustitude-core/commit/4eacf02e4137d6a3d2318729c9b25d0c1a1b2d92))
    - Some Clone derives for Manager and additional API access for Parameter ([`fa7eea6`](https://github.com/denehoffman/rustitude-core/commit/fa7eea606e211784de37d6f43f6d24fac21cae35))
    - Update README.md ([`a257aac`](https://github.com/denehoffman/rustitude-core/commit/a257aace868fb14b062eb9ac2b502a1416c7cab6))
</details>

## 0.3.2 (2024-04-29)

<csr-id-8d9ec2e806f7d6f47413377e7d1ff92fa8819802/>

### Other

 - <csr-id-8d9ec2e806f7d6f47413377e7d1ff92fa8819802/> version bump

### Style

 - <csr-id-ba2e0b7750188d5d837ea32d03a4bdbea157819d/> added AmplitudeID and ParameterID types to simplify Manage functions

### Reverted

 - <csr-id-dd8e9a557f461c3403f757161e5b81ba724ec5d9/> remove unused packages and files, remove pyo3 bindings for future crate
 - <csr-id-7159f771aea4932e4f24edaace03b53fa4ceebbe/> rename crate to rustitude-core in preparation for restructuring
 - <csr-id-df1084e43f987c985a38d0558202d2089b18da3e/> move ArcLocks into Amplitude to enable better PyO3 integration, this also means less &mut selfs and no need for macros for scalar-like Amplitudes
 - <csr-id-b833b07a63f99f048832fa9b2f18176d830c00dc/> remove some iter methods from Dataset and make it store an Arc to allow Manager to have no lifetime annotations
 - <csr-id-d915bbbfb88d52cb03ea8d89120105f197e0288d/> remove gomez support

### Bug Fixes

 - <csr-id-ad26a016702f345cfca364ec1410ff1d9b218cd8/> fix docs
 - <csr-id-a1627283a98fdbdb2cd70cad5428d6ec02119ef9/> fix more tests
 - <csr-id-5f3e4f4a2b18bcb315de99918ae0d5b7d8420ab9/> fix tests with renamed lib
 - <csr-id-783f6eda92fc4ce180e1d8081594394abbcad6f1/> forgot to add required crates for ROOT file reading
 - <csr-id-47c3d0886f86f2ae8f6b652511181cf835038330/> ensured convert gets the correct tree name for writing
 - <csr-id-b15712bdbb77222f78b55039bb3f4c3b278e986f/> fix doctests in amplitude.rs
 - <csr-id-d33c869c8167711453d4072535a711442099c556/> parking_lot::RwLocks don't need to be unwrapped

### New Features

 - <csr-id-2b950bbd0fa6e28365dd6fc6df7236d60c857347/> add pyo3 bindings to four_momentum
 - <csr-id-fe918b06885d84c71c55038f9b7667edb238e643/> add bounds and initial values to parameters
 - <csr-id-ddb3c08b8c0ea41c80cb5ab3cd647aeb0ff4e0b7/> add prelim ROOT file reading and ways to split datasets
 - <csr-id-319bac9d4b70a18ca6fd9b43bc4f8d48598d21d9/> add bounds to Problem impl for Managers
 - <csr-id-8ae63ca712d3c8ee8954035de09b549e44c0c6bd/> add bounds and initial values to Parameters
 - <csr-id-294c5e4c66694a39f5b2a6d90d68d2acedd17208/> switch to parking_lot::RwLock
 - <csr-id-31de0d8595fd3d1c94f69dd6b2370ac09a731f44/> add amplitude::PolarComplexScalar

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 19 calendar days.
 - 19 days passed between releases.
 - 21 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude-core v0.3.2 ([`4f5bbd5`](https://github.com/denehoffman/rustitude-core/commit/4f5bbd5d259affd3d3e85218ec76bd91a76ee479))
    - Version bump ([`8d9ec2e`](https://github.com/denehoffman/rustitude-core/commit/8d9ec2e806f7d6f47413377e7d1ff92fa8819802))
    - Release rustitude-core v0.3.1 ([`1b6436c`](https://github.com/denehoffman/rustitude-core/commit/1b6436c869266a7cb19b9f98699a11bd353f12d8))
    - Fix docs ([`ad26a01`](https://github.com/denehoffman/rustitude-core/commit/ad26a016702f345cfca364ec1410ff1d9b218cd8))
    - Fix more tests ([`a162728`](https://github.com/denehoffman/rustitude-core/commit/a1627283a98fdbdb2cd70cad5428d6ec02119ef9))
    - Fix tests with renamed lib ([`5f3e4f4`](https://github.com/denehoffman/rustitude-core/commit/5f3e4f4a2b18bcb315de99918ae0d5b7d8420ab9))
    - Merge pull request #5 from denehoffman/pyo3_bindings ([`5f6f246`](https://github.com/denehoffman/rustitude-core/commit/5f6f24627e60dd35861049d6f7ef47c634830caa))
    - Remove unused packages and files, remove pyo3 bindings for future crate ([`dd8e9a5`](https://github.com/denehoffman/rustitude-core/commit/dd8e9a557f461c3403f757161e5b81ba724ec5d9))
    - Rename crate to rustitude-core in preparation for restructuring ([`7159f77`](https://github.com/denehoffman/rustitude-core/commit/7159f771aea4932e4f24edaace03b53fa4ceebbe))
    - Move ArcLocks into Amplitude to enable better PyO3 integration, this also means less &mut selfs and no need for macros for scalar-like Amplitudes ([`df1084e`](https://github.com/denehoffman/rustitude-core/commit/df1084e43f987c985a38d0558202d2089b18da3e))
    - Add pyo3 bindings to four_momentum ([`2b950bb`](https://github.com/denehoffman/rustitude-core/commit/2b950bbd0fa6e28365dd6fc6df7236d60c857347))
    - Remove some iter methods from Dataset and make it store an Arc to allow Manager to have no lifetime annotations ([`b833b07`](https://github.com/denehoffman/rustitude-core/commit/b833b07a63f99f048832fa9b2f18176d830c00dc))
    - Remove gomez support ([`d915bbb`](https://github.com/denehoffman/rustitude-core/commit/d915bbbfb88d52cb03ea8d89120105f197e0288d))
    - Forgot to add required crates for ROOT file reading ([`783f6ed`](https://github.com/denehoffman/rustitude-core/commit/783f6eda92fc4ce180e1d8081594394abbcad6f1))
    - Add bounds and initial values to parameters ([`fe918b0`](https://github.com/denehoffman/rustitude-core/commit/fe918b06885d84c71c55038f9b7667edb238e643))
    - Add prelim ROOT file reading and ways to split datasets ([`ddb3c08`](https://github.com/denehoffman/rustitude-core/commit/ddb3c08b8c0ea41c80cb5ab3cd647aeb0ff4e0b7))
    - Ensured convert gets the correct tree name for writing ([`47c3d08`](https://github.com/denehoffman/rustitude-core/commit/47c3d0886f86f2ae8f6b652511181cf835038330))
    - Update convert to clean up .root files by default ([`25d7998`](https://github.com/denehoffman/rustitude-core/commit/25d7998aa17dac1e4a66096b7b2cb2ec86e43f0d))
    - Fix doctests in amplitude.rs ([`b15712b`](https://github.com/denehoffman/rustitude-core/commit/b15712bdbb77222f78b55039bb3f4c3b278e986f))
    - Add bounds to Problem impl for Managers ([`319bac9`](https://github.com/denehoffman/rustitude-core/commit/319bac9d4b70a18ca6fd9b43bc4f8d48598d21d9))
    - Added AmplitudeID and ParameterID types to simplify Manage functions ([`ba2e0b7`](https://github.com/denehoffman/rustitude-core/commit/ba2e0b7750188d5d837ea32d03a4bdbea157819d))
    - Add bounds and initial values to Parameters ([`8ae63ca`](https://github.com/denehoffman/rustitude-core/commit/8ae63ca712d3c8ee8954035de09b549e44c0c6bd))
    - Parking_lot::RwLocks don't need to be unwrapped ([`d33c869`](https://github.com/denehoffman/rustitude-core/commit/d33c869c8167711453d4072535a711442099c556))
    - Switch to parking_lot::RwLock ([`294c5e4`](https://github.com/denehoffman/rustitude-core/commit/294c5e4c66694a39f5b2a6d90d68d2acedd17208))
    - Add amplitude::PolarComplexScalar ([`31de0d8`](https://github.com/denehoffman/rustitude-core/commit/31de0d8595fd3d1c94f69dd6b2370ac09a731f44))
</details>

## 0.3.1 (2024-04-10)

<csr-id-35fd81ade394522801d288e4a2d084b581d5e5a5/>
<csr-id-1d91ee9b2928b9761b0568104ea5e8b7841bf24c/>
<csr-id-73067f2c6f657ba3b35a197a0ab2ea8029e359e6/>
<csr-id-55443f6454e0072abc3cfc41d38e5fa297cdb9cd/>
<csr-id-aa9d91971816ff3d99a47e928be5bfb2360c0694/>
<csr-id-042cee3a55e95567058401a72260911fcffccc0b/>
<csr-id-ba2e0b7750188d5d837ea32d03a4bdbea157819d/>

### Chore

 - <csr-id-35fd81ade394522801d288e4a2d084b581d5e5a5/> update Cargo.lock

### Style

 - <csr-id-ba2e0b7750188d5d837ea32d03a4bdbea157819d/> added AmplitudeID and ParameterID types to simplify Manage functions

### Reverted

 - <csr-id-dd8e9a557f461c3403f757161e5b81ba724ec5d9/> remove unused packages and files, remove pyo3 bindings for future crate
 - <csr-id-7159f771aea4932e4f24edaace03b53fa4ceebbe/> rename crate to rustitude-core in preparation for restructuring
 - <csr-id-df1084e43f987c985a38d0558202d2089b18da3e/> move ArcLocks into Amplitude to enable better PyO3 integration, this also means less &mut selfs and no need for macros for scalar-like Amplitudes
 - <csr-id-b833b07a63f99f048832fa9b2f18176d830c00dc/> remove some iter methods from Dataset and make it store an Arc to allow Manager to have no lifetime annotations
 - <csr-id-d915bbbfb88d52cb03ea8d89120105f197e0288d/> remove gomez support

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
 - <csr-id-2b950bbd0fa6e28365dd6fc6df7236d60c857347/> add pyo3 bindings to four_momentum
 - <csr-id-fe918b06885d84c71c55038f9b7667edb238e643/> add bounds and initial values to parameters
 - <csr-id-ddb3c08b8c0ea41c80cb5ab3cd647aeb0ff4e0b7/> add prelim ROOT file reading and ways to split datasets
 - <csr-id-319bac9d4b70a18ca6fd9b43bc4f8d48598d21d9/> add bounds to Problem impl for Managers
 - <csr-id-8ae63ca712d3c8ee8954035de09b549e44c0c6bd/> add bounds and initial values to Parameters
 - <csr-id-294c5e4c66694a39f5b2a6d90d68d2acedd17208/> switch to parking_lot::RwLock
 - <csr-id-31de0d8595fd3d1c94f69dd6b2370ac09a731f44/> add amplitude::PolarComplexScalar

### Bug Fixes

 - <csr-id-f5d1b75e8c0d97f9c2090afb6778f31bd0d61804/> make Manager::compute public again
 - <csr-id-69b3dce41f3aedcb07c6f5f56e4529086bba163d/> fix tests
 - <csr-id-5307dc79d26eaf38692e558f72a5f9185d78db68/> reorganize amplitude module into amplitude and manager restructure manager via Manage trait
 - <csr-id-6cb74a26eccd5311bd9c42461b4a8340e55acf0c/> remove nightly-only feature which was unused
 - <csr-id-db41c39e93443701f2a45bda806bfd438a6ba141/> doctest
 - <csr-id-ad26a016702f345cfca364ec1410ff1d9b218cd8/> fix docs
 - <csr-id-a1627283a98fdbdb2cd70cad5428d6ec02119ef9/> fix more tests
 - <csr-id-5f3e4f4a2b18bcb315de99918ae0d5b7d8420ab9/> fix tests with renamed lib
 - <csr-id-783f6eda92fc4ce180e1d8081594394abbcad6f1/> forgot to add required crates for ROOT file reading
 - <csr-id-47c3d0886f86f2ae8f6b652511181cf835038330/> ensured convert gets the correct tree name for writing
 - <csr-id-b15712bdbb77222f78b55039bb3f4c3b278e986f/> fix doctests in amplitude.rs
 - <csr-id-d33c869c8167711453d4072535a711442099c556/> parking_lot::RwLocks don't need to be unwrapped

### Other

 - <csr-id-1d91ee9b2928b9761b0568104ea5e8b7841bf24c/> update .gitignore
 - <csr-id-73067f2c6f657ba3b35a197a0ab2ea8029e359e6/> get rid of expect, gluex
 - <csr-id-55443f6454e0072abc3cfc41d38e5fa297cdb9cd/> some updates to bacon and other configs
 - <csr-id-aa9d91971816ff3d99a47e928be5bfb2360c0694/> first commit of functional code
   This really doesn't work well at all, it is super slow and hoards memory

### Style

 - <csr-id-042cee3a55e95567058401a72260911fcffccc0b/> remove line from docs-header.html

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 62 commits contributed to the release over the course of 46 calendar days.
 - 67 days passed between releases.
 - 23 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude v0.3.1 ([`54efefe`](https://github.com/denehoffman/rustitude-core/commit/54efefe447a0d00b4da366a4fb3198662f4c6cc7))
    - Update README.md ([`32fb835`](https://github.com/denehoffman/rustitude-core/commit/32fb8351ee7fa4d1b8883391f3360a63a643497d))
    - Add pretty printing to Parameter struct ([`bf655e4`](https://github.com/denehoffman/rustitude-core/commit/bf655e45feab393115b52ac711d4e9d3d487e799))
    - Formalize Parameter struct ([`060bd61`](https://github.com/denehoffman/rustitude-core/commit/060bd61296a4311ab3fd57f91ddf9a66187a9e7f))
    - Update README.md ([`b9c9730`](https://github.com/denehoffman/rustitude-core/commit/b9c973034a53153de41370591b64d7f8817e87a3))
    - Rework loading from parquet and different ways to load EPS ([`15f3afe`](https://github.com/denehoffman/rustitude-core/commit/15f3afe9a97a1715a67095d5dcd2b8ea5fbd8e07))
    - Update README.md ([`03d7239`](https://github.com/denehoffman/rustitude-core/commit/03d7239ab40a9edc3e545e8ea9051c6507658e8f))
    - Modify output of Manage::parameters to include other parameter fields ([`ae0f81f`](https://github.com/denehoffman/rustitude-core/commit/ae0f81f9e12236ecfb0755522773b2c18d2874ba))
    - Make Manager::compute public again ([`f5d1b75`](https://github.com/denehoffman/rustitude-core/commit/f5d1b75e8c0d97f9c2090afb6778f31bd0d61804))
    - Fix tests ([`69b3dce`](https://github.com/denehoffman/rustitude-core/commit/69b3dce41f3aedcb07c6f5f56e4529086bba163d))
    - Re-export num_complex::Complex64 ([`8c9cf90`](https://github.com/denehoffman/rustitude-core/commit/8c9cf902b85584098e754f02381ac3f3735170e6))
    - Reorganize amplitude module into amplitude and manager restructure manager via Manage trait ([`5307dc7`](https://github.com/denehoffman/rustitude-core/commit/5307dc79d26eaf38692e558f72a5f9185d78db68))
    - Remove line from docs-header.html ([`042cee3`](https://github.com/denehoffman/rustitude-core/commit/042cee3a55e95567058401a72260911fcffccc0b))
    - Update .gitignore ([`1d91ee9`](https://github.com/denehoffman/rustitude-core/commit/1d91ee9b2928b9761b0568104ea5e8b7841bf24c))
    - Update Cargo.lock ([`35fd81a`](https://github.com/denehoffman/rustitude-core/commit/35fd81ade394522801d288e4a2d084b581d5e5a5))
    - Document ParameterType ([`e0509c9`](https://github.com/denehoffman/rustitude-core/commit/e0509c98ab2389991b78980b7df01d4bf2ee5369))
    - Document Scalar and ComplexScalar ([`fcd1fc0`](https://github.com/denehoffman/rustitude-core/commit/fcd1fc0584eeccdac26571b1422212e2ef2dee4e))
    - Document Amplitude ([`2793683`](https://github.com/denehoffman/rustitude-core/commit/2793683c3820e127efa817fb324ec78353ba7064))
    - Document Node ([`182ac51`](https://github.com/denehoffman/rustitude-core/commit/182ac516a990ee2c9fccebcc253397c0bcf6db8e))
    - Document macros ([`54197fc`](https://github.com/denehoffman/rustitude-core/commit/54197fc39b455db3b10f820ac09185f11eb12588))
    - Remove nightly-only feature which was unused ([`6cb74a2`](https://github.com/denehoffman/rustitude-core/commit/6cb74a26eccd5311bd9c42461b4a8340e55acf0c))
    - Create rust.yml ([`343e9cc`](https://github.com/denehoffman/rustitude-core/commit/343e9cc4e7bb408a60d7d86ae535e97bd6a5c41a))
    - Delete .github/workflows/release-plz.yml ([`a208525`](https://github.com/denehoffman/rustitude-core/commit/a208525dfb7cc79348c0eeef2c447197840a1563))
    - Reorganization ([`7f92845`](https://github.com/denehoffman/rustitude-core/commit/7f928459688704d8eab6c67e00a549321c6709b6))
    - Update rustitude-gluex Cargo.toml ([`9eca00b`](https://github.com/denehoffman/rustitude-core/commit/9eca00bec3df06136e4bf71be5d74cc560b728cc))
    - Version bump ([`666d2e6`](https://github.com/denehoffman/rustitude-core/commit/666d2e6b4bfa89f281fb0b06c06ce3e5b835e525))
    - Release-plz settings ([`62d27a6`](https://github.com/denehoffman/rustitude-core/commit/62d27a6c1fd93bd3ae41deed38d904884b1e442b))
    - Figured it out ([`3d5aaba`](https://github.com/denehoffman/rustitude-core/commit/3d5aaba517c23ff6e1b17faeb1f87e1fdda9ad8a))
    - Trying again ([`22b5493`](https://github.com/denehoffman/rustitude-core/commit/22b54936f75cba19955371094324ac18e821aebc))
    - A few more fixes ([`da05752`](https://github.com/denehoffman/rustitude-core/commit/da05752e412781f3818c38b9514f164c5aeefd86))
    - Fix version ([`1dd6433`](https://github.com/denehoffman/rustitude-core/commit/1dd643306b88115882bb6a6fe3dc51bee7747fe7))
    - Some more clean up ([`a7b0df1`](https://github.com/denehoffman/rustitude-core/commit/a7b0df194671e5bccdc078c9df2d2a290f51a702))
    - Reset version ([`2310315`](https://github.com/denehoffman/rustitude-core/commit/231031574b0a52fcc310f0a241f36e9ac9e4fc69))
    - Get rid of expect, gluex ([`73067f2`](https://github.com/denehoffman/rustitude-core/commit/73067f2c6f657ba3b35a197a0ab2ea8029e359e6))
    - Auto stash before revert of "readmes" ([`f2dc808`](https://github.com/denehoffman/rustitude-core/commit/f2dc808222b59d69d221be15bfd1029bf27f5f7a))
    - Revert "readmes" ([`04e5473`](https://github.com/denehoffman/rustitude-core/commit/04e54739064a384a4f72e87015c930931fce2b5e))
    - Readmes ([`4525d23`](https://github.com/denehoffman/rustitude-core/commit/4525d23141f022675674fa99d74bc0c25e3b56dd))
    - Fix media ([`e3320e1`](https://github.com/denehoffman/rustitude-core/commit/e3320e17ecd5c2f13ba257cdc5a4e65091a1287b))
    - Merge branch 'main' of github.com:denehoffman/rustitude ([`586abbf`](https://github.com/denehoffman/rustitude-core/commit/586abbf045ed4dc4b6aa8b84046ca983dff41818))
    - Undo bad commits ([`21b66dc`](https://github.com/denehoffman/rustitude-core/commit/21b66dcdf5c3ca0e60f7cb340a98190e3fb8b351))
    - Adjusting changelogs prior to release of rustitude v0.3.0 ([`21f3871`](https://github.com/denehoffman/rustitude-core/commit/21f38711c669aa2e8d13fb67d71bee65c82073b8))
    - Adjusting changelogs prior to release of rustitude v0.3.0 ([`0d12f1b`](https://github.com/denehoffman/rustitude-core/commit/0d12f1ba923d7c3c6da44e6299af0d0e3f8f7486))
    - Preliminary changelog ([`6a9e174`](https://github.com/denehoffman/rustitude-core/commit/6a9e17470623fe33c6ca1e297f9e5f41ccb86a61))
    - Merge pull request #4 from denehoffman/the_everything_is_a_vector_rewrite ([`c47ba17`](https://github.com/denehoffman/rustitude-core/commit/c47ba171fd517f9c08e0d6f52b5b2b4cec583cc5))
    - Get rid of benchmarks for now, move mds ([`7526439`](https://github.com/denehoffman/rustitude-core/commit/7526439994bed98a457aa67fbd223ecf93e95b26))
    - First attempt separating submodules ([`605a77e`](https://github.com/denehoffman/rustitude-core/commit/605a77e83e418b2e6658d01e4f05a62c46347012))
    - Doctest ([`db41c39`](https://github.com/denehoffman/rustitude-core/commit/db41c39e93443701f2a45bda806bfd438a6ba141))
    - More convenient API changes ([`9e247af`](https://github.com/denehoffman/rustitude-core/commit/9e247afe6fb9bc845fc00249f4ce9f35a3102960))
    - Finished main bulk of code I think ([`1ebc06d`](https://github.com/denehoffman/rustitude-core/commit/1ebc06ddbfc3141143e21af6aee3f9fe521e6d7e))
    - Move aux_data back to node level, speeds up everything ([`b85f1f5`](https://github.com/denehoffman/rustitude-core/commit/b85f1f59eef49a38c5c75132ebbaba507d232967))
    - Add F0 amplitude and fix benchmarks ([`daca025`](https://github.com/denehoffman/rustitude-core/commit/daca025a707b5b8ee350f3fe7262a964024c1c58))
    - Update the way parameters are registered ([`db587b5`](https://github.com/denehoffman/rustitude-core/commit/db587b5c5a82bf3d89637f0d542f8c5662554085))
    - Small demo, incomplete ([`cab86ae`](https://github.com/denehoffman/rustitude-core/commit/cab86aee21e1adf61b51d6400f1737090e1afba8))
    - I think I've failed again ([`7144a4f`](https://github.com/denehoffman/rustitude-core/commit/7144a4fcfe3e2db155cb7e1cf8dffd3c5d422b8f))
    - Create basic amplitude API ([`38870c2`](https://github.com/denehoffman/rustitude-core/commit/38870c27f0454ef903deb14cdb2507422761623c))
    - Add some benchmarks, speed up kmatrix by precalculating more of the p-vector ([`c46d2c4`](https://github.com/denehoffman/rustitude-core/commit/c46d2c48fec6e34741989508aa7fb655773685e4))
    - Trying out some SIMD stuff ([`927555d`](https://github.com/denehoffman/rustitude-core/commit/927555d85f54e80be3bc2bcf74d02fb82cec2c51))
    - First commit for this branch ([`12ec2a4`](https://github.com/denehoffman/rustitude-core/commit/12ec2a421ec25edd90ac7f82e11616e229eb5297))
    - Some updates to bacon and other configs ([`55443f6`](https://github.com/denehoffman/rustitude-core/commit/55443f6454e0072abc3cfc41d38e5fa297cdb9cd))
    - First commit of functional code ([`aa9d919`](https://github.com/denehoffman/rustitude-core/commit/aa9d91971816ff3d99a47e928be5bfb2360c0694))
    - Prototype ([`b03d9c6`](https://github.com/denehoffman/rustitude-core/commit/b03d9c6b35e46bca10f7d47fa1e7528386cf7a0a))
    - Gitignore ([`2c082e9`](https://github.com/denehoffman/rustitude-core/commit/2c082e9e6e20a238345d32c5164c35b77ef6239f))
</details>

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 34 calendar days.
 - 34 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude version 0.2.0-alpha ([`be408f1`](https://github.com/denehoffman/rustitude-core/commit/be408f129f003ec8ec273cc2a7e89480c743b525))
    - Update README.md ([`74a6366`](https://github.com/denehoffman/rustitude-core/commit/74a6366d63556e60532036e00179cc05d385f500))
    - Merge pull request #3 from denehoffman/nodule ([`e64a074`](https://github.com/denehoffman/rustitude-core/commit/e64a07424a9379981f49e59045ab9ecaac30983a))
    - Remove experimental feature that was unused ([`040a79c`](https://github.com/denehoffman/rustitude-core/commit/040a79c53d18d5550e2ac4a9096e8a43b7a31a98))
    - Complete rewrite of basically everything. ([`e6e1e5b`](https://github.com/denehoffman/rustitude-core/commit/e6e1e5bd15e2bcb8d723953876e31734a4ddf291))
    - Initial commit for new API ([`e225408`](https://github.com/denehoffman/rustitude-core/commit/e2254081e30b17bfa3803cca93a42361f0772903))
    - Changing stuff around so we no longer run out of memory (DashMaps bad) but it's very slow ([`1e46650`](https://github.com/denehoffman/rustitude-core/commit/1e466508aa95edbdacac60cb1ad495a8e5bcacd0))
    - Change par_evaluate_on to evaluate_on_par ([`ffb0253`](https://github.com/denehoffman/rustitude-core/commit/ffb02531cc77f25bc9bdf02b19df97eb7b82b28b))
    - Demo the Ylm amplitude ([`fcd8d8e`](https://github.com/denehoffman/rustitude-core/commit/fcd8d8ea14237bb900c5b8451c4970579cdfa10e))
    - Re-implement dependency resolution ([`67279a8`](https://github.com/denehoffman/rustitude-core/commit/67279a800722f3bf90754502126f5a387f0dabd8))
    - Add Resolver trait and some other minor organizational things ([`c57b8cc`](https://github.com/denehoffman/rustitude-core/commit/c57b8ccb558f61bae4739b0e22604856807d741a))
    - Changed resolve to function to match amplitude ([`bf6d071`](https://github.com/denehoffman/rustitude-core/commit/bf6d0714c8969e3e82f3479778ae033b7a1cfd8b))
    - Major rewrite of Dataset and Variable types ([`45274b2`](https://github.com/denehoffman/rustitude-core/commit/45274b24fb5cd7d596585909c868ae6223af3824))
    - Update README.md ([`e064d5b`](https://github.com/denehoffman/rustitude-core/commit/e064d5bbf75ad4efa8fb56d41b8b05f3de0e5d8b))
    - Give logo a white background ([`7e561e9`](https://github.com/denehoffman/rustitude-core/commit/7e561e9a5bfebcf901c0f460448f9f68c286757f))
    - Add logo ([`e64d69a`](https://github.com/denehoffman/rustitude-core/commit/e64d69a70e00537cd059b0e8cddfbe6c9ebc656b))
</details>

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude v0.1.3 ([`be29215`](https://github.com/denehoffman/rustitude-core/commit/be292152a39c1d693cd7ae5d23057db71f9b4350))
    - Merge branch 'main' of github.com:denehoffman/rustitude ([`0e180b9`](https://github.com/denehoffman/rustitude-core/commit/0e180b9bfc7504d92be07ff999affa307d78861f))
    - Try out logging fit steps ([`e85e1ca`](https://github.com/denehoffman/rustitude-core/commit/e85e1ca1fb81476ac90249b57bcdc60e22881d9a))
    - Remove some commented-out code ([`f0b655b`](https://github.com/denehoffman/rustitude-core/commit/f0b655bac628665bf7f1a479f3e941d0280407ae))
    - Add Pow trait and pow function to Amplitude ([`545f263`](https://github.com/denehoffman/rustitude-core/commit/545f263a714e9d8ed7fc91c7250af97275fe9738))
    - Allow for different amplitudes for data and MC ([`0dff196`](https://github.com/denehoffman/rustitude-core/commit/0dff19617e8264c61a9c1569b06a56797c4f55d3))
    - Add Branch struct ([`676daf3`](https://github.com/denehoffman/rustitude-core/commit/676daf37764d153e4d7c4898c9784b3243814f2b))
    - Doctests were taking Strings after I changed to &str ([`2f5de7f`](https://github.com/denehoffman/rustitude-core/commit/2f5de7f864a35d38f9c6d612a4e3db5354b4c2fe))
    - Move data extraction into dataset and propogate errors ([`2c0a933`](https://github.com/denehoffman/rustitude-core/commit/2c0a933b1e2861987b172cdfc81a479ced68792c))
    - Change inputs to functions to &str ([`3442057`](https://github.com/denehoffman/rustitude-core/commit/3442057a56351b0fb3a5c53fb89df242c36a4c66))
    - Delete .github/workflows/release-plz.yml ([`7562e69`](https://github.com/denehoffman/rustitude-core/commit/7562e699053e758b1606391bfebcab54447b316e))
</details>

## 0.1.2-beta.2 (2023-12-29)

<csr-id-8d596bf94049e0cd4327902bf63b9ae240c51a13/>

### Chore

 - <csr-id-8d596bf94049e0cd4327902bf63b9ae240c51a13/> release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #2 from denehoffman/release-plz-2023-12-29T05-39-06Z ([`432a50f`](https://github.com/denehoffman/rustitude-core/commit/432a50fc89bf310aca6bfbd57766d1e5f5a961ad))
    - Release ([`8d596bf`](https://github.com/denehoffman/rustitude-core/commit/8d596bf94049e0cd4327902bf63b9ae240c51a13))
    - Update and rename main.yml to release-plz.yml ([`d7ecaf0`](https://github.com/denehoffman/rustitude-core/commit/d7ecaf0fabadf92d8073eeab40be4490a8de9083))
    - Create main.yml ([`5260781`](https://github.com/denehoffman/rustitude-core/commit/52607811145a82e98938b0f009711cbbb5493523))
    - Typos in documentation ([`dc53009`](https://github.com/denehoffman/rustitude-core/commit/dc530098b4ff2dd6195e8b00fe4365d2d0d4abb8))
    - Update so doctests pass ([`f8d544b`](https://github.com/denehoffman/rustitude-core/commit/f8d544bd5775474bc19bb9e09ccbce78ad092eeb))
</details>

## 0.1.2-beta.1 (2023-12-29)

<csr-id-0b5ace3f3d7f9c2549e2011a488bbd35d55290d9/>

### Chore

 - <csr-id-0b5ace3f3d7f9c2549e2011a488bbd35d55290d9/> Release rustitude version 0.1.2-beta.1

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 7 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rustitude version 0.1.2-beta.1 ([`0b5ace3`](https://github.com/denehoffman/rustitude-core/commit/0b5ace3f3d7f9c2549e2011a488bbd35d55290d9))
    - Update documentation ([`0a4329a`](https://github.com/denehoffman/rustitude-core/commit/0a4329ae32d1ccc82bb439f4db019ea558e34892))
    - Overhaul of Parameters and how evaluate calls work, amplitudes keep track of external parameters, implemented ParticleSwarm with likelihood ([`16536e3`](https://github.com/denehoffman/rustitude-core/commit/16536e3aae353b15dc2fe6777ed656486ef82130))
    - Remove all data from repo ([`14ea8f0`](https://github.com/denehoffman/rustitude-core/commit/14ea8f0ef61fe4615239656f519d84806f790cf2))
    - Add Variantly to clear up a bunch of boilerplate, some other aesthetic changes ([`997a27c`](https://github.com/denehoffman/rustitude-core/commit/997a27c7a0242ebfd62023f07f40a288899be5f1))
    - More pedantic clippy suggestions, some documentation too ([`ff6c741`](https://github.com/denehoffman/rustitude-core/commit/ff6c741e4ae566cc2a31196d4dc3307f191ec211))
    - Add some flags for running ([`6d43621`](https://github.com/denehoffman/rustitude-core/commit/6d436211e1bba318fe61369ce3bf0d5fdbbbe312))
    - Linting fixes ([`87d9e93`](https://github.com/denehoffman/rustitude-core/commit/87d9e93eefe017b666502b9c6d25a6aae5eb3a52))
    - Remove with_deps ([`fd63c5b`](https://github.com/denehoffman/rustitude-core/commit/fd63c5baa8111de14cc1021eea063f172e743a5d))
    - Add debugging to release (for now) ([`59a361a`](https://github.com/denehoffman/rustitude-core/commit/59a361a94f36265606e04fcc1e2d46fb31127aca))
    - Update .gitignore ([`72384f0`](https://github.com/denehoffman/rustitude-core/commit/72384f0817079a4bfc2a471641715598b6f6b38e))
    - Add bacon.toml ([`bea460e`](https://github.com/denehoffman/rustitude-core/commit/bea460eb6e6cb6c60d6c6c6ee0bbafc9371d7189))
    - Merge branch 'main' of github.com:denehoffman/rustitude ([`cc21ca1`](https://github.com/denehoffman/rustitude-core/commit/cc21ca159716b313a33ea45f7d4f4809a2e1106b))
    - Remove main.rs from crate ([`79df6a6`](https://github.com/denehoffman/rustitude-core/commit/79df6a643ecfcb6a340740d30bce22fbf1a419e3))
    - Offload barrier factor and mass calcualtion to variables, huge speedup ([`7d99261`](https://github.com/denehoffman/rustitude-core/commit/7d99261c1e0eaef2f58e370d1b5e6409782b95d1))
    - Change input to evaluate_on and par_evaluate_all from vectors to slices ([`3dcd766`](https://github.com/denehoffman/rustitude-core/commit/3dcd7665faa015d1d0dca54798519e597df660b5))
    - Update unwrapping functions ([`bd2dd88`](https://github.com/denehoffman/rustitude-core/commit/bd2dd889b5b49c11f91053de197f941d450a7a60))
    - Add some upgrades to the standard library for performance (looks like parking_lot helps a lot) ([`f698a8e`](https://github.com/denehoffman/rustitude-core/commit/f698a8e4f198c1cb140b2217ea1f87c3a7d0b971))
    - Fix compilation on ARM Macs ([`87c0ce8`](https://github.com/denehoffman/rustitude-core/commit/87c0ce88b85633b9517debe832fae5eee306df77))
    - Update rust.yml ([`392afdf`](https://github.com/denehoffman/rustitude-core/commit/392afdf76715397f17dc0b0481be20a6a27fc7b4))
    - Add some derives for the parameter structs, changed their name to &str and a few other modifications, also add some likelihood code ([`38d4448`](https://github.com/denehoffman/rustitude-core/commit/38d4448cf90748342bf6b0a9985fa5fb01b0827a))
    - Bump version ([`b7e4b93`](https://github.com/denehoffman/rustitude-core/commit/b7e4b9319e1f26a04bdbce49a1c3a50ec11b7f87))
    - Alias ndarray-linalg features ([`7dc3c60`](https://github.com/denehoffman/rustitude-core/commit/7dc3c602976e2c5b0da215123f5605422c7c539e))
    - Create README.md ([`2a82b08`](https://github.com/denehoffman/rustitude-core/commit/2a82b08812755f8c480627a09b0bcf764f3c3a76))
    - Create LICENSE ([`2c500da`](https://github.com/denehoffman/rustitude-core/commit/2c500da9a9c7123c0516840ce92926a07a19135d))
    - Delete data.root ([`48f2fae`](https://github.com/denehoffman/rustitude-core/commit/48f2faedfdf3d5bd9050bbacc38e647edda11afc))
    - Create rust.yml ([`865837a`](https://github.com/denehoffman/rustitude-core/commit/865837a8ef8beab96c17a0f61b7bc95e5a0995d2))
    - Actually we can have parquets back for now ([`12d7cc4`](https://github.com/denehoffman/rustitude-core/commit/12d7cc43563f3201161f82f244d928d96019f193))
    - Remove data files ([`b18eb01`](https://github.com/denehoffman/rustitude-core/commit/b18eb0132e4d6843fd988d3826739cb6b45c9f9b))
    - Initial commit ([`ba95984`](https://github.com/denehoffman/rustitude-core/commit/ba959845593f7906a2b3e1be247373bf3c5e4635))
</details>

