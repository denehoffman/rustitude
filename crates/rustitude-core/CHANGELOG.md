# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 (2024-05-15)

### Documentation

 - <csr-id-95f85ed6b16400c882e7535c7fa113ead9876353/> update links in readmes

### New Features

 - <csr-id-ae5bd43902756b612beecb057151dac39dfca208/> update ExtendedLogLikelihood call signature to make num_threads optional and default to 1

### Other

 - <csr-id-64ec5097cc99eb9bb6d73376e6d3b2788f637d9d/> more Cargo.lock and readme updates
 - <csr-id-fa66ff1c89c3761f22b8d1586c9df4eb81937a49/> Add rustitude-core to crates subdirectory

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update ExtendedLogLikelihood call signature to make num_threads optional and default to 1 ([`ae5bd43`](https://github.com/denehoffman/rustitude/commit/ae5bd43902756b612beecb057151dac39dfca208))
    - Update links in readmes ([`95f85ed`](https://github.com/denehoffman/rustitude/commit/95f85ed6b16400c882e7535c7fa113ead9876353))
    - More Cargo.lock and readme updates ([`64ec509`](https://github.com/denehoffman/rustitude/commit/64ec5097cc99eb9bb6d73376e6d3b2788f637d9d))
    - Add rustitude-core to crates subdirectory ([`fa66ff1`](https://github.com/denehoffman/rustitude/commit/fa66ff1c89c3761f22b8d1586c9df4eb81937a49))
</details>

## 0.3.4 (2024-05-06)

<csr-id-2137d6ed155df261e70f9905c6400a9fdcfade35/>
<csr-id-ce7f083ef0d38e53d6b2ebfe8382c9ed52146713/>

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

## 0.3.2 (2024-04-29)

<csr-id-8d9ec2e806f7d6f47413377e7d1ff92fa8819802/>
<csr-id-ba2e0b7750188d5d837ea32d03a4bdbea157819d/>

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
