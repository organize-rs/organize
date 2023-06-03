# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.2 (2023-06-03)

<csr-id-930593f55f0128609daf98ccee901d5e33c29223/>
<csr-id-cb1e1151c09afed48f40b989132ae89324f9f2b1/>
<csr-id-a7d737c5cc9e6465c7cf724c5d020b0f7e8ad935/>
<csr-id-a024a32327344ff109d8e6e68819f507d7c9e8ab/>
<csr-id-28ad00049f14ec1761f26baf2fe2088854094900/>
<csr-id-7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2/>
<csr-id-19110b9592fb35314402f9bac71f52791c5a7cd1/>
<csr-id-ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9/>
<csr-id-b665490feb2a7a44996847d80ee918ea82cc4772/>
<csr-id-22192eb528c2f95ad01ef124082dfae518e8c464/>
<csr-id-afb116bc91add3ac76f6c55e44932391eec6e7e8/>
<csr-id-3f6df32eac07d369b035ffb06156c2b47b0f9121/>
<csr-id-abdc5ecfe89c8db0b3f7077681ef3a23a7797df5/>
<csr-id-c2a5e017868e9f2af2bda3576b7e342356783722/>
<csr-id-67f506ea0f869bb0a55c1d3938d3d8b58612c979/>
<csr-id-a52abf6e5aa902c22c3fd7eebdd3088a2b3c6a50/>
<csr-id-2abf8ea4fb19e8d1be640fb52abe1aba9c4e08c4/>
<csr-id-db912c4ccfbc525ff4c5dccf231fda064e9a001e/>
<csr-id-2a75ebb8b239d79e02262e5aeb470c66b25afbb9/>
<csr-id-2d6ad427618ab76d5b73471dba62f6280218f63e/>
<csr-id-31c790a1b244c5b98de9bba21b5ed9052d47d788/>
<csr-id-db2d3480a1f965e861f92b8df04079461ee2f35d/>
<csr-id-91a8840c5826eb39493384a3e2d1bbb69f38c298/>
<csr-id-18ee7029fa931b0f95046f11c2919b23f11c1470/>
<csr-id-a3163027800f1843476b8855a41f2115fdb3d3eb/>
<csr-id-b633c561839314c760bf9a1a70e5b1aadac23b12/>
<csr-id-2dec9033ce8187cdb74b23e2363d09d65131bbdd/>
<csr-id-5b35248f532666f09dd8fd9308be1d16cb7c9f81/>
<csr-id-f4a9cfaa19d9f088768f2f705f14a400ad701a8b/>
<csr-id-85fd452e00835f14e84da9547bb5659449771cde/>
<csr-id-8b47644af39c99f79658cdd54c067f9787b02347/>
<csr-id-2c06e62ede4f1de31c75dd786c65f87b0855dffc/>
<csr-id-54a8993981273a39fb0da07de5853a2fbc5764b4/>
<csr-id-0c79c6b5b75adc05c54fee49f3398ed871ffd9b3/>
<csr-id-95d7740dfbda24d934eea7ecce21b39076cf251c/>
<csr-id-847737a5554f76a76366cd67fe8b2870277c935b/>
<csr-id-99d7b0c109017a78c1ef726dcc9d40d0507b0426/>
<csr-id-a82a20821ebe60d386efcb9856f4d22b45ab4a2e/>
<csr-id-fd90047424eb9e6f04481a1ef35825e360b06912/>
<csr-id-44a037fa00212ec2d07a0449f9b3d0aee05e3d55/>

### Chore

 - <csr-id-930593f55f0128609daf98ccee901d5e33c29223/> fully annotate template config
 - <csr-id-cb1e1151c09afed48f40b989132ae89324f9f2b1/> cargo fix & cargo fmt

### Chore

 - <csr-id-44a037fa00212ec2d07a0449f9b3d0aee05e3d55/> add license and config template to manifest and folder structure

