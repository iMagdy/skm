# Developer Certificate of Origin

This project uses the Developer Certificate of Origin, also known as the DCO.

The DCO is a lightweight way for contributors to certify that they have the right to submit their contribution to the project under the project's open source license.

By contributing to this project, you certify that your contribution satisfies the official Developer Certificate of Origin 1.1.

## Required sign-off

Every commit contributed to this project must include a `Signed-off-by` line.

Use:

```bash
git commit -s -m "Your commit message"
```

This adds a line like:

```text
Signed-off-by: Your Name <your.email@example.com>
```

The name and email in the sign-off should identify the contributor who made the certification.

## What the sign-off means

By signing off a commit, you certify that at least one of the following is true:

- You created the contribution and have the right to submit it under the project's open source license.
- The contribution is based on previous work that is compatible with the project's open source license, and you have the right to submit it.
- The contribution was provided to you by someone who certified the same rights, and you are passing it on without removing that certification.

You also understand that the contribution and its sign-off may become part of the public project history and may be redistributed under the project's license.

## Fixing a missing sign-off

If your latest commit is missing a sign-off, run:

```bash
git commit --amend --signoff
git push --force-with-lease
```

If multiple commits are missing sign-offs, you can rebase and sign each commit:

```bash
git rebase --signoff HEAD~N
git push --force-with-lease
```

Replace `N` with the number of commits you need to update.

## Pull request requirement

Pull requests may not be merged unless all commits include a valid `Signed-off-by` line.

The maintainers may ask contributors to update commits before review or merge.

## Not the same as cryptographic commit signing

DCO sign-off is not the same as GPG, SSH, or verified commit signing.

DCO sign-off is a certification line in the commit message.

Cryptographic signing verifies commit identity. DCO sign-off certifies contribution rights.

Both may be useful, but this project requires DCO sign-off unless the maintainers explicitly say otherwise.
