# Project Title Release Process

This covers how to do an official release of a package from this repository.

# Roles:

1. Maintainer / Release Manager - People running and managing the release process
2. Owner - Engineer who owns the PR
3. Reviewer - Engineer who does not own the PR and provides a code review for the PR

# Full Release Process

## For Owner

1. For every `Issue` / Scrum Card (User Story), create a branch in `<TYPE>/<DESCRIPTION>` format
    1. Types: `feat`, `fix`, `docs`, `chore`, `refactor` , `test` 
    2. Example: `feat/proxy-program` 
2. Do **signed commits** of your progress with your branch
    1. `git commit -S`
    2. [Learn more about Git signing](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)
3. Once done open a `Pull Request` against `main` following the repo’s `Pull Request` format
4. Tag another engineer in the team who has the most context about your work to take a look at it and do a `code review` 
5. If the reviewer asks for an action, address the reviewer’s needs and repeat `Step 4`

## For Reviewer

> [!WARNING]
> Make sure to look out for **feature flags** when reviewing incomplete feature PRs. Usually applied for big EPICs.

1. When tagged in a `Pull Request` review the code as soon as possible
    1. Baseline requirement is to ensure `Pull Request` follows the repo’s `Pull Request` format
2. If upon reviewing an action is needed, mention `Pull Request` **owner** to take action
3. If upon reviewing the PR is ready for merging, mention `Maintainer` with approval to initiate `Merge` 

## For Maintainer

> [!WARNING]
> Prioritize to **Release Early**, **Release Often** (RERO) and avoid longstanding Pull Requests / WIP Branches / Release Candidates

### Trunk Merge (`main`)

1. Trunk merges can happen any time as long as incomplete features / breaking changes are kept behind feature flags
2. If prompted / mentioned by Owner / Reviewer, assess if `Pull Request` follows the [Trunk Merge Definition of Done](https://www.notion.so/Release-Plan-and-Management-1232d67d7b5f809aa257c4943d724281?pvs=21)
3. Merge as soon as Definition of Done is met

### Stable Releasing

1. Every week Maintainer will assess with the team / community if there are anything worth releasing to stable. If there is, initiate a Stable Release process to achieve `Definition of Done`
2. For every week, if one isn’t open yet, open a stable release candidate branch named the semantic versioning of the upcoming stable release (e.g. `v0.1.1`)
3. After creating the release candidate branch, park an open `Pull Request` with the same name as the release candidate’s branch + “Release Candidate” (e.g. `v0.1.1 Release Candidate`)
4. This `Pull Request` is used for tracking the changes that is slated for a Stable Release
5. Merge trunk (`main`) to release candidate branch as soon as you and team agree that there’s anything worth releasing to stable
    1. New changes from trunk can be merged to release candidate in the middle of a review so long as features/breaking changes/incomplete changes are behind feature flags
6. Create a User Acceptance Test around the User Story / Epic
7. Complete UAT by 2 QA / Users
8. If failing, apply fixes on `trunk` and merge back into release candidate branch
9. If passing, create a Stable Release named after the Semantic Versioning (e.g. `v0.1.1` ) and tag it with its Semantic Versioning as well (e.g. `v0.1.1`)

### Nightly Releasing

1. Create a `nightly-<COMMIT HASH>` release from trunk (`main`) every 10PM UTC+8 and tag it with `nightly-<COMMIT HASH>`.
    1. Can also be automated as long as a `nightly` release is created every day at 10PM UTC+8


# Definition of Done

## Ready for Trunk (`main`) merge

- Pull Request follows proper format and contains required information
- 1 Code Reviewer Done

## Ready for Release

### Stable Release

- Internal feature flags should be removed and new stable features should now be exposed (unless it’s truly meant to be an optional feature)
- 2 Passing User Acceptance Tests (UAT) done for Epic / User Story
- Related documentation to User Story / Epic is done and ready for publishing

### Nightly Release

- Anything pushed to the main branch is considered DONE and eligible for a nightly release