### Documentation

 - <csr-id-e39e96c5b5b08ecad0cb197c62f49fd0809faabf/> add a bit more documentation for filters and actions
 - <csr-id-0f49e89282eaa8fc5f11a1db81553484e9a669b9/> start annotating a template config file
 - <csr-id-96c3d950812a8df8cdccb53a47ae0e6165462117/> add some thoughts about impl of actions

### New Features

 - <csr-id-f6bfca7440be8ee3f3aeed24b3164458cd2dcfe9/> consider tags when running rules from config
 - <csr-id-94757b5e2eddb54b0a706634f408bb5fcabfaf94/> implement parsing rules from config files
 - <csr-id-2c55b20c24da830f5d81468450882f126c8d5531/> add total entry count to output of filters
 - <csr-id-d088c50c894a9a94a47b2e3b66502fa3d5fdfc59/> adding cli option for filter mode
 - <csr-id-7c2991827f503c7caf6f82e964f9cc91f5c39d0e/> Free the way for applying multiple filters

### Bug Fixes

 - <csr-id-45182d5fa76f1716c98ce372aea4177ef6d3bb50/> chose better name for filter_groups
 - <csr-id-ef8ea621bd5fcbdb24050204ef77eb2e330cf880/> make MaxDepth newtype  transparent to serde

### Other

 - <csr-id-a7d737c5cc9e6465c7cf724c5d020b0f7e8ad935/> add more doctests for filters
 - <csr-id-a024a32327344ff109d8e6e68819f507d7c9e8ab/> add some doctests for filters
 - <csr-id-28ad00049f14ec1761f26baf2fe2088854094900/> add some doctests for filters
 - <csr-id-7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2/> check for tags when running rules from config file
 - <csr-id-19110b9592fb35314402f9bac71f52791c5a7cd1/> implement template generation
 - <csr-id-ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9/> implement some snapshot testing
 - <csr-id-b665490feb2a7a44996847d80ee918ea82cc4772/> make lists comma separated in filters
 - <csr-id-22192eb528c2f95ad01ef124082dfae518e8c464/> fix naming in config
 - <csr-id-afb116bc91add3ac76f6c55e44932391eec6e7e8/> more state changes for runner
 - <csr-id-3f6df32eac07d369b035ffb06156c2b47b0f9121/> implement first part of rule runner
 - <csr-id-abdc5ecfe89c8db0b3f7077681ef3a23a7797df5/> first implementation draft for a config run
 - <csr-id-c2a5e017868e9f2af2bda3576b7e342356783722/> implement more of filesystem and action part
 - <csr-id-67f506ea0f869bb0a55c1d3938d3d8b58612c979/> : add concurrency and filesystem functionality
 - <csr-id-a52abf6e5aa902c22c3fd7eebdd3088a2b3c6a50/> more unit tests for matches_date
 - <csr-id-2abf8ea4fb19e8d1be640fb52abe1aba9c4e08c4/> implement unit tests for date matching
 - <csr-id-db912c4ccfbc525ff4c5dccf231fda064e9a001e/> add unit test for mimetype
 - <csr-id-2a75ebb8b239d79e02262e5aeb470c66b25afbb9/> fix unit test
 - <csr-id-2d6ad427618ab76d5b73471dba62f6280218f63e/> implement more unit tests for name filter
 - <csr-id-31c790a1b244c5b98de9bba21b5ed9052d47d788/> implement more test cases for name filter
 - <csr-id-db2d3480a1f965e861f92b8df04079461ee2f35d/> add more unit tests for filters
 - <csr-id-91a8840c5826eb39493384a3e2d1bbb69f38c298/> parse first parts of py-organize config
 - <csr-id-18ee7029fa931b0f95046f11c2919b23f11c1470/> Switch to jwalk to walk the directory tree parallelized
 - <csr-id-a3163027800f1843476b8855a41f2115fdb3d3eb/> apply multiple filters to walkdir iter

