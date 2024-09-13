# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.4 (2024-09-14)

### Refactor

 - <csr-id-e47c73615465a8efd64bf67cc494c692ee5fabad/> Exhaustively match each meta attribute per field type
   While this approach ends up as way more code, it is much more legible
   and reasonable.
   
   Other types will still require work for a similar implementation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Exhaustively match each meta attribute per field type ([`e47c736`](https://github.com/zexa/de_hypertext/commit/e47c73615465a8efd64bf67cc494c692ee5fabad))
</details>

## v0.1.3 (2024-09-12)

<csr-id-ce5fbdf62da0472e920aad34bd49e6b887cc7d12/>

### Chore

 - <csr-id-ce5fbdf62da0472e920aad34bd49e6b887cc7d12/> bump to 0.1.3

### Bug Fixes

 - <csr-id-a60a53d8a8440fc32af5973f3e9a6872404e0836/> missing let selector on derive Option<T>
   rebase me with option_t fixes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release de_hypertext_core v0.1.3, de_hypertext v0.1.3 ([`cd782a5`](https://github.com/zexa/de_hypertext/commit/cd782a541154017cf7906dc1b193869ec732fc95))
    - Release de_hypertext_core v0.1.3, de_hypertext v0.1.3 ([`f3f21e7`](https://github.com/zexa/de_hypertext/commit/f3f21e79cb0df4deaa88e5dd18153c354ddb8306))
    - Bump to 0.1.3 ([`ce5fbdf`](https://github.com/zexa/de_hypertext/commit/ce5fbdf62da0472e920aad34bd49e6b887cc7d12))
    - Missing let selector on derive Option<T> ([`a60a53d`](https://github.com/zexa/de_hypertext/commit/a60a53d8a8440fc32af5973f3e9a6872404e0836))
</details>

## v0.1.2 (2024-09-09)

<csr-id-18869b43e2eb4d73b335857c975425079902ecbd/>

### New Features

 - <csr-id-b55468da37bec90a88a710b76562ea3218f8b7d0/> replace trim with transform

### Bug Fixes

 - <csr-id-0d1841eba4f4b58ea1580dc7781a8c1e5f362509/> Vec<T> implementation using .next()

### Refactor

 - <csr-id-18869b43e2eb4d73b335857c975425079902ecbd/> Remove generic from Deserializer

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 5 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release de_hypertext_core v0.1.2, de_hypertext v0.1.1 ([`7d83c5e`](https://github.com/zexa/de_hypertext/commit/7d83c5e086df84aade1dedba9955841733fef6ea))
    - Release de_hypertext_core v0.1.2, de_hypertext v0.1.1 ([`459c79b`](https://github.com/zexa/de_hypertext/commit/459c79b76def84acc26c274b55d066cfd06ff4fb))
    - Replace trim with transform ([`b55468d`](https://github.com/zexa/de_hypertext/commit/b55468da37bec90a88a710b76562ea3218f8b7d0))
    - Remove generic from Deserializer ([`18869b4`](https://github.com/zexa/de_hypertext/commit/18869b43e2eb4d73b335857c975425079902ecbd))
    - Vec<T> implementation using .next() ([`0d1841e`](https://github.com/zexa/de_hypertext/commit/0d1841eba4f4b58ea1580dc7781a8c1e5f362509))
</details>

## v0.1.1 (2024-09-04)

<csr-id-511a74f54a9d60e9b3f1392d4c641f0067177813/>
<csr-id-38fbf13d45119aa5e09c3e0439e8694c5af7c19d/>
<csr-id-c9ca4f04b4709afccd0bc3b0face685e17ce5741/>
<csr-id-397eb04326de526a60709b46c766d45761266350/>
<csr-id-90fde25ef3f79b4da709fab7839a6cef5787236d/>
<csr-id-1ec7c39b85d73f9a6e26fcbe71e7515149e5364c/>

### Chore

 - <csr-id-511a74f54a9d60e9b3f1392d4c641f0067177813/> add changelogs
 - <csr-id-38fbf13d45119aa5e09c3e0439e8694c5af7c19d/> add description

### New Features

 - <csr-id-76bf125a6302d1dc4f3d244a9284f566f6049107/> no selector & attr without selector

### Bug Fixes

 - <csr-id-8feb05abde654dc4d83a4c25f6d91e4c4daf4e8c/> remove unnecessary curly brackets

### Refactor

 - <csr-id-c9ca4f04b4709afccd0bc3b0face685e17ce5741/> decrease some nesting
 - <csr-id-397eb04326de526a60709b46c766d45761266350/> move tests to their own crate
 - <csr-id-90fde25ef3f79b4da709fab7839a6cef5787236d/> make the macro testable
 - <csr-id-1ec7c39b85d73f9a6e26fcbe71e7515149e5364c/> Use DeserializeError instead of Box<dyn Error> everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release de_hypertext_core v0.1.1, de_hypertext_macro v0.1.1, de_hypertext v0.1.1 ([`b7e579e`](https://github.com/zexa/de_hypertext/commit/b7e579ea177c8a8899107a5efb4b3aa56c7a1819))
    - Add changelogs ([`511a74f`](https://github.com/zexa/de_hypertext/commit/511a74f54a9d60e9b3f1392d4c641f0067177813))
    - No selector & attr without selector ([`76bf125`](https://github.com/zexa/de_hypertext/commit/76bf125a6302d1dc4f3d244a9284f566f6049107))
    - Decrease some nesting ([`c9ca4f0`](https://github.com/zexa/de_hypertext/commit/c9ca4f04b4709afccd0bc3b0face685e17ce5741))
    - Remove unnecessary curly brackets ([`8feb05a`](https://github.com/zexa/de_hypertext/commit/8feb05abde654dc4d83a4c25f6d91e4c4daf4e8c))
    - Move tests to their own crate ([`397eb04`](https://github.com/zexa/de_hypertext/commit/397eb04326de526a60709b46c766d45761266350))
    - Make the macro testable ([`90fde25`](https://github.com/zexa/de_hypertext/commit/90fde25ef3f79b4da709fab7839a6cef5787236d))
    - Add description ([`38fbf13`](https://github.com/zexa/de_hypertext/commit/38fbf13d45119aa5e09c3e0439e8694c5af7c19d))
    - Use DeserializeError instead of Box<dyn Error> everywhere ([`1ec7c39`](https://github.com/zexa/de_hypertext/commit/1ec7c39b85d73f9a6e26fcbe71e7515149e5364c))
    - Quote de_hypertext instead of de_hypertext_* ([`5b6b87c`](https://github.com/zexa/de_hypertext/commit/5b6b87c8ab92e8dd1f802a03a02824efb1d43a9c))
    - Refactor core nested example to reflect macro ([`0021279`](https://github.com/zexa/de_hypertext/commit/00212793a7e154254a00a7b34ce48200e1e8fa20))
    - Remove old docs ([`aa416cc`](https://github.com/zexa/de_hypertext/commit/aa416cc9a16e997f204e382a49bd75d933cf26b2))
    - Remove unused DeserializeError::MissingDocument ([`0fbde85`](https://github.com/zexa/de_hypertext/commit/0fbde85bd4c1342cba0efd45ccd54609cd6601cf))
    - Initial commit ([`a75abf1`](https://github.com/zexa/de_hypertext/commit/a75abf164fdd5020927b3065c5a2b065f16c888d))
</details>

## v0.1.0 (2024-09-04)

<csr-id-38fbf13d45119aa5e09c3e0439e8694c5af7c19d/>
<csr-id-c9ca4f04b4709afccd0bc3b0face685e17ce5741/>
<csr-id-397eb04326de526a60709b46c766d45761266350/>
<csr-id-90fde25ef3f79b4da709fab7839a6cef5787236d/>
<csr-id-1ec7c39b85d73f9a6e26fcbe71e7515149e5364c/>

### Chore

 - <csr-id-38fbf13d45119aa5e09c3e0439e8694c5af7c19d/> add description

### New Features

 - <csr-id-76bf125a6302d1dc4f3d244a9284f566f6049107/> no selector & attr without selector

### Bug Fixes

 - <csr-id-8feb05abde654dc4d83a4c25f6d91e4c4daf4e8c/> remove unnecessary curly brackets

### Refactor

 - <csr-id-c9ca4f04b4709afccd0bc3b0face685e17ce5741/> decrease some nesting
 - <csr-id-397eb04326de526a60709b46c766d45761266350/> move tests to their own crate
 - <csr-id-90fde25ef3f79b4da709fab7839a6cef5787236d/> make the macro testable
 - <csr-id-1ec7c39b85d73f9a6e26fcbe71e7515149e5364c/> Use DeserializeError instead of Box<dyn Error> everywhere

