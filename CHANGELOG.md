# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.1 (2023-06-05)

### New Features

 - <csr-id-ea7d2648b7adc1fb4cef184f2a4b1c9b2868c46d/> Reuse existing commands filter/action to output config snippets
 - <csr-id-975995c36897e3c9e68c99daf065cc4c52f8b71c/> Implement confirmation dialogue before irreversibly applying an action
 - <csr-id-37aad75d242f73091b3fc1058db28c40bbe6da70/> implement previewing an action
 - <csr-id-27f0ad6f0613eee61df03bf8f2385cee94799f85/> split the pipeline into several actors that could be run in parallel if needed

### Bug Fixes

 - <csr-id-404d0b8af5c2b16b454fdcc3b3bcfdef7c709356/> Fix descriptions for commands
 - <csr-id-859f50fdfab805a51567808f0631a24a3b522298/> Reexport TemplateStringKind for documentation

### Other

 - <csr-id-7144ddda26a7a4368d5c5af3a3108df8217cee4f/> add doctests for some actions

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix descriptions for commands ([`404d0b8`](https://github.com/organize-rs/organize/commit/404d0b8af5c2b16b454fdcc3b3bcfdef7c709356))
    - Reuse existing commands filter/action to output config snippets ([`ea7d264`](https://github.com/organize-rs/organize/commit/ea7d2648b7adc1fb4cef184f2a4b1c9b2868c46d))
    - Implement confirmation dialogue before irreversibly applying an action ([`975995c`](https://github.com/organize-rs/organize/commit/975995c36897e3c9e68c99daf065cc4c52f8b71c))
    - Implement previewing an action ([`37aad75`](https://github.com/organize-rs/organize/commit/37aad75d242f73091b3fc1058db28c40bbe6da70))
    - Split the pipeline into several actors that could be run in parallel if needed ([`27f0ad6`](https://github.com/organize-rs/organize/commit/27f0ad6f0613eee61df03bf8f2385cee94799f85))
    - Reexport TemplateStringKind for documentation ([`859f50f`](https://github.com/organize-rs/organize/commit/859f50fdfab805a51567808f0631a24a3b522298))
    - Add doctests for some actions ([`7144ddd`](https://github.com/organize-rs/organize/commit/7144ddda26a7a4368d5c5af3a3108df8217cee4f))
</details>

## 0.4.0 (2023-06-03)

<csr-id-930593f55f0128609daf98ccee901d5e33c29223/>
<csr-id-cb1e1151c09afed48f40b989132ae89324f9f2b1/>
<csr-id-7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2/>
<csr-id-19110b9592fb35314402f9bac71f52791c5a7cd1/>
<csr-id-ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9/>
<csr-id-22192eb528c2f95ad01ef124082dfae518e8c464/>
<csr-id-afb116bc91add3ac76f6c55e44932391eec6e7e8/>
<csr-id-3f6df32eac07d369b035ffb06156c2b47b0f9121/>
<csr-id-abdc5ecfe89c8db0b3f7077681ef3a23a7797df5/>
<csr-id-91a8840c5826eb39493384a3e2d1bbb69f38c298/>
<csr-id-18ee7029fa931b0f95046f11c2919b23f11c1470/>
<csr-id-075f0256f0dc5c2a69e350c2232468988f31c3e2/>
<csr-id-b544485a914108185f2e10e1b7ae336acdc7ac5e/>
<csr-id-a3163027800f1843476b8855a41f2115fdb3d3eb/>
<csr-id-85fd452e00835f14e84da9547bb5659449771cde/>
<csr-id-8b47644af39c99f79658cdd54c067f9787b02347/>
<csr-id-2c06e62ede4f1de31c75dd786c65f87b0855dffc/>
<csr-id-fb1a2158884b70ea37df7b332acd6d4170b07e91/>
<csr-id-54a8993981273a39fb0da07de5853a2fbc5764b4/>
<csr-id-99d7b0c109017a78c1ef726dcc9d40d0507b0426/>
<csr-id-a82a20821ebe60d386efcb9856f4d22b45ab4a2e/>
<csr-id-fd90047424eb9e6f04481a1ef35825e360b06912/>
<csr-id-13984918e9cbbe3a7787af715004b5729b5a58a8/>


<csr-id-cb1e1151c09afed48f40b989132ae89324f9f2b1/>
<csr-id-7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2/>
<csr-id-19110b9592fb35314402f9bac71f52791c5a7cd1/>
<csr-id-ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9/>
<csr-id-22192eb528c2f95ad01ef124082dfae518e8c464/>
<csr-id-afb116bc91add3ac76f6c55e44932391eec6e7e8/>
<csr-id-3f6df32eac07d369b035ffb06156c2b47b0f9121/>
<csr-id-abdc5ecfe89c8db0b3f7077681ef3a23a7797df5/>
<csr-id-91a8840c5826eb39493384a3e2d1bbb69f38c298/>
<csr-id-18ee7029fa931b0f95046f11c2919b23f11c1470/>
<csr-id-075f0256f0dc5c2a69e350c2232468988f31c3e2/>
<csr-id-b544485a914108185f2e10e1b7ae336acdc7ac5e/>
<csr-id-a3163027800f1843476b8855a41f2115fdb3d3eb/>
<csr-id-85fd452e00835f14e84da9547bb5659449771cde/>
<csr-id-8b47644af39c99f79658cdd54c067f9787b02347/>
<csr-id-2c06e62ede4f1de31c75dd786c65f87b0855dffc/>
<csr-id-fb1a2158884b70ea37df7b332acd6d4170b07e91/>
<csr-id-54a8993981273a39fb0da07de5853a2fbc5764b4/>
<csr-id-99d7b0c109017a78c1ef726dcc9d40d0507b0426/>
<csr-id-a82a20821ebe60d386efcb9856f4d22b45ab4a2e/>
<csr-id-fd90047424eb9e6f04481a1ef35825e360b06912/>
<csr-id-13984918e9cbbe3a7787af715004b5729b5a58a8/>

### Chore

 - <csr-id-930593f55f0128609daf98ccee901d5e33c29223/> fully annotate template config
 - <csr-id-cb1e1151c09afed48f40b989132ae89324f9f2b1/> cargo fix & cargo fmt

### Documentation

 - <csr-id-0f49e89282eaa8fc5f11a1db81553484e9a669b9/> start annotating a template config file

### New Features

 - <csr-id-94757b5e2eddb54b0a706634f408bb5fcabfaf94/> implement parsing rules from config files
 - <csr-id-13388896c02874263ff7838591fd4776a7912b1e/> docs command opens browser
 - <csr-id-d088c50c894a9a94a47b2e3b66502fa3d5fdfc59/> adding cli option for filter mode
 - <csr-id-7c2991827f503c7caf6f82e964f9cc91f5c39d0e/> Free the way for applying multiple filters

### Bug Fixes

 - <csr-id-435488c65e552016a8c6a9f6849390441db2e1e6/> Fix generate completions

### Other

 - <csr-id-7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2/> check for tags when running rules from config file
 - <csr-id-19110b9592fb35314402f9bac71f52791c5a7cd1/> implement template generation
 - <csr-id-ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9/> implement some snapshot testing
 - <csr-id-22192eb528c2f95ad01ef124082dfae518e8c464/> fix naming in config
 - <csr-id-afb116bc91add3ac76f6c55e44932391eec6e7e8/> more state changes for runner
 - <csr-id-3f6df32eac07d369b035ffb06156c2b47b0f9121/> implement first part of rule runner
 - <csr-id-abdc5ecfe89c8db0b3f7077681ef3a23a7797df5/> first implementation draft for a config run
 - <csr-id-91a8840c5826eb39493384a3e2d1bbb69f38c298/> parse first parts of py-organize config
 - <csr-id-18ee7029fa931b0f95046f11c2919b23f11c1470/> Switch to jwalk to walk the directory tree parallelized
 - <csr-id-075f0256f0dc5c2a69e350c2232468988f31c3e2/> add new run command to run a specified rule from a config or script
 - <csr-id-b544485a914108185f2e10e1b7ae336acdc7ac5e/> add new config command to generate and check configs
 - <csr-id-a3163027800f1843476b8855a41f2115fdb3d3eb/> apply multiple filters to walkdir iter

### Refactor

 - <csr-id-85fd452e00835f14e84da9547bb5659449771cde/> split filters in seperate modules
 - <csr-id-8b47644af39c99f79658cdd54c067f9787b02347/> flatten library structure
 - <csr-id-2c06e62ede4f1de31c75dd786c65f87b0855dffc/> change Rule to contain new types
 - <csr-id-fb1a2158884b70ea37df7b332acd6d4170b07e91/> make command arguments more reasonable
 - <csr-id-54a8993981273a39fb0da07de5853a2fbc5764b4/> add scripting and shell filter, add check for config and scripts subcommand, and run subcommands for config and scripts, add generate subcommand
 - <csr-id-99d7b0c109017a78c1ef726dcc9d40d0507b0426/> improve display
 - <csr-id-a82a20821ebe60d386efcb9856f4d22b45ab4a2e/> Make FilterWalker more configurable for upcoming features
 - <csr-id-fd90047424eb9e6f04481a1ef35825e360b06912/> make filters use more option/result methods

### Other (BREAKING)

 - <csr-id-13984918e9cbbe3a7787af715004b5729b5a58a8/> Re-export filters and actions from core library for documentation purposes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 29 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Re-export filters and actions from core library for documentation purposes ([`1398491`](https://github.com/organize-rs/organize/commit/13984918e9cbbe3a7787af715004b5729b5a58a8))
    - Fully annotate template config ([`930593f`](https://github.com/organize-rs/organize/commit/930593f55f0128609daf98ccee901d5e33c29223))
    - Check for tags when running rules from config file ([`7c0b0c9`](https://github.com/organize-rs/organize/commit/7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2))
    - Cargo fix & cargo fmt ([`cb1e115`](https://github.com/organize-rs/organize/commit/cb1e1151c09afed48f40b989132ae89324f9f2b1))
    - Implement template generation ([`19110b9`](https://github.com/organize-rs/organize/commit/19110b9592fb35314402f9bac71f52791c5a7cd1))
    - Start annotating a template config file ([`0f49e89`](https://github.com/organize-rs/organize/commit/0f49e89282eaa8fc5f11a1db81553484e9a669b9))
    - Implement some snapshot testing ([`ad6f684`](https://github.com/organize-rs/organize/commit/ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9))
    - Implement parsing rules from config files ([`94757b5`](https://github.com/organize-rs/organize/commit/94757b5e2eddb54b0a706634f408bb5fcabfaf94))
    - Fix naming in config ([`22192eb`](https://github.com/organize-rs/organize/commit/22192eb528c2f95ad01ef124082dfae518e8c464))
    - More state changes for runner ([`afb116b`](https://github.com/organize-rs/organize/commit/afb116bc91add3ac76f6c55e44932391eec6e7e8))
    - Implement first part of rule runner ([`3f6df32`](https://github.com/organize-rs/organize/commit/3f6df32eac07d369b035ffb06156c2b47b0f9121))
    - First implementation draft for a config run ([`abdc5ec`](https://github.com/organize-rs/organize/commit/abdc5ecfe89c8db0b3f7077681ef3a23a7797df5))
    - Split filters in seperate modules ([`85fd452`](https://github.com/organize-rs/organize/commit/85fd452e00835f14e84da9547bb5659449771cde))
    - More deserialization attempts ([`c32b4b3`](https://github.com/organize-rs/organize/commit/c32b4b30c602b61d5ce94a037ccf2a496232c935))
    - Flatten library structure ([`8b47644`](https://github.com/organize-rs/organize/commit/8b47644af39c99f79658cdd54c067f9787b02347))
    - Parse first parts of py-organize config ([`91a8840`](https://github.com/organize-rs/organize/commit/91a8840c5826eb39493384a3e2d1bbb69f38c298))
    - Change Rule to contain new types ([`2c06e62`](https://github.com/organize-rs/organize/commit/2c06e62ede4f1de31c75dd786c65f87b0855dffc))
    - Fix generate completions ([`435488c`](https://github.com/organize-rs/organize/commit/435488c65e552016a8c6a9f6849390441db2e1e6))
    - Make command arguments more reasonable ([`fb1a215`](https://github.com/organize-rs/organize/commit/fb1a2158884b70ea37df7b332acd6d4170b07e91))
    - Add scripting and shell filter, add check for config and scripts subcommand, and run subcommands for config and scripts, add generate subcommand ([`54a8993`](https://github.com/organize-rs/organize/commit/54a8993981273a39fb0da07de5853a2fbc5764b4))
    - Start sketching out scripting ideas ([`54a3446`](https://github.com/organize-rs/organize/commit/54a34468662642d3e2f161425be28cb957859b78))
    - Switch to jwalk to walk the directory tree parallelized ([`18ee702`](https://github.com/organize-rs/organize/commit/18ee7029fa931b0f95046f11c2919b23f11c1470))
    - Add new run command to run a specified rule from a config or script ([`075f025`](https://github.com/organize-rs/organize/commit/075f0256f0dc5c2a69e350c2232468988f31c3e2))
    - Add new config command to generate and check configs ([`b544485`](https://github.com/organize-rs/organize/commit/b544485a914108185f2e10e1b7ae336acdc7ac5e))
    - Docs command opens browser ([`1338889`](https://github.com/organize-rs/organize/commit/13388896c02874263ff7838591fd4776a7912b1e))
    - Improve display ([`99d7b0c`](https://github.com/organize-rs/organize/commit/99d7b0c109017a78c1ef726dcc9d40d0507b0426))
    - Adding cli option for filter mode ([`d088c50`](https://github.com/organize-rs/organize/commit/d088c50c894a9a94a47b2e3b66502fa3d5fdfc59))
    - Make FilterWalker more configurable for upcoming features ([`a82a208`](https://github.com/organize-rs/organize/commit/a82a20821ebe60d386efcb9856f4d22b45ab4a2e))
    - Free the way for applying multiple filters ([`7c29918`](https://github.com/organize-rs/organize/commit/7c2991827f503c7caf6f82e964f9cc91f5c39d0e))
    - Fix borrow issues ([`398edb2`](https://github.com/organize-rs/organize/commit/398edb29916398c43b15531779233eda4b28eef5))
    - Apply multiple filters to walkdir iter ([`a316302`](https://github.com/organize-rs/organize/commit/a3163027800f1843476b8855a41f2115fdb3d3eb))
    - Make filters use more option/result methods ([`fd90047`](https://github.com/organize-rs/organize/commit/fd90047424eb9e6f04481a1ef35825e360b06912))
</details>

## 0.3.1 (2023-05-25)

<csr-id-4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4/>
<csr-id-95b9b3cf0ce06a224c0c79782ae7c470c31475d8/>
<csr-id-36d02df0f52798af534151ea4d3ed4f7876934b2/>
<csr-id-09c428cc45bbb348ca08e5fd233c999408ca2500/>
<csr-id-543150dd96aca886bacd0057bda1957d19b4322d/>
<csr-id-14300ea60bcccf500d813ee267792899a278a9ff/>
<csr-id-c005013d1c49f5d717d635c3ece760bb5c904e09/>


<csr-id-95b9b3cf0ce06a224c0c79782ae7c470c31475d8/>
<csr-id-36d02df0f52798af534151ea4d3ed4f7876934b2/>
<csr-id-09c428cc45bbb348ca08e5fd233c999408ca2500/>
<csr-id-543150dd96aca886bacd0057bda1957d19b4322d/>
<csr-id-14300ea60bcccf500d813ee267792899a278a9ff/>
<csr-id-c005013d1c49f5d717d635c3ece760bb5c904e09/>

### New Features

 - <csr-id-1246468f87132a8b52169fd186f3626f357817bd/> docs command opens browser
 - <csr-id-fea4cc14fe1f64cc9fd91664bf07f8940cba15a1/> adding cli option for filter mode
 - <csr-id-33fc910001ac35967bc1e424b110c88ace6b9186/> Free the way for applying multiple filters

### Other

 - <csr-id-4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4/> Switch to jwalk to walk the directory tree parallelized
 - <csr-id-95b9b3cf0ce06a224c0c79782ae7c470c31475d8/> add new run command to run a specified rule from a config or script
 - <csr-id-36d02df0f52798af534151ea4d3ed4f7876934b2/> add new config command to generate and check configs
 - <csr-id-09c428cc45bbb348ca08e5fd233c999408ca2500/> apply multiple filters to walkdir iter

### Refactor

 - <csr-id-543150dd96aca886bacd0057bda1957d19b4322d/> improve display
 - <csr-id-14300ea60bcccf500d813ee267792899a278a9ff/> Make FilterWalker more configurable for upcoming features
 - <csr-id-c005013d1c49f5d717d635c3ece760bb5c904e09/> make filters use more option/result methods

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Switch to jwalk to walk the directory tree parallelized ([`4757013`](https://github.com/organize-rs/organize/commit/4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4))
    - Add new run command to run a specified rule from a config or script ([`95b9b3c`](https://github.com/organize-rs/organize/commit/95b9b3cf0ce06a224c0c79782ae7c470c31475d8))
    - Add new config command to generate and check configs ([`36d02df`](https://github.com/organize-rs/organize/commit/36d02df0f52798af534151ea4d3ed4f7876934b2))
    - Docs command opens browser ([`1246468`](https://github.com/organize-rs/organize/commit/1246468f87132a8b52169fd186f3626f357817bd))
    - Improve display ([`543150d`](https://github.com/organize-rs/organize/commit/543150dd96aca886bacd0057bda1957d19b4322d))
    - Adding cli option for filter mode ([`fea4cc1`](https://github.com/organize-rs/organize/commit/fea4cc14fe1f64cc9fd91664bf07f8940cba15a1))
    - Make FilterWalker more configurable for upcoming features ([`14300ea`](https://github.com/organize-rs/organize/commit/14300ea60bcccf500d813ee267792899a278a9ff))
    - Free the way for applying multiple filters ([`33fc910`](https://github.com/organize-rs/organize/commit/33fc910001ac35967bc1e424b110c88ace6b9186))
    - Fix borrow issues ([`3d1b0a1`](https://github.com/organize-rs/organize/commit/3d1b0a19e71441bdec6a9b609833b91a8ef890d8))
    - Apply multiple filters to walkdir iter ([`09c428c`](https://github.com/organize-rs/organize/commit/09c428cc45bbb348ca08e5fd233c999408ca2500))
    - Make filters use more option/result methods ([`c005013`](https://github.com/organize-rs/organize/commit/c005013d1c49f5d717d635c3ece760bb5c904e09))
    - Start sketching out scripting ideas ([`b84d301`](https://github.com/organize-rs/organize/commit/b84d3019f5d334c61bb85733db16bf85ed7072d7))
</details>

## 0.3.0 (2023-05-21)

### Feature

- Breaking change: Change syntax for date based filters to use range based syntax 

## 0.2.6 (2023-05-21)

<csr-id-d2b33280185f68daaff2671d1a4470d9041f3bcb/>



### Chore

 - <csr-id-d2b33280185f68daaff2671d1a4470d9041f3bcb/> add description to lib.rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add description to lib.rs ([`d2b3328`](https://github.com/organize-rs/organize/commit/d2b33280185f68daaff2671d1a4470d9041f3bcb))
</details>

## 0.2.5 (2023-05-20)

### New Features

 - <csr-id-345d8885d1ffe9bcfdc42c62fccbdc59a457ed0a/> Implement `mimetype` filter

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Implement `mimetype` filter ([`345d888`](https://github.com/organize-rs/organize/commit/345d8885d1ffe9bcfdc42c62fccbdc59a457ed0a))
    - Implement `created` filter ([`f07ab6a`](https://github.com/organize-rs/organize/commit/f07ab6a4bd9be7674dad416f7b74e9b9196b3dca))
    - Remove human-panic dependency ([`9382256`](https://github.com/organize-rs/organize/commit/938225668c8879192a8e81a4872797907e3b4641))
    - Research dependencies ([`9f12de9`](https://github.com/organize-rs/organize/commit/9f12de940ba56278c3eec43449dd5663f284e1e4))
    - Cargo fix & cargo fmt ([`ee231a6`](https://github.com/organize-rs/organize/commit/ee231a69fcd825e6121c380f408c21ff2bf6c425))
    - Cargo fix ([`0695061`](https://github.com/organize-rs/organize/commit/06950617d566bd19764a3f4b403a92f787b2536d))
    - Add doc comments for ignore args ([`626a2ac`](https://github.com/organize-rs/organize/commit/626a2ac78814a6fb4f654f22bb27e422aa136fcf))
    - Implement `empty` filter and global ignore for file names and directory paths ([`d51a81a`](https://github.com/organize-rs/organize/commit/d51a81a593cb37c54c0c91edfac60a5eb8de7c89))
</details>

## 0.2.4 (2023-05-20)

## 0.2.3 (2023-05-19)

## 0.2.2 (2023-05-19)

## 0.2.1 (2023-05-18)

## 0.2.0 (2023-05-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix some grouping issues in Cli, create `cli` feature in organize-rs_core ([`b734e62`](https://github.com/organize-rs/organize/commit/b734e625d869163b07f63923414ffa900f93ca64))
    - Implement `filter_by_extension` ([`45e4d5b`](https://github.com/organize-rs/organize/commit/45e4d5b03185d5cd016d16795fdba0336c1496bd))
    - First try for implementing a file extension filter ([`45f2966`](https://github.com/organize-rs/organize/commit/45f296647ea46461ec89550f48eb22e07c037d5c))
    - Fix indirection ([`e6fde80`](https://github.com/organize-rs/organize/commit/e6fde8017240234eb4cb7e1adb259b5a2b6abd7c))
    - Implement stub for filter methods ([`6c6f0f8`](https://github.com/organize-rs/organize/commit/6c6f0f89709a5f7b78ad8de3099ac3cbd6c5f6e3))
    - Add czkawka_core dependency ([`5063aec`](https://github.com/organize-rs/organize/commit/5063aecdd41b99534d7c2539bcd60a5756401110))
    - Refine commands/subcommands ([`ed535f6`](https://github.com/organize-rs/organize/commit/ed535f68f92e4ec187a73fb628fcf4e86d1bda3e))
    - Add `actions` and `filters` as subcommands ([`60df64e`](https://github.com/organize-rs/organize/commit/60df64e3380870fb5182e9cd4f47bb792bc55ce7))
    - Start parsing config ([`0e36272`](https://github.com/organize-rs/organize/commit/0e36272f9e7db8e65daaad39d228d986ab952673))
    - Refactor to workspace and create new core library ([`0de540b`](https://github.com/organize-rs/organize/commit/0de540b0aa0ab07dc4f3b118e6f95b30312ea44e))
    - Support opening text editor on Linux and OSX ([`b5a62b6`](https://github.com/organize-rs/organize/commit/b5a62b611987c1933c8dbfaaaf17a56586d0676e))
    - Implement `edit` command ([`a03feb2`](https://github.com/organize-rs/organize/commit/a03feb276f3f832254c1fc1ce00802ba0b2693cd))
    - Add boilerplate implementations for Enums ([`3c54381`](https://github.com/organize-rs/organize/commit/3c54381db155151046a81b796763fe1bf6bdeefb))
</details>

## v0.1.0 (2023-05-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update Readme and add package details to Cargo.toml ([`7265563`](https://github.com/organize-rs/organize/commit/72655635badb43ad473f82348ee0375e15f312d9))
    - Derive Deserialize and Serialize via Serde ([`2fb46a5`](https://github.com/organize-rs/organize/commit/2fb46a595bf4cc4ebdb1e9a55ae88a036f45c8d3))
    - Derive Copy for some Enums ([`16acd7d`](https://github.com/organize-rs/organize/commit/16acd7db44946ada7dabe166b7c5966ddf64370a))
    - Add aliases and refactor ([`0cda7dc`](https://github.com/organize-rs/organize/commit/0cda7dce78393c015bc0b72e0a6d50e5bfcf86dd))
    - Fixes to documention ([`1ae9d53`](https://github.com/organize-rs/organize/commit/1ae9d538745d8d624ad9c36b4c850a73c151bcf8))
    - Sketch out OrganizeFilter from documentation ([`27d7cdb`](https://github.com/organize-rs/organize/commit/27d7cdb8aca99cf758d4c532cc04942431e26bee))
    - Refine Recurse and Filters into Enum ([`5a35643`](https://github.com/organize-rs/organize/commit/5a35643d7d17e58691d325a3579744972e4e89c3))
    - Reorganize `actions` module ([`453c5ba`](https://github.com/organize-rs/organize/commit/453c5ba0c51cfea33165b33e22a354125b86af9a))
    - Sketch out OrganizeAction from documentation ([`f26488c`](https://github.com/organize-rs/organize/commit/f26488ce12b4f5709a104afc99b3362a304b19b1))
    - Implement `reveal` command ([`9083240`](https://github.com/organize-rs/organize/commit/908324034408452bd7387330928e9387e7d71aa1))
    - Add command stubs ([`b54a8a5`](https://github.com/organize-rs/organize/commit/b54a8a575abc8bf69e9946299c9bb076cc1b438e))
    - Fix clippy lints ([`3e36508`](https://github.com/organize-rs/organize/commit/3e365089de3a81292902f231ed9e4c19d885bf09))
    - Init abscissa app and generate config types from feldmann/organize ([`330a400`](https://github.com/organize-rs/organize/commit/330a400857b29282300034e191f830b21634125a))
</details>

