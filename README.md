# reindeer-local-buildrs-issue-demo

This is a demo repository related to the issue

Created for verifying whether build scripts for local crates behave correctly

## Usage

**1. Clone this repository:**

```bash
git clone https://github.com/yxdai-nju/buck2-proc-macro-issue-demo.git --recurse-submodules
```

**2. Run this command:**

```bash
./run_reindeer.sh && buck2 clean && buck2 run //project:my_bin
```

**3. Check the results:**

First, check the command-line output. There can be 3 possibilities:
- *"build.rs did not run before compilation"* – this shouldn't happen
- *"build.rs ran before compilation but did not detect the static/resource.txt file"* – indicating that the filegroup for `my_buildrs` is possibly missing
- *"build.rs ran before compilation and successfully detected the static/resource.txt file"* – the desired result

If you get the third output, it indicates that the extra filegroup for case #1 (local crates) is working.

Second, examine the content of `buck-out/v2/gen/root/[hash]/project/__thiserror-2.0.12-build-script-run__/cwd`. The desired outcome is for it to be non-empty, with `build.rs` sitting directly under this directory (no further subdirectories).

If you see this outcome, it indicates that the extra filegroup for case #2 (git patches) is working.

## Structure

- `project` is a Rust workspace root
- `project/crates/my_buildrs` is a workspace member, providing a library target with its own build.rs
- `project/crates/my_bin` is a workspace member, providing a binary target that depends on `my_buildrs`
- `project/fixups/**/fixups.toml` are fixups for both third-party crates and local (workspace) crates
- `project/{Cargo.toml,reindeer.toml}` set up Reindeer
- `.buckconfig` uses `buck2/prelude` for the prelude cell

The `my_buildrs` crate has a `build.rs` which sets rustc flags depending on whether a resource file exists during the buildscript run. The `lib.rs` then provides a function that prints different messages depending on which flag is set. The `my_bin` crate later invokes this function.
