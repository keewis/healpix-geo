# maintainer's guide

## continuous integration

To save compute time, CI is triggered only when the relevant language was impacted by a PR. More concretely:

- rust CI will only run when `$project_root/Cargo.*` or `$project_root/healpix-geo/*` was modified
- python CI will run when the rust CI runs, and additionally when `$project_root/healpix-geo-python/*` was modified
- ReadTheDocs runs will be skipped except on `main` or on tags, or when the python CI runs
- js CI will run when the rust CI runs, and additionally when `$project_root/healpix-geo-js/*` was modified

This behavior can be changed by adding labels or commit message tags (text enclosed in brackets in the first line of the commit message):

- adding a `[run-rtd]` commit message tag will always run RTD on that commit
- adding a `[skip-rtd]` commit message tag will skip RTD on that commit (`[run-rtd]` is prioritized)
- adding a `[skip-ci]` commit message will skip all CI on that commit
- adding a `skip-ci` label to the PR will skip all github CI for the entire PR

## releases

To trigger a release, create a release through the "draft releases" page of github.

Releases can happen independently per language, as long as the name of the tag matches `{language}-v*` where language is currently one of `python` and `js`. This will upload a version to the language-specific package index (PyPI for `python` and npmjs.org for `js`). npmjs.org packages are uploaded to the staging area and need to be manually confirmed through the website.

After the release is confirmed, activate a version build on ReadTheDocs (click on "add version" in the versions tab) and rename the slug from `python-v{version}` to just `v{version}`. If anything went wrong with the build, it is fine to push another tag called `python-docs-v{version}` (or anything, really, it should just be different from the release pattern above). Be aware that in that case you may have to choose a different slug such as `v{version}post0`.

Usually a version update will be triggered on conda-forge within a day, but if not (or you need to be faster than that), use a bot command issue to trigger a version update. You'll find a list of bot commands by choosing the "bot commands" issue template. Also be aware that "when in doubt, rerender" (through another bot command) can resolve a lot of conda-forge build issues.