### Refactor

 - <csr-id-b633c561839314c760bf9a1a70e5b1aadac23b12/> add branch itself to `tag_applies` method
 - <csr-id-2dec9033ce8187cdb74b23e2363d09d65131bbdd/> Move MAX_DEPTH to associated constant
 - <csr-id-5b35248f532666f09dd8fd9308be1d16cb7c9f81/> units tests to be less repetitive
 - <csr-id-f4a9cfaa19d9f088768f2f705f14a400ad701a8b/> to filetime for platform-agnostic atime, ctime, mtime inspection
 - <csr-id-85fd452e00835f14e84da9547bb5659449771cde/> split filters in seperate modules
 - <csr-id-8b47644af39c99f79658cdd54c067f9787b02347/> flatten library structure
 - <csr-id-2c06e62ede4f1de31c75dd786c65f87b0855dffc/> change Rule to contain new types
 - <csr-id-54a8993981273a39fb0da07de5853a2fbc5764b4/> add scripting and shell filter, add check for config and scripts subcommand, and run subcommands for config and scripts, add generate subcommand
 - <csr-id-0c79c6b5b75adc05c54fee49f3398ed871ffd9b3/> more functional syntax
 - <csr-id-95d7740dfbda24d934eea7ecce21b39076cf251c/> implement custom serializer for MaxDepth
 - <csr-id-847737a5554f76a76366cd67fe8b2870277c935b/> Add associated constants for SizeRange and PeriodRange
 - <csr-id-99d7b0c109017a78c1ef726dcc9d40d0507b0426/> improve display
 - <csr-id-a82a20821ebe60d386efcb9856f4d22b45ab4a2e/> Make FilterWalker more configurable for upcoming features
 - <csr-id-fd90047424eb9e6f04481a1ef35825e360b06912/> make filters use more option/result methods

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 57 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 50 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release organize-rs_core v0.2.2, organize-rs v0.4.0 ([`38b98b8`](https://github.com/organize-rs/organize/commit/38b98b805dd984773c7445d4b3bcdcfe62965596))
    - Add license and config template to manifest and folder structure ([`44a037f`](https://github.com/organize-rs/organize/commit/44a037fa00212ec2d07a0449f9b3d0aee05e3d55))
    - Release organize-rs_core v0.2.2, organize-rs v0.4.0 ([`ed623d7`](https://github.com/organize-rs/organize/commit/ed623d73fa574d4641e11e27cf6a2d4936ed5abe))
    - Release organize-rs_core v0.2.2, organize-rs v0.4.0 ([`27615a9`](https://github.com/organize-rs/organize/commit/27615a970fd946871a2c64f151490bc0269a45a5))
    - Release organize-rs_core v0.2.2, organize-rs v0.4.0 ([`8b71b45`](https://github.com/organize-rs/organize/commit/8b71b4546f100d0003c76f80f1d6c6d07615f74e))
    - Add a bit more documentation for filters and actions ([`e39e96c`](https://github.com/organize-rs/organize/commit/e39e96c5b5b08ecad0cb197c62f49fd0809faabf))
    - Add more doctests for filters ([`a7d737c`](https://github.com/organize-rs/organize/commit/a7d737c5cc9e6465c7cf724c5d020b0f7e8ad935))
    - Add some doctests for filters ([`a024a32`](https://github.com/organize-rs/organize/commit/a024a32327344ff109d8e6e68819f507d7c9e8ab))
    - Add some doctests for filters ([`28ad000`](https://github.com/organize-rs/organize/commit/28ad00049f14ec1761f26baf2fe2088854094900))
    - Fully annotate template config ([`930593f`](https://github.com/organize-rs/organize/commit/930593f55f0128609daf98ccee901d5e33c29223))
    - Add branch itself to `tag_applies` method ([`b633c56`](https://github.com/organize-rs/organize/commit/b633c561839314c760bf9a1a70e5b1aadac23b12))
    - Consider tags when running rules from config ([`f6bfca7`](https://github.com/organize-rs/organize/commit/f6bfca7440be8ee3f3aeed24b3164458cd2dcfe9))
    - Check for tags when running rules from config file ([`7c0b0c9`](https://github.com/organize-rs/organize/commit/7c0b0c9ce915e8b43d4542bc79b53432dd5bc2d2))
    - Cargo fix & cargo fmt ([`cb1e115`](https://github.com/organize-rs/organize/commit/cb1e1151c09afed48f40b989132ae89324f9f2b1))
    - Implement template generation ([`19110b9`](https://github.com/organize-rs/organize/commit/19110b9592fb35314402f9bac71f52791c5a7cd1))
    - Start annotating a template config file ([`0f49e89`](https://github.com/organize-rs/organize/commit/0f49e89282eaa8fc5f11a1db81553484e9a669b9))
    - Implement some snapshot testing ([`ad6f684`](https://github.com/organize-rs/organize/commit/ad6f6844dc93dac07e4d1921d1dbe0ab984b15c9))
    - Implement parsing rules from config files ([`94757b5`](https://github.com/organize-rs/organize/commit/94757b5e2eddb54b0a706634f408bb5fcabfaf94))
    - Chose better name for filter_groups ([`45182d5`](https://github.com/organize-rs/organize/commit/45182d5fa76f1716c98ce372aea4177ef6d3bb50))
    - Make lists comma separated in filters ([`b665490`](https://github.com/organize-rs/organize/commit/b665490feb2a7a44996847d80ee918ea82cc4772))
    - Fix naming in config ([`22192eb`](https://github.com/organize-rs/organize/commit/22192eb528c2f95ad01ef124082dfae518e8c464))
    - More state changes for runner ([`afb116b`](https://github.com/organize-rs/organize/commit/afb116bc91add3ac76f6c55e44932391eec6e7e8))
    - Implement first part of rule runner ([`3f6df32`](https://github.com/organize-rs/organize/commit/3f6df32eac07d369b035ffb06156c2b47b0f9121))
    - Move MAX_DEPTH to associated constant ([`2dec903`](https://github.com/organize-rs/organize/commit/2dec9033ce8187cdb74b23e2363d09d65131bbdd))
    - Make MaxDepth newtype  transparent to serde ([`ef8ea62`](https://github.com/organize-rs/organize/commit/ef8ea621bd5fcbdb24050204ef77eb2e330cf880))
    - First implementation draft for a config run ([`abdc5ec`](https://github.com/organize-rs/organize/commit/abdc5ecfe89c8db0b3f7077681ef3a23a7797df5))
    - Implement more of filesystem and action part ([`c2a5e01`](https://github.com/organize-rs/organize/commit/c2a5e017868e9f2af2bda3576b7e342356783722))
    - : add concurrency and filesystem functionality ([`67f506e`](https://github.com/organize-rs/organize/commit/67f506ea0f869bb0a55c1d3938d3d8b58612c979))
    - Units tests to be less repetitive ([`5b35248`](https://github.com/organize-rs/organize/commit/5b35248f532666f09dd8fd9308be1d16cb7c9f81))
    - More unit tests for matches_date ([`a52abf6`](https://github.com/organize-rs/organize/commit/a52abf6e5aa902c22c3fd7eebdd3088a2b3c6a50))
    - Implement unit tests for date matching ([`2abf8ea`](https://github.com/organize-rs/organize/commit/2abf8ea4fb19e8d1be640fb52abe1aba9c4e08c4))
    - To filetime for platform-agnostic atime, ctime, mtime inspection ([`f4a9cfa`](https://github.com/organize-rs/organize/commit/f4a9cfaa19d9f088768f2f705f14a400ad701a8b))
    - Add some thoughts about impl of actions ([`96c3d95`](https://github.com/organize-rs/organize/commit/96c3d950812a8df8cdccb53a47ae0e6165462117))
    - Add unit test for mimetype ([`db912c4`](https://github.com/organize-rs/organize/commit/db912c4ccfbc525ff4c5dccf231fda064e9a001e))
    - Fix unit test ([`2a75ebb`](https://github.com/organize-rs/organize/commit/2a75ebb8b239d79e02262e5aeb470c66b25afbb9))
    - Implement more unit tests for name filter ([`2d6ad42`](https://github.com/organize-rs/organize/commit/2d6ad427618ab76d5b73471dba62f6280218f63e))
    - Implement more test cases for name filter ([`31c790a`](https://github.com/organize-rs/organize/commit/31c790a1b244c5b98de9bba21b5ed9052d47d788))
    - Split filters in seperate modules ([`85fd452`](https://github.com/organize-rs/organize/commit/85fd452e00835f14e84da9547bb5659449771cde))
    - Add more unit tests for filters ([`db2d348`](https://github.com/organize-rs/organize/commit/db2d3480a1f965e861f92b8df04079461ee2f35d))
    - More deserialization attempts ([`c32b4b3`](https://github.com/organize-rs/organize/commit/c32b4b30c602b61d5ce94a037ccf2a496232c935))
    - Flatten library structure ([`8b47644`](https://github.com/organize-rs/organize/commit/8b47644af39c99f79658cdd54c067f9787b02347))
    - Parse first parts of py-organize config ([`91a8840`](https://github.com/organize-rs/organize/commit/91a8840c5826eb39493384a3e2d1bbb69f38c298))
    - Change Rule to contain new types ([`2c06e62`](https://github.com/organize-rs/organize/commit/2c06e62ede4f1de31c75dd786c65f87b0855dffc))
    - Add scripting and shell filter, add check for config and scripts subcommand, and run subcommands for config and scripts, add generate subcommand ([`54a8993`](https://github.com/organize-rs/organize/commit/54a8993981273a39fb0da07de5853a2fbc5764b4))
    - Release organize-rs_core v0.2.1, organize-rs v0.3.1 ([`e9689a9`](https://github.com/organize-rs/organize/commit/e9689a90133a48eaa76cd6d51aa4a8b3b496a868))
    - Add total entry count to output of filters ([`2c55b20`](https://github.com/organize-rs/organize/commit/2c55b20c24da830f5d81468450882f126c8d5531))
    - More functional syntax ([`0c79c6b`](https://github.com/organize-rs/organize/commit/0c79c6b5b75adc05c54fee49f3398ed871ffd9b3))
    - Switch to jwalk to walk the directory tree parallelized ([`18ee702`](https://github.com/organize-rs/organize/commit/18ee7029fa931b0f95046f11c2919b23f11c1470))
    - Implement custom serializer for MaxDepth ([`95d7740`](https://github.com/organize-rs/organize/commit/95d7740dfbda24d934eea7ecce21b39076cf251c))
    - Add associated constants for SizeRange and PeriodRange ([`847737a`](https://github.com/organize-rs/organize/commit/847737a5554f76a76366cd67fe8b2870277c935b))
    - Improve display ([`99d7b0c`](https://github.com/organize-rs/organize/commit/99d7b0c109017a78c1ef726dcc9d40d0507b0426))
    - Adding cli option for filter mode ([`d088c50`](https://github.com/organize-rs/organize/commit/d088c50c894a9a94a47b2e3b66502fa3d5fdfc59))
    - Make FilterWalker more configurable for upcoming features ([`a82a208`](https://github.com/organize-rs/organize/commit/a82a20821ebe60d386efcb9856f4d22b45ab4a2e))
    - Free the way for applying multiple filters ([`7c29918`](https://github.com/organize-rs/organize/commit/7c2991827f503c7caf6f82e964f9cc91f5c39d0e))
    - Fix borrow issues ([`398edb2`](https://github.com/organize-rs/organize/commit/398edb29916398c43b15531779233eda4b28eef5))
    - Apply multiple filters to walkdir iter ([`a316302`](https://github.com/organize-rs/organize/commit/a3163027800f1843476b8855a41f2115fdb3d3eb))
    - Make filters use more option/result methods ([`fd90047`](https://github.com/organize-rs/organize/commit/fd90047424eb9e6f04481a1ef35825e360b06912))
</details>

## 0.2.1 (2023-05-25)

<csr-id-4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4/>
<csr-id-09c428cc45bbb348ca08e5fd233c999408ca2500/>
<csr-id-48dd7daf79444deee11134c5c42fe4e9aa4e18ba/>
<csr-id-8504e820eca6dd8be66d707695e56d0c8c8f3be6/>
<csr-id-9d55a017ae9ba14b726fe542a9354fcf045b06f3/>
<csr-id-543150dd96aca886bacd0057bda1957d19b4322d/>
<csr-id-14300ea60bcccf500d813ee267792899a278a9ff/>
<csr-id-c005013d1c49f5d717d635c3ece760bb5c904e09/>

### New Features

 - <csr-id-12b5e9cb07d2258ac4a56ce7d38993802e6f7385/> add total entry count to output of filters
 - <csr-id-fea4cc14fe1f64cc9fd91664bf07f8940cba15a1/> adding cli option for filter mode
 - <csr-id-33fc910001ac35967bc1e424b110c88ace6b9186/> Free the way for applying multiple filters

### Other

 - <csr-id-4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4/> Switch to jwalk to walk the directory tree parallelized
 - <csr-id-09c428cc45bbb348ca08e5fd233c999408ca2500/> apply multiple filters to walkdir iter

### Refactor

 - <csr-id-48dd7daf79444deee11134c5c42fe4e9aa4e18ba/> more functional syntax
 - <csr-id-8504e820eca6dd8be66d707695e56d0c8c8f3be6/> implement custom serializer for MaxDepth
 - <csr-id-9d55a017ae9ba14b726fe542a9354fcf045b06f3/> Add associated constants for SizeRange and PeriodRange
 - <csr-id-543150dd96aca886bacd0057bda1957d19b4322d/> improve display
 - <csr-id-14300ea60bcccf500d813ee267792899a278a9ff/> Make FilterWalker more configurable for upcoming features
 - <csr-id-c005013d1c49f5d717d635c3ece760bb5c904e09/> make filters use more option/result methods

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release organize-rs_core v0.2.1, organize-rs v0.3.1 ([`2ff132c`](https://github.com/organize-rs/organize/commit/2ff132c615a868856281bffdfd84de35b848a365))
    - Add total entry count to output of filters ([`12b5e9c`](https://github.com/organize-rs/organize/commit/12b5e9cb07d2258ac4a56ce7d38993802e6f7385))
    - More functional syntax ([`48dd7da`](https://github.com/organize-rs/organize/commit/48dd7daf79444deee11134c5c42fe4e9aa4e18ba))
    - Switch to jwalk to walk the directory tree parallelized ([`4757013`](https://github.com/organize-rs/organize/commit/4757013ebe9fe5d37ea3d7b7cddf155910e0f5b4))
    - Implement custom serializer for MaxDepth ([`8504e82`](https://github.com/organize-rs/organize/commit/8504e820eca6dd8be66d707695e56d0c8c8f3be6))
    - Add associated constants for SizeRange and PeriodRange ([`9d55a01`](https://github.com/organize-rs/organize/commit/9d55a017ae9ba14b726fe542a9354fcf045b06f3))
    - Improve display ([`543150d`](https://github.com/organize-rs/organize/commit/543150dd96aca886bacd0057bda1957d19b4322d))
    - Adding cli option for filter mode ([`fea4cc1`](https://github.com/organize-rs/organize/commit/fea4cc14fe1f64cc9fd91664bf07f8940cba15a1))
    - Make FilterWalker more configurable for upcoming features ([`14300ea`](https://github.com/organize-rs/organize/commit/14300ea60bcccf500d813ee267792899a278a9ff))
    - Free the way for applying multiple filters ([`33fc910`](https://github.com/organize-rs/organize/commit/33fc910001ac35967bc1e424b110c88ace6b9186))
    - Fix borrow issues ([`3d1b0a1`](https://github.com/organize-rs/organize/commit/3d1b0a19e71441bdec6a9b609833b91a8ef890d8))
    - Apply multiple filters to walkdir iter ([`09c428c`](https://github.com/organize-rs/organize/commit/09c428cc45bbb348ca08e5fd233c999408ca2500))
    - Make filters use more option/result methods ([`c005013`](https://github.com/organize-rs/organize/commit/c005013d1c49f5d717d635c3ece760bb5c904e09))
</details>

## 0.2.0 (2023-05-21)

### New Features (BREAKING)

 - <csr-id-13f7560c1deb053fb74880247d01b60be6ef0ca1/> implement range syntax also for date related filters

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release organize-rs_core v0.2.0, safety bump organize-rs v0.3.0 ([`fb27edc`](https://github.com/organize-rs/organize/commit/fb27edc1f40e49e4db6d8553ebb1317feb0cb8be))
    - Implement range syntax also for date related filters ([`13f7560`](https://github.com/organize-rs/organize/commit/13f7560c1deb053fb74880247d01b60be6ef0ca1))
</details>

## 0.1.6 (2023-05-21)

### New Features

 - <csr-id-da1d07a58a6ccafc1f930132eb0cb4c182af9569/> Implement `size` filter
 - <csr-id-0cf330e125e4154f71cef3c990ab574928cf49d1/> Implement parser for byte size conditions for `size` filter

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release organize-rs_core v0.1.6 ([`fc39cd1`](https://github.com/organize-rs/organize/commit/fc39cd180ae6c20023e3a821f2156eaf953a8729))
    - Merge branch 'develop' ([`7bf59e8`](https://github.com/organize-rs/organize/commit/7bf59e8bfb8da7033192034153a9216a2db185c4))
    - Implement `size` filter ([`da1d07a`](https://github.com/organize-rs/organize/commit/da1d07a58a6ccafc1f930132eb0cb4c182af9569))
    - Implement parser for byte size conditions for `size` filter ([`0cf330e`](https://github.com/organize-rs/organize/commit/0cf330e125e4154f71cef3c990ab574928cf49d1))
</details>

## 0.1.5 (2023-05-20)

### New Features

 - <csr-id-345d8885d1ffe9bcfdc42c62fccbdc59a457ed0a/> Implement `mimetype` filter

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 5 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release organize-rs_core v0.1.5, organize-rs v0.2.5 ([`6de8935`](https://github.com/organize-rs/organize/commit/6de893599162bbbb838c7c5f5fd3a81536cb9b30))
    - Implement `mimetype` filter ([`345d888`](https://github.com/organize-rs/organize/commit/345d8885d1ffe9bcfdc42c62fccbdc59a457ed0a))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.4, organize-rs v0.2.4 ([`b00bbe0`](https://github.com/organize-rs/organize/commit/b00bbe03cc7b50a08dcb2e6c98eb41a3a586f488))
    - Implement `last_accessed` and `last_modified` filters ([`4410c3f`](https://github.com/organize-rs/organize/commit/4410c3f8a45909c69a3fdca63ad8f6845b601dc3))
    - Implement `created` filter ([`f07ab6a`](https://github.com/organize-rs/organize/commit/f07ab6a4bd9be7674dad416f7b74e9b9196b3dca))
    - Remove human-panic dependency ([`9382256`](https://github.com/organize-rs/organize/commit/938225668c8879192a8e81a4872797907e3b4641))
    - Remove unused import ([`9f56d4c`](https://github.com/organize-rs/organize/commit/9f56d4ce1211abaf96f720c9874b1bba1915d755))
    - Cargo fix & cargo fmt ([`ee231a6`](https://github.com/organize-rs/organize/commit/ee231a69fcd825e6121c380f408c21ff2bf6c425))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.3, organize-rs v0.2.3 ([`c4d5428`](https://github.com/organize-rs/organize/commit/c4d5428c29ca7bf24746abf8ff45c74a4838159d))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.2, organize-rs v0.2.2 ([`2ebfdd7`](https://github.com/organize-rs/organize/commit/2ebfdd7fe2d386f54104f1a0a0d0230fd793f271))
    - Implement `empty` filter and global ignore for file names and directory paths ([`d51a81a`](https://github.com/organize-rs/organize/commit/d51a81a593cb37c54c0c91edfac60a5eb8de7c89))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.1, organize-rs v0.2.1 ([`1e0e2dc`](https://github.com/organize-rs/organize/commit/1e0e2dc36593da72422817d50eff61f13444ea32))
    - Update Versions ([`bb0cbce`](https://github.com/organize-rs/organize/commit/bb0cbce647d6f864100e81d1ac65a50fecb614c7))
    - Fix file_stem case insensitivity ([`03509fe`](https://github.com/organize-rs/organize/commit/03509fe7f6eedf5a94253a2e8e094c47ba114f69))
    - Add description to core lib ([`3b83e8e`](https://github.com/organize-rs/organize/commit/3b83e8ec10c2cbf24c7c35923aba9fc75687921c))
    - Update workspace manifests ([`d4eba0d`](https://github.com/organize-rs/organize/commit/d4eba0d0052f1d114bd7988edacfc5e53a62e4a9))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.0, organize-rs v0.2.0 ([`2dbdfe3`](https://github.com/organize-rs/organize/commit/2dbdfe3b7b2b656075901a013a587f6e6d4883cf))
    - Adjusting changelogs prior to release of organize-rs_core v0.1.0, organize-rs v0.2.0 ([`c008e22`](https://github.com/organize-rs/organize/commit/c008e22937993460a2997c33c13410625c25fc5b))
    - Add Changelogs ([`3b0ccbd`](https://github.com/organize-rs/organize/commit/3b0ccbda803b2a51cb5a33a03e8c9a382caeae96))
    - Update readme and version ([`287b420`](https://github.com/organize-rs/organize/commit/287b420a81ad730f79d90250c6fde2c7dda6f662))
    - Fix `name` filter `--ends_with` to include `file_stem` ([`af0efed`](https://github.com/organize-rs/organize/commit/af0efed87cabec1c5d7cf465452511008b2e475a))
    - Implement `name` filter ([`88587b0`](https://github.com/organize-rs/organize/commit/88587b09d28b0e1d8dfb9161086308edb19fbfbc))
    - Fix some grouping issues in Cli, create `cli` feature in organize-rs_core ([`b734e62`](https://github.com/organize-rs/organize/commit/b734e625d869163b07f63923414ffa900f93ca64))
    - Implement `filter_by_extension` ([`45e4d5b`](https://github.com/organize-rs/organize/commit/45e4d5b03185d5cd016d16795fdba0336c1496bd))
    - First try for implementing a file extension filter ([`45f2966`](https://github.com/organize-rs/organize/commit/45f296647ea46461ec89550f48eb22e07c037d5c))
    - Implement stub for filter methods ([`6c6f0f8`](https://github.com/organize-rs/organize/commit/6c6f0f89709a5f7b78ad8de3099ac3cbd6c5f6e3))
    - Add boilerplate for matching filters and actions ([`83c8cbc`](https://github.com/organize-rs/organize/commit/83c8cbc4dbf6e54b1941b8d1c5eabf399932845c))
    - Add czkawka_core dependency ([`5063aec`](https://github.com/organize-rs/organize/commit/5063aecdd41b99534d7c2539bcd60a5756401110))
    - Refine commands/subcommands ([`ed535f6`](https://github.com/organize-rs/organize/commit/ed535f68f92e4ec187a73fb628fcf4e86d1bda3e))
    - Rethink structure and change roadmap ([`f5df157`](https://github.com/organize-rs/organize/commit/f5df157a0ee6944c398450d5651965999ec7f1f3))
    - Add `actions` and `filters` as subcommands ([`60df64e`](https://github.com/organize-rs/organize/commit/60df64e3380870fb5182e9cd4f47bb792bc55ce7))
    - Start parsing config ([`0e36272`](https://github.com/organize-rs/organize/commit/0e36272f9e7db8e65daaad39d228d986ab952673))
    - Refactor to workspace and create new core library ([`0de540b`](https://github.com/organize-rs/organize/commit/0de540b0aa0ab07dc4f3b118e6f95b30312ea44e))
</details>

## 0.1.4 (2023-05-20)

## 0.1.3 (2023-05-19)

## 0.1.2 (2023-05-19)

## 0.1.1 (2023-05-18)

## 0.1.0 (2023-05-18)

