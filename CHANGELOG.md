# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

<csr-id-b903cf4956dd89a6999bfb83d46b814822e6b240/>
<csr-id-3164ba3347f3bc82b88bb88db29e51d2b13bc363/>
<csr-id-82c926831fc6c8a5c67e6453555cfb6624d60700/>
<csr-id-7d8da29c8102787d05b2c565a14c5dba61b2f10b/>
<csr-id-74fbbaf94e518acc6cbaf1dcc8196a7922710214/>
<csr-id-f58dfa77507a33330158973f772bcec09574cb3f/>
<csr-id-6fb66df4aa42537aaf7f415233e02dc6cfd647b8/>

### Chore

 - <csr-id-b903cf4956dd89a6999bfb83d46b814822e6b240/> prepare to publish to crates.io
 - <csr-id-3164ba3347f3bc82b88bb88db29e51d2b13bc363/> add license
 - <csr-id-82c926831fc6c8a5c67e6453555cfb6624d60700/> add description, readme, repository to Cargo.toml
 - <csr-id-7d8da29c8102787d05b2c565a14c5dba61b2f10b/> add rustfmt
 - <csr-id-74fbbaf94e518acc6cbaf1dcc8196a7922710214/> cargo init
 - <csr-id-f58dfa77507a33330158973f772bcec09574cb3f/> add README
 - <csr-id-6fb66df4aa42537aaf7f415233e02dc6cfd647b8/> add gitignore

### Chore

 - <csr-id-aa01da2a5371b2f1d04e2d24cd72dba86aa386f2/> bump version to v0.2.1
 - <csr-id-a7fddb9134e77929fb5a8399c89a167cda2a5e99/> bump version to v0.2.0-pre

### New Features (BREAKING)

 - <csr-id-c5e19c84183a42412fc862898283962561d908f5/> remove is_empty method and return non-zero length
   - fix doc comments

### Documentation

 - <csr-id-0b86ae0651febfa814f7df008cae7d2bf582695d/> show methods behind feature flag bytes

### New Features

 - <csr-id-ad7e532ac221e4c1bbc4673903e3aff1e1e75381/> implement NonEmptyBz
 - <csr-id-a5a5620f484361ddad58c0beb7dbe6639a6c487d/> add cloned method
 - <csr-id-d9c5b1cefb8c2b78ff88b95825f74124b4332607/> add infalliable first-last accessor methods
   - provide `first()`, `last()`, `split_first()`, `split_last()` methods.
 - <csr-id-41c416c63bdeace91082bfeeb30380756f5aa288/> add support to obtain non empty byte slice from non empty borrowed array

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to v0.2.1 (aa01da2)
    - Merge pull request #13 from hubcycle/12-nebz-borrowed-array-as-slice (2230085)
    - Bump version to v0.2.0-pre (a7fddb9)
    - Add support to obtain non empty byte slice from non empty borrowed array (41c416c)
    - Merge pull request #11 from hubcycle/release/v0.2.0 (223e5cc)
</details>

<csr-unknown>
use unsafe blocks for performance, and throughly document the safety.add tests and miri workflow to ci.<csr-unknown/>

## 0.1.2 (2025-07-18)

<csr-id-5c51d53a14360aa051003275f89859c7deca5e07/>

### Chore

 - <csr-id-5c51d53a14360aa051003275f89859c7deca5e07/> add changelog for v0.1.2

### New Features

 - <csr-id-d9c5b1cefb8c2b78ff88b95825f74124b4332607/> add infalliable first-last accessor methods
   - provide `first()`, `last()`, `split_first()`, `split_last()` methods.
- use unsafe blocks for performance, and throughly document the safety.
- add tests and miri workflow to ci.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add changelog for v0.1.2 (5c51d53)
    - Merge pull request #7 from hubcycle/feature/6-infalliable-first-last (f7a5cad)
    - Add infalliable first-last accessor methods (d9c5b1c)
    - Merge pull request #5 from hubcycle/release/v0.1.1 (1c36b38)
