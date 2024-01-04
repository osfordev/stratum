# OS For Developers Stratum Tool

TBD

## Get Started

TBD

## Development

### Repository Structure

This is workspace branch of multi project repository based on [orphan](https://git-scm.com/docs/git-checkout#Documentation/git-checkout.txt---orphanltnew-branchgt) branches.

Branches (sub-projects):

* `rust` - Rust implementation of Stratum Tool (and a dynamic system library)
* `shell` - Shell implementation of Stratum Tool (experemental)

### Initialize Workspace

1. Clone the repository
	```shell
	git clone git@github.com:osfordev/stratum.git osfordev/stratum
	```
1. Enter into cloned directory
	```shell
	cd osfordev/stratum
	```
1. Initialize [worktree](https://git-scm.com/docs/git-worktree) by execute following commands:
	```shell
	for BRANCH in rust shell; do git worktree add "${BRANCH}" "${BRANCH}"; done
	```
1. Open VSCode Workspace
	```shell
	code "OS For Developers Stratum.code-workspace"
	```


### Notes

Add new orphan branch

```shell
NEW_BRANCH=...
git worktree add --detach "./${NEW_BRANCH}"
cd "./${NEW_BRANCH}"
git checkout --orphan "${NEW_BRANCH}"
git reset --hard
git commit --allow-empty -m "Initial Commit"
git push origin "${NEW_BRANCH}":"${NEW_BRANCH}"
```
