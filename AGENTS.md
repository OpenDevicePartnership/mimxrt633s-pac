# AGENTS.md — mimxrt633s-pac

This file is the entry point for AI coding agents (GitHub Copilot, Claude,
Cursor, Aider, etc.) and for human contributors who want a single, concise
description of how this repository expects work to be done. It is a strict
superset of `.github/copilot-instructions.md`; agents that previously consumed
only `copilot-instructions.md` should now prefer this file.

If anything in this file disagrees with the source code or with CI, the
**source code and CI are authoritative**. Update this file in the same
commit that changes the underlying convention.

---

## 1. What this crate is

`mimxrt633s-pac` is a **Peripheral Access Crate (PAC)** for the NXP
i.MX RT600 series part `MIMXRT633S`. It is consumed by higher-level HAL
crates (notably `embassy-imxrt`) to provide typed, register-level access to
on-chip peripherals.

- Crate name: `mimxrt633s-pac`
- Edition: 2021
- MSRV: `1.76` (enforced by CI; see `Cargo.toml` `rust-version` and
  `.github/workflows/check.yml` `msrv` job)
- License: MIT
- Target environment: `no_std`, ARM Cortex-M33 (`thumbv8m.main-none-eabihf`)
- Upstream repository: <https://github.com/OpenDevicePartnership/mimxrt633s-pac>

### Features

Defined in `Cargo.toml`:

| Feature | Effect |
|---------|--------|
| `rt`    | Pull in `cortex-m-rt/device` and install `device.x` link script via `build.rs`. |
| `defmt` | Implement `defmt::Format` for generated register types. |
| `debug` | Implement `core::fmt::Debug` for generated register types (gated by `--impl-debug-feature debug` at generation time). |

Features must remain **strictly additive**. CI runs `cargo hack
--feature-powerset check` to verify this; see §6.

---

## 2. Generated code — read this first