</details>

## 0.1.1 (2025-07-18)

<csr-id-f8cfcfeb5eb7a1799e8e30f0bf60774722a6bab7/>

### Chore

 - <csr-id-f8cfcfeb5eb7a1799e8e30f0bf60774722a6bab7/> add changelog

### Documentation

 - <csr-id-0b86ae0651febfa814f7df008cae7d2bf582695d/> show methods behind feature flag bytes

### New Features

 - <csr-id-a5a5620f484361ddad58c0beb7dbe6639a6c487d/> add cloned method

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add changelog (f8cfcfe)
    - Merge pull request #4 from hubcycle/dev (94fc052)
    - Show methods behind feature flag bytes (0b86ae0)
    - Merge pull request #3 from hubcycle/dev (df471fa)
    - Add cloned method (a5a5620)
</details>

## 0.2.0 (2025-07-19)

### Chore

 - <csr-id-32fb3e7bd087641eb6bef672f9947f36a571c03a/> add changelog for v0.2.0

### New Features (BREAKING)

 - <csr-id-c5e19c84183a42412fc862898283962561d908f5/> remove is_empty method and return non-zero length
   - fix doc comments

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add changelog for v0.2.0 (32fb3e7)
    - Merge pull request #10 from hubcycle/9-remove-is_empty-and-return-non-zero-length (8a6bd1e)
    - Remove is_empty method and return non-zero length (c5e19c8)
    - Merge pull request #8 from hubcycle/release/v0.1.2 (4aada5a)
</details>

## 0.1.0 (2025-07-18)

<csr-id-6923fd24ed8c89b262c326f57897eecd379c420c/>
<csr-id-b903cf4956dd89a6999bfb83d46b814822e6b240/>
<csr-id-3164ba3347f3bc82b88bb88db29e51d2b13bc363/>
<csr-id-82c926831fc6c8a5c67e6453555cfb6624d60700/>
<csr-id-7d8da29c8102787d05b2c565a14c5dba61b2f10b/>
<csr-id-74fbbaf94e518acc6cbaf1dcc8196a7922710214/>
<csr-id-f58dfa77507a33330158973f772bcec09574cb3f/>
<csr-id-6fb66df4aa42537aaf7f415233e02dc6cfd647b8/>

### Chore

 - <csr-id-6923fd24ed8c89b262c326f57897eecd379c420c/> add changelog
 - <csr-id-b903cf4956dd89a6999bfb83d46b814822e6b240/> prepare to publish to crates.io
 - <csr-id-3164ba3347f3bc82b88bb88db29e51d2b13bc363/> add license
 - <csr-id-82c926831fc6c8a5c67e6453555cfb6624d60700/> add description, readme, repository to Cargo.toml
 - <csr-id-7d8da29c8102787d05b2c565a14c5dba61b2f10b/> add rustfmt
 - <csr-id-74fbbaf94e518acc6cbaf1dcc8196a7922710214/> cargo init
 - <csr-id-f58dfa77507a33330158973f772bcec09574cb3f/> add README
 - <csr-id-6fb66df4aa42537aaf7f415233e02dc6cfd647b8/> add gitignore

### New Features

 - <csr-id-ad7e532ac221e4c1bbc4673903e3aff1e1e75381/> implement NonEmptyBz

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #2 from hubcycle/dev (19147c7)
    - Add changelog (6923fd2)
    - Merge pull request #1 from hubcycle/dev (4434803)
    - Prepare to publish to crates.io (b903cf4)
    - Implement NonEmptyBz (ad7e532)
    - Add license (3164ba3)
    - Add description, readme, repository to Cargo.toml (82c9268)
    - Add rustfmt (7d8da29)
    - Cargo init (74fbbaf)
    - Add README (f58dfa7)
    - Add gitignore (6fb66df)
</details>