**Almost everything under `src/` is generated.** It is produced by
[`svd2rust`](https://github.com/rust-embedded/svd2rust) from a patched
CMSIS-SVD description of the MIMXRT633S. Hand-editing generated files will
be silently lost the next time the PAC is regenerated.

### Source of truth

```
svd/MIMXRT633S.svd                  ← raw vendor SVD (excluded from crate via [package.exclude])
patch/MIMXRT633S.yaml               ← top-level svdtools patch (entry point)
patch/{adc,crc,ctimer,dma,espi,
       i2s,inputmux,toplevel,
       usart}.yaml                  ← per-peripheral svdtools patches, _include'd from MIMXRT633S.yaml
build.rs                            ← installs device.x when the `rt` feature is on
device.x                            ← linker fragment used by cortex-m-rt
src/lib.rs, src/<peripheral>/...    ← GENERATED — do not edit by hand
```

### Where to make changes

| You want to … | Edit … |
|---------------|--------|
| Fix a wrong register/field name, width, access, reset value | the relevant file in `patch/` |
| Add or remove a peripheral, change interrupt list | `patch/toplevel.yaml` (and add to `_include:` in `patch/MIMXRT633S.yaml`) |
| Upgrade the underlying SVD | replace `svd/MIMXRT633S.svd`, re-run regeneration, review `patch/` for drift |
| Change generator flags (e.g., `--impl-defmt`, `--impl-debug`, `--ignore-groups`) | update the regeneration commands in `README.md` **and** the regeneration section below |
| Change crate metadata, features, MSRV, dependencies | `Cargo.toml` (this is **not** generated) |
| Change CI | `.github/workflows/*.yml` |
| Change build script behavior | `build.rs`, `device.x` |
| Change linting rules for dependencies/licenses | `deny.toml` |

If a CI failure or HAL bug points at a generated file, the fix nearly always
belongs in a `patch/*.yaml` file, not in `src/`.

### Regenerating the PAC

The canonical procedure is reproduced from `README.md`. Run it from the
repository root.

**Unix:**

```console
$ svdtools patch patch/MIMXRT633S.yaml
$ svd2rust -i svd/MIMXRT633S.svd.patched \
      --reexport-interrupt --ignore-groups \
      --impl-defmt defmt --impl-debug --impl-debug-feature debug
$ rm -r src/*
$ form -i lib.rs -o src
$ rm lib.rs
$ cargo fmt
```

**Windows (PowerShell / cmd):**

```console
$ svdtools.exe patch patch\MIMXRT633S.yaml
$ svd2rust.exe -i svd\MIMXRT633S.svd.patched `
      --reexport-interrupt --ignore-groups `
      --impl-defmt defmt --impl-debug --impl-debug-feature debug
$ rm -r src\*
$ form -i lib.rs -o src
$ rm lib.rs
$ cargo fmt
$ cd src
$ dos2unix **\*.rs *.rs
```

The Windows `dos2unix` step is required because `form` / `svd2rust` emit
CRLF on Windows and the rest of the repository is LF (see §4).

After regeneration:

1. `git status` should show changes confined to `src/`, possibly to
   `svd/MIMXRT633S.svd.patched`, and nothing else.
2. Run the full local check suite from §6 before opening a PR.
3. The first commit should be **regeneration only** (no other changes), to
   make review and `git bisect` tractable.

### Required tools for regeneration

| Tool | Suggested install |
|------|-------------------|
| `svdtools` | `cargo install svdtools` |
| `svd2rust` (must match the version baked into `src/lib.rs` doc-comment; currently `v0.35.0`) | `cargo install svd2rust --version 0.35.0` |
| `form`     | `cargo install form` |
| `rustfmt` (nightly recommended, see §6) | `rustup component add rustfmt --toolchain nightly` |
| `dos2unix` (Windows only) | bundled with Git for Windows, or `choco install dos2unix` |

If you bump the svd2rust version, also bump the version string referenced
in the generated `src/lib.rs` doc-comment by regenerating.

---

## 3. Working in this repository as an agent

### Golden rules

1. **Do not hand-edit generated files** (`src/**`, `device.x` is generated
   by `build.rs` from the checked-in copy). If a change is required there,
   patch the SVD / `patch/*.yaml` and regenerate.
2. **Keep generated and non-generated changes in separate commits.** A
   "regenerate PAC" commit must contain only the regeneration output; logic,
   metadata, or CI changes go in separate commits.
3. **Run the full local check suite (§6) before pushing.** CI is matrixed
   over multiple toolchains and feature combinations; local `cargo check`
   alone is not sufficient.
4. **Preserve the public API.** This crate is published to crates.io and is
   gated by `cargo-semver-checks` in CI. Breaking changes must come with a
   semver-major version bump in `Cargo.toml`.
5. **Be careful with `unsafe`.** Per `CONTRIBUTING.md`, embedded work
   requires `unsafe`, but it must be wrapped in safe interfaces — do not
   sprinkle `unsafe` keywords in HAL or example code that consumes the PAC.

### Things you should not change without an explicit task

- The crate name, edition, MSRV, or license.
- The svd2rust generator flags, unless the task is exactly "change generator
  flags" — they affect every file in `src/`.
- The interrupt list in `src/lib.rs` — patch `patch/toplevel.yaml` instead.
- CI workflow triggers (`on: push: branches: [main]`, `on: pull_request:`)
  and the per-commit matrix in `check.yml`. The `commit_list` job
  intentionally runs the suite on every commit in a PR, which is why each
  commit must independently build cleanly.

### Things you should keep in sync when you do change them

- Bumping `rust-version` in `Cargo.toml` → update the `msrv` matrix in
  `.github/workflows/check.yml`.
- Adding a feature in `Cargo.toml` → confirm `cargo hack --feature-powerset
  check` still passes (it runs in CI).
- Adding a peripheral via `patch/` → add it to `_include:` in
  `patch/MIMXRT633S.yaml` and regenerate.
- Changing the svd2rust version → regenerate, then update the version
  string in `README.md` if applicable.

---

## 4. Line endings, formatting, and style

### Line endings

The repository uses **LF** line endings throughout (verified on the
generated `src/` tree, on `patch/*.yaml`, and on workflow YAML). Configure
git locally to avoid CRLF contamination:

```console
$ git config core.autocrlf false
```

On Windows, after running `form` / `svd2rust`, run `dos2unix` on the
generated tree (see §2).

Before committing, `git diff --check` must report no errors (no whitespace
errors, no mixed line endings, no trailing whitespace introduced).

### Formatting

- All Rust code must pass `cargo fmt --check`. CI runs this on **nightly**
  rustfmt; stable rustfmt is usually fine but if you see a diff, run
  `cargo +nightly fmt`.
- There is no `rustfmt.toml`; default rustfmt rules apply.
- Generated code is reformatted by the regeneration script's final
  `cargo fmt`. Do not commit generated code that has not been through
  `cargo fmt`.

### Clippy

- CI runs `cargo clippy` on `stable` and `beta`.
- The generated tree currently emits a large number of clippy *warnings*
  (mostly stylistic, e.g., elided lifetimes that could be `'_`). These are
  **expected** and originate in svd2rust output; do not attempt to "fix"
  them with `cargo clippy --fix` against generated files. If clippy starts
  reporting a new *error* (not warning), the fix usually belongs in the
  generator / patch layer, not in `src/`.

---

## 5. Commits and pull requests

These rules are a superset of `.github/copilot-instructions.md`. The
copilot-instructions file points here.

### Commit messages

- **Subject line:** capitalized, ≤ 50 characters, imperative mood
  ("Fix bug", not "Fixed bug").
- Separate subject from body with a single blank line.
- Wrap the body at 72 characters.
- Use the body to explain **what** and **why**, not **how**.
- Each commit must build cleanly on its own (no `rustc` or `clippy`
  errors) — `check.yml` runs the matrix on every commit in a PR.
- Squash typo / formatting fixups into the commit that introduced them.

### AI attribution

Every commit that includes AI-generated or AI-assisted work **must**
contain an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

Where:

- `AGENT_NAME` is the AI tool or framework (e.g., `GitHub Copilot`).
- `MODEL_VERSION` is the specific model version used
  (e.g., `claude-opus-4.7`, `gpt-5.3-codex`).
- `[TOOL1] [TOOL2]` are optional specialized analysis tools
  (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`). Basic
  development tools (git, cargo, editors) should **not** be listed.

AI agents **must verify their own identity** (agent name and model version
at runtime) before composing the trailer — do **not** hard-code a model
name from a previous session.

AI agents **MUST NOT** add `Signed-off-by` trailers. Only humans can
certify the Developer Certificate of Origin.

Example trailer block:

```
Patch ESPI register layout

The ESPI peripheral in SVD r0p2 misreports the width of the PERIPHERAL
register as 16 bits; it is 32. Patch the SVD via svdtools and regenerate.

Assisted-by: GitHub Copilot:claude-opus-4.7
```

### Branches and PR etiquette

Per `CONTRIBUTING.md`:

- Use meaningful branch names of the form `user_alias/feature` (e.g.,
  `febalbi/patch-espi-width`). Avoid `wip/`, `test/`, etc.
- Open the PR **as a draft** first; wait for the lint/sanity workflows in
  `.github/workflows/` to pass before requesting review.
- Squash-merging is disabled on this repo. Reorganize your branch so each
  commit is independently buildable and meaningful before opening the PR
  for review.
- For early design feedback, prefix the PR title with `RFC:`.
- When reporting a regression, use `git bisect` and identify the first
  offending commit.

### What goes in a PR

| PR type | Expected contents |
|---------|-------------------|
| Regenerate PAC | One commit containing only `src/**` (and possibly `svd/*.patched`) changes. The message should state the svd2rust version and any patch changes that motivated the regeneration. |
| SVD patch | One or more commits under `patch/**`, plus a single regeneration commit. The patch and the regeneration commit should be separate. |
| CI / metadata | Changes confined to `.github/`, `Cargo.toml`, `deny.toml`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md`. No `src/` changes. |
| Bump svd2rust | Patch commits (if any) + regeneration commit + a metadata commit that updates README/AGENTS.md references. |

---

## 6. Local verification — run these before pushing

The commands below mirror `.github/workflows/check.yml` and `nostd.yml`.
Run them from the repository root. Times shown are observed locally on a
warm cache; cold builds are slower.

### Required (mirrors CI)

```console
$ cargo fmt --check                        # mirrors `fmt` job (nightly in CI, stable usually OK)
$ cargo check                              # mirrors `msrv` job (CI pins toolchain to 1.76)
$ cargo clippy                             # mirrors `clippy` job (CI runs stable + beta)
$ cargo doc --no-deps --all-features       # mirrors `doc` job (CI uses nightly + RUSTDOCFLAGS=--cfg docsrs)
$ cargo hack --feature-powerset check      # mirrors `hack` job; requires `cargo install cargo-hack`
$ cargo deny check --all-features          # mirrors `deny` job; requires `cargo install cargo-deny`
$ cargo check --target thumbv8m.main-none-eabihf --no-default-features
                                           # mirrors `nostd.yml`; requires `rustup target add thumbv8m.main-none-eabihf`
```

### Optional but recommended

```console
$ cargo +nightly fmt                       # if `cargo fmt --check` fails on stable
$ cargo semver-checks                      # mirrors `semver` job (requires `cargo install cargo-semver-checks`)
```

### Expected output

- All seven required commands must exit `0`.
- `cargo check` and `cargo clippy` currently emit thousands of warnings from
  the generated code. These are tolerated. Any *new* warning that did not
  exist before your change, however, should be investigated.
- `git diff --check` must be clean.
- If you only touched `patch/*.yaml`, `Cargo.toml`, `README.md`,
  `CONTRIBUTING.md`, or `AGENTS.md` — and **not** `src/` — you may skip
  the `nostd` and `cargo hack` runs locally; CI will still cover them.

### When CI fails but local checks pass

- Check the toolchain: CI uses `nightly` for `fmt` and `doc`, `stable` and
  `beta` for `clippy`, and `1.76` for `msrv`. A clippy lint added in beta
  but not stable will surface in CI first.
- Check the matrix: `commit_list` runs every job on every commit in the
  PR. A commit in the middle of the series that does not independently
  build will fail CI even if the tip is green.

---

## 7. Repository tour

```
.
├── .github/
│   ├── copilot-instructions.md   ← legacy entry point; now points to this file
│   └── workflows/
│       ├── check.yml             ← fmt / clippy / semver / doc / hack / deny / msrv
│       └── nostd.yml             ← cargo check on thumbv8m.main-none-eabihf
├── AGENTS.md                     ← this file
├── Cargo.toml                    ← crate metadata, features, MSRV
├── CODEOWNERS
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md               ← human-oriented contribution rules
├── LICENSE                       ← MIT
├── README.md                     ← user-facing docs + regeneration recipe
├── SECURITY.md
├── build.rs                      ← installs device.x when `rt` is enabled
├── deny.toml                     ← cargo-deny configuration
├── device.x                      ← cortex-m-rt linker fragment
├── patch/                        ← svdtools YAML patches (SOURCE OF TRUTH for SVD fixes)
│   ├── MIMXRT633S.yaml           ← top-level patch; lists peripheral patches in `_include:`
│   ├── adc.yaml
│   ├── crc.yaml
│   ├── ctimer.yaml
│   ├── dma.yaml
│   ├── espi.yaml
│   ├── i2s.yaml
│   ├── inputmux.yaml
│   ├── toplevel.yaml
│   └── usart.yaml
├── src/                          ← GENERATED by svd2rust — do not hand-edit
│   ├── lib.rs
│   ├── generic.rs
│   └── <peripheral>/...
└── svd/
    └── MIMXRT633S.svd            ← vendor SVD; excluded from the published crate
```

---

## 8. Quick reference for common tasks

### "Fix a wrong register field" (most common task)

1. Identify the peripheral and the wrong field in `src/<peripheral>/...`.
   Note the SVD name, **do not edit the file**.
2. Open the corresponding `patch/<peripheral>.yaml` (create it and add to
   `patch/MIMXRT633S.yaml` `_include:` if missing).
3. Add an `svdtools` patch entry; consult
   <https://github.com/rust-embedded/svdtools> for the YAML schema.
4. Regenerate (§2). Commit the patch and the regeneration separately.
5. Run §6.

### "Add a new peripheral"

1. Verify the peripheral is present in `svd/MIMXRT633S.svd`. If not, the
   vendor SVD itself must be updated.
2. Add or extend a `patch/<peripheral>.yaml` if any fixups are needed.
3. Add it to `_include:` in `patch/MIMXRT633S.yaml` if it has its own
   patch file.
4. Regenerate. Verify the new peripheral appears in `src/`.
5. Run §6.

### "Bump svd2rust"

1. Install the new version: `cargo install svd2rust --version X.Y.Z`.
2. Regenerate (§2). Expect a large diff in `src/`.
3. Verify §6 still passes (especially `cargo-semver-checks`).
4. Update the version string in any documentation that references it.

### "Bump MSRV"

1. Update `rust-version` in `Cargo.toml`.
2. Update the `msrv` matrix in `.github/workflows/check.yml`.
3. Justify the bump in the commit body (which dependency or language feature
   requires it).

---

## 9. Out of scope for this crate

These items belong in downstream crates, not here:

- HAL abstractions, drivers, async runtimes — go to `embassy-imxrt` and
  similar crates.
- Board support packages, example applications, RTOS integrations.
- Anything that requires a `std`-dependent dependency.

If you find yourself wanting to add such code here, stop and confirm the
intent before proceeding.

---

## 10. Pointers

- Upstream SVD2Rust: <https://github.com/rust-embedded/svd2rust>
- svdtools: <https://github.com/rust-embedded/svdtools>
- form: <https://github.com/djmcgill/form>
- cortex-m-rt: <https://github.com/rust-embedded/cortex-m>
- Downstream HAL: <https://github.com/OpenDevicePartnership/embassy-imxrt>

---

*Last reviewed against the repository at branch `improve-agentic-workflow`.*
*Update this file in the same commit as any convention change above.*

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
