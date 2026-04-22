# Rust 2024 Edition + Latest Dependencies Upgrade Plan

Target: `rustc 1.97.0-nightly` · Edition 2024 · All deps bumped to latest

---

## Checklist

### Phase 1 — Fix Immediate Build Blockers ✅

- [x] **1.1** Rename `gen` → `tokens` in `rust_book/macros/hello_macro/hello_macro_derive/src/lib.rs` (`gen` is reserved in 2024)
- [x] **1.2** Fix `extern "C"` → `unsafe extern "C"` in `bindgen_experiments/src/bindings.rs`
- [x] **1.3** Bump `polars` `0.43.1` → `0.53.0` in `polars/polars_rs/Cargo.toml` (fixes `ethnum` transmute bug); excluded from main workspace (chrono version conflict with datafusion)
- [x] **1.4** Bump `datafusion` `"42.1.0"` → `"53.1.0"` in `data_fusion/Cargo.toml`; fixed non-exhaustive match on new `DataFusionError` variants
- [x] **1.5** Run `cargo update` on main workspace; `cargo check` passes clean

### Phase 2 — Edition 2021 → 2024 ✅

- [x] **2.1** Change `edition = "2021"` → `edition = "2024"` in ALL Cargo.toml files
- [x] **2.2** Bump workspace resolver `"2"` → `"3"` in all four workspace roots
- [x] **2.3** Added `[workspace]` to all ~46 standalone Cargo.toml files to prevent workspace discovery errors
- [x] **2.4** Fix `ref` in match patterns: `data_structures_and_algos` skip_list, doubly_linked_list, btree; `macros/builder` fields.rs
- [x] **2.5** Fix `extern "C"` + `#[no_mangle]` + `static mut` reference in `rust_book/intermediate/src/main.rs`
- [x] **2.6** Verified all sub-workspaces and standalone packages compile under edition 2024

### Phase 3 — Fix Broken / Abandoned Dependencies ✅

- [x] **3.1** Replace `proc-macro-error` → `proc-macro-error2 = "2"` in `macros/panic-to-result/`
- [x] **3.2** Bump `syn = "1.0"` → `syn = "2"` in `rust_book/macros/hello_macro/hello_macro_derive/`
- [x] **3.3** `bencher` in `data_structures_and_algos/ch4_lists` — verified still works (no migration needed)
- [x] **3.4** Replace git URL `rand` dep in `rust_by_example/cargo/foo/Cargo.toml` with `rand = "0.9"`

### Phase 4 — Bump All Dependencies to Latest ✅

- [x] **4.1** `sea-orm` left at `0.12.15` — runtime needs MySQL (see Phase 8)
- [x] **4.2** `leptos` `0.6.11` — commented out from main workspace (see Phase 7)
- [x] **4.3** `wgpu` → `"29.0"`, `winit` → `"0.30"` in `webgpu/`; full ApplicationHandler rewrite
- [x] **4.4** `ratatui` → `0.30.0`, `crossterm` → `0.29.0` across all ratatui projects; fixed Title→Line, f.size()→f.area(), Terminal::draw error type
- [x] **4.5** `bevy` `0.13.2` — all 4 bevy projects compile as-is
- [x] **4.6** `pyo3` → `"0.28"` across all PyO3 projects; fixed `with_gil`→`attach`, `_bound` suffixes, `Cell<u64>`→`AtomicU64`
- [x] **4.7** `tonic` → `0.13`, `tonic-build` → `0.13`, `prost` → `0.13` (tonic-build 0.14 removed compile_protos)
- [x] **4.8** `axum` → `"0.8"`, `tower` → `"0.5"`, `hyper` → `"1"` in `tokio/axum-tokio/`
- [x] **4.9** `rand` → `"0.9"` in most places; `cookbook/algorithms` → `"0.10"` + `rand_distr` → `"0.6"` with full RngExt API migration
- [x] **4.10** `clap` → `"4.5"`, `wasm-bindgen` → `"0.2"` (latest patch)
- [x] **4.11** Fixed `polars/polars_rs` — `cols([...]) * lit(x)` (Selector * Expr) → separate `col()` expressions

### Phase 5 — Initial Validation ✅

- [x] **5.1–5.15** All packages `cargo check` clean (see deferred items below)

---

### Phase 6 — Consolidate Standalones into Workspaces ✅

- [x] **6.1** Created `async/Cargo.toml` workspace covering all 9 `async/*` packages
- [x] **6.2** Created `bevy/Cargo.toml` workspace covering all 4 `bevy/*` packages
- [x] **6.3** Created `clis_and_tuis/Cargo.toml` workspace covering all clis/tuis packages
- [x] **6.4** Created `cookbook/Cargo.toml` workspace covering `cmd_cli` + `algorithms`
- [x] **6.5** Created `rust_by_example/Cargo.toml` workspace covering cargo/bar, cargo/foo, testing
- [x] **6.6** Created `PyO3/Cargo.toml` workspace covering all 3 PyO3 packages
- [x] **6.7** Created `rustonomicon/Cargo.toml` workspace covering all 10 rustonomicon packages
- [x] **6.8** Created `wasm/Cargo.toml` workspace covering snake_game
- [x] **6.9** Removed individual `[workspace]` stubs from all 38 absorbed packages
- [x] **6.10** All new sub-workspaces verified `cargo check` clean

### Phase 7 — Fix Leptos (0.6 → 0.8 Migration) ✅

- [x] **7.1** Update `webdev/leptos/leptos-tutorial/Cargo.toml`: `leptos = "0.8.19"`
- [x] **7.2** Add `leptos-tutorial` back to main `Cargo.toml` workspace members
- [x] **7.3** Migrate `src/main.rs`: `use leptos::prelude::*`, `create_signal` → `signal`, `mount_to_body` → `leptos::mount::mount_to_body`, `count()` → `count.get_untracked()`
- [x] **7.4** Migrate `src/components/progressbar.rs`: `use leptos::prelude::*`
- [x] **7.5** Migrate `src/components/conditionals.rs`: `signal`, `value()` → `value.get()`
- [x] **7.6** Migrate `src/components/control.rs`: `signal`, `NodeRef::new()`, `input_element()` → `input_element.get()`
- [x] **7.7** Migrate `src/components/child_components.rs`: `signal`, `Children` type — simplified `WrapsChildren` (Fragment.nodes removed in 0.8)
- [x] **7.8** Migrate `src/components/parent_child_comms.rs`: `signal`
- [x] **7.9** Migrate `src/components/lists.rs`: `signal`, `create_signal` → `signal`
- [x] **7.10** Migrate `src/components/use_effect.rs`: `RwSignal::new`, `Effect::new`, `NodeRef::new`, removed custom `Show` (conflicts with built-in), `with!` macro → `first.get()/last.get()`, renamed `log` fn → `log_msg`
- [x] **7.11** Migrate `src/components/errorhandling.rs`: `signal`
- [x] **7.12** `cargo check --target wasm32-unknown-unknown` passes clean; `cargo check` (native) also clean

### Phase 8 — Fix Sea-ORM (Start MySQL + Validate) ✅

- [x] **8.1** Started MySQL via Docker: `docker run --name mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=password -d mysql:latest`
- [x] **8.2** Created `bakeries_db` database
- [x] **8.3** Upgraded sea-orm `0.12.15` → `2.0.0-rc.38`; fixed `execute` → `execute_unprepared` for raw SQL; fixed non-exhaustive `DatabaseBackend` match
- [x] **8.4** Ran `sea-orm-cli migrate refresh` — both migrations applied cleanly
- [x] **8.5** `cargo test` passes (0 failures)
- Note: restart MySQL container after reboot with `docker start mysql`

### Phase 9 — Fix Bindgen / rustonomicon/ffi ✅

- [x] **9.1** clang-22 already installed at `/usr/lib/llvm-22/bin/clang`; fixed broken alternative via `sudo update-alternatives --config clang`
- [x] **9.2** Upgraded `bindgen` `0.69.4` → `0.72` (generates edition 2024 compatible `unsafe extern "C"`); added `.cargo/config.toml` with `CLANG`/`LIBCLANG_PATH` env vars
- [x] **9.3** Installed `libsnappy-dev`; rewrote `rustonomicon/ffi/build.rs` to link system snappy (removed cmake); fixed `extern` → `unsafe extern "C"`, `#[no_mangle]` → `#[unsafe(no_mangle)]`, static mut ref fix
- [x] **9.4** `rustonomicon/ffi/` `cargo test` passes: `call_snappy` + `call_readline` both ok; ffi added to rustonomicon workspace

### Phase 10 — Documentation ✅

- [x] **10.1** Breaking-change notes with official doc links added to upgrade-plan.md (phases 1–9)
- [x] **10.2** Updated README files: root README (marked Polars/DataFusion/WebGPU/EffectiveRust ✅, added entries 17-20, updated date), leptos README (0.8 migration note + updated code examples), sea_orm README (2.0-rc.38 note + Docker MySQL), bevy README (0.13.2 note + workspace structure note)

### Phase 11 — Cargo Tests ✅

- [x] **11.1** Main workspace — all pass
- [x] **11.2** `data_structures_and_algos/` — 30 tests pass
- [x] **11.3** `macros/builder/` — 9 tests pass
- [x] **11.4** `async/`, `clis_and_tuis/`, `cookbook/`, `rust_by_example/`, `rustonomicon/` — all pass
- [x] **11.5** No failures found

### Phase 12 — Code Quality (Clippy + Fmt) ✅

- [x] **12.1** `cargo fmt --all` — main workspace (formatted leptos + other files)
- [x] **12.2** `cargo clippy --all-targets` — main workspace; no errors, only expected learning-code warnings
- [x] **12.3** `cargo fmt --all` + `cargo clippy --all-targets` for all sub-workspaces: async, bevy, clis_and_tuis, cookbook, rust_by_example, PyO3, rustonomicon, wasm, data_structures_and_algos, polars, macros/builder
- [x] **12.4** Fixed bevy workspace: moved `[profile.*]` to workspace `Cargo.toml`, moved `[target.*]` to `bevy/.cargo/config.toml`; removed stale `[workspace]` stubs from bevy members

### Phase 13 — Bevy 0.13.2 → 0.18.1 Migration ✅

- [x] **13.1** All 4 bevy workspace `Cargo.toml` files bumped to `bevy = "0.18.1"`
- [x] **13.2** Removed `bevy_mod_picking` (archived; picking upstreamed into bevy 0.14); replaced with `bevy::picking::prelude::*` + `MeshPickingPlugin`
- [x] **13.3** `bevy/three_d_shapes`: fix `RenderAssetUsages` import (`bevy::render::render_asset` → `bevy::asset`)
- [x] **13.4** `bevy/pong`: `add_event` → `add_message`; `#[derive(Event)]` → `#[derive(Message)]`; `EventWriter/EventReader` → `MessageWriter/MessageReader`; `JustifyText::Center` → `Justify::Center`
- [x] **13.5** `bevy/picking`: full rewrite — `Trigger<Pointer<E>>` → `On<Pointer<E>>`; `trigger.target()` → `trigger.entity`; `trigger.entity()` → `trigger.observer()`; `trigger.event().field` → `trigger.field` (via Deref); `Pointer<Down>` → `Pointer<Press>`; `on_event` → `on_message`; `EventWriter/EventReader` → `MessageWriter/MessageReader`; `material.base_color.to_hsla()` → `Hsla::from(material.base_color)`
- [x] **13.6** `bevy/quickstart_ecs`: renamed `Name` → `PersonName` to avoid conflict with `bevy::core::Name` added to prelude in 0.16+
- [x] **13.7** All bevy crates `cargo check` + `cargo test` pass

### Phase 14 — Remaining Dep Bumps ✅

- [x] **14.1** `clis_and_tuis/basics/grrs`: `indicatif` `0.17` → `0.18`
- [x] **14.2** `clis_and_tuis/basics/misc`: `config` `0.14` → `0.15`, `signal-hook` `0.3` → `0.4`
- [x] **14.3** `clis_and_tuis/ratatui/async-ratatui-counter`: `config` `0.14` → `0.15`, `directories` `5` → `6`, `human-panic` `1` → `2`, `json5` `0.4` → `1`, `signal-hook` `0.3` → `0.4`
- [x] **14.4** `macros/builder`: `cargo update` bumped `proc-macro2` `1.0.82` → `1.0.106`, `quote` `1.0.36` → `1.0.45`, `syn` `2.0.64` → `2.0.117`, `trybuild` `1.0.96` → `1.0.116`
- [x] **14.5** All updated packages `cargo check` + `cargo test` pass

---

## Ongoing Notes

- **sea-orm Docker** — restart with `docker start mysql` after reboots
- **polars** — excluded from main workspace (chrono version conflict with datafusion); standalone workspace at `polars/polars_rs/`

### bevy 0.13 → 0.18 Breaking Changes Applied
**Docs:** https://bevyengine.org/learn/migration-guides/  
- **Required Components** — `Bundle` types like `Camera2dBundle`, `Camera3dBundle`, `PbrBundle`, `PointLightBundle`, `MaterialMesh2dBundle` removed; spawn component tuples directly e.g. `(Mesh3d(h), MeshMaterial3d(m), Transform::...)`
- **Event system renamed to Messages** — `#[derive(Event)]` → `#[derive(Message)]`; `add_event::<E>()` → `add_message::<E>()`; `EventWriter<E>` → `MessageWriter<E>`; `EventReader<E>` → `MessageReader<E>`; `on_event::<E>` → `on_message::<E>`
- **Observer API** — `Trigger<E>` param → `On<E>`; `trigger.target()` → `trigger.entity` (Deref); `trigger.entity()` → `trigger.observer()`; fields accessed via Deref on `On<E>` → `E`
- **Picking** — `bevy_mod_picking` archived; use `bevy::picking::prelude::*` + `MeshPickingPlugin`; `Pointer<Down>` → `Pointer<Press>`
- **Text system** — `TextBundle` removed; use `(Text::new("..."), TextFont { font_size: ..., ..default() }, TextColor(...), TextLayout::new_with_justify(Justify::Center), Node {...})`; `JustifyText` → `Justify`
- **Color** — `Color::rgb` → `Color::srgb`; constants moved to `bevy::color::palettes::css::*`; `color.to_hsla()` → `Hsla::from(color)`
- **`RenderAssetUsages`** — moved from `bevy::render::render_asset` to `bevy::asset`
- **`time.delta_seconds()`** → `time.delta_secs()`; **`events.send()`** → `events.write()`
- **`Name` in prelude** — `bevy::core::Name` added to prelude in 0.16+; rename local `Name` structs

### leptos 0.8 Breaking Changes Applied
**Docs:** https://leptos.dev/  |  https://github.com/leptos-rs/leptos/blob/main/CHANGELOG.md
- `use leptos::*` → `use leptos::prelude::*`
- `create_signal(v)` → `signal(v)` (returns `(ReadSignal<T>, WriteSignal<T))`)
- `create_rw_signal(v)` → `RwSignal::new(v)`
- `create_memo(|_| ...)` → `Memo::new(|_| ...)`
- `create_effect(|_| ...)` → `Effect::new(|_| ...)`
- `create_node_ref::<T>()` → `NodeRef::<T>::new()`; node ref access `ref()` → `ref.get()`
- `count()` (signal as function) → `count.get()` (method call)
- `with!` macro removed → use individual `.get()` calls
- `Fragment.nodes` field removed → `Children` returns `AnyView`; simplified `WrapsChildren`
- `mount_to_body` → `leptos::mount::mount_to_body`
- Custom `Show` component conflicts with built-in `Show` — rename or remove

---

## Key Breaking-Change Notes Applied

### Rust 2024 Edition
- `gen` is reserved keyword → rename variables/methods
- `extern "C" {}` → `unsafe extern "C" {}`
- `#[no_mangle]` → `#[unsafe(no_mangle)]`
- `Some(ref x)` patterns → `Some(x)` (implicit ergonomics)
- `&static mut` shared references forbidden even in `unsafe`
- Workspace resolver `"2"` → `"3"` required for edition 2024 workspaces

### `rand` 0.8 → 0.9 / 0.10
**Docs:** https://docs.rs/rand/latest/rand/  |  https://github.com/rust-random/rand/blob/master/CHANGELOG.md
- `rng.gen()` / `rng.gen::<T>()` → `rng.random()` / `rng.random::<T>()`
- `rng.gen_range()` → `rng.random_range()` (rand 0.10 only; 0.9 keeps deprecated shim)
- `rand::thread_rng()` → `rand::rng()` (0.10)
- `rand::distributions::*` → `rand::distr::*` (0.10)
- `Standard` → `StandardUniform` (0.10)
- `Uniform::from(1..7)` → `Uniform::new(1, 7).unwrap()` (0.10)
- `rand 0.10` uses `RngExt` trait (not `Rng`) for most methods
- `rand_distr 0.6` requires `rand 0.10`

### `winit` 0.29 → 0.30
**Docs:** https://docs.rs/winit/latest/winit/  |  https://github.com/rust-windowing/winit/blob/master/CHANGELOG.md
- `WindowBuilder::new().build(&event_loop)` → `event_loop.create_window(Window::default_attributes())`
- `EventLoop::run(move |event, _target, flow|)` deprecated → `run_app(&mut app)` + `ApplicationHandler` trait
- Window creation moved to `ApplicationHandler::resumed()` callback
- `ControlFlow::Exit` → `event_loop.exit()`
- `KeyboardInput` event: `VirtualKeyCode` → `PhysicalKey::Code(KeyCode::...)`

### `wgpu` 22 → 29
**Docs:** https://docs.rs/wgpu/latest/wgpu/  |  https://github.com/gfx-rs/wgpu/blob/trunk/CHANGELOG.md
- Requires matching winit 0.30 / ApplicationHandler pattern
- `rwh_05` → `rwh_06` raw-window-handle feature

### `ratatui` 0.26 → 0.30
**Docs:** https://docs.rs/ratatui/latest/ratatui/  |  https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md
- `widgets::block` module removed → items available at `widgets::*`
- `Title` struct removed → use `Line` directly
- `Position::Bottom` → `TitlePosition::Bottom`
- `.title(line.alignment(...))` → `.title_top()` / `.title_bottom()`
- `frame.size()` → `frame.area()`
- `Terminal::draw()` returns `Result<_, B::Error>` → add `where io::Error: From<B::Error>` to callers

### `crossterm` 0.27 → 0.29
**Docs:** https://docs.rs/crossterm/latest/crossterm/
- `KeyEventKind` matching unchanged; internal event handling improvements

### `pyo3` 0.21 → 0.28
**Docs:** https://pyo3.rs/  |  https://pyo3.rs/v0.28.0/migration.html
- `Python::with_gil(|py| {...})` → `Python::attach(|py| {...})`
- All `_bound` suffix transitional methods removed; use base names: `import`, `eval`, `call`, `into_py_dict`
- `py.eval()` now takes `&CStr` → use `c"..."` C string literals (Rust 1.77+)
- `from_py_with = "fn_name"` → `from_py_with = fn_name` (no quotes, path expression)
- `#[pyclass]` types now require `Send + Sync` → replace `Cell<T>` with `AtomicT`
- `PyComplex::from_doubles_bound` → `PyComplex::from_doubles`
- `Py<T>::call_bound` → `Py<T>::call`
- `qualname()` returns `Bound<'_, PyString>` → `.extract::<String>()?`

### `axum` 0.7 → 0.8
**Docs:** https://docs.rs/axum/latest/axum/  |  https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md
- `Router::new()` API unchanged; internal handler / extractor changes
- `tower` 0.4 → 0.5: `ServiceBuilder` layer API changes

### `tonic` + `tonic-build` 0.11 → 0.13
**Docs:** https://docs.rs/tonic/  |  https://github.com/hyperium/tonic/blob/master/CHANGELOG.md
- `tonic-build 0.14` removed `compile_protos()` entirely (became code-gen-only) — stay on 0.13
- `prost` 0.12 → 0.13: message encoding/decoding trait changes

### `polars` 0.43 → 0.53
**Docs:** https://docs.pola.rs/  |  https://github.com/pola-rs/polars/blob/main/CHANGELOG.md
- `cols(["a", "b"])` now returns `Selector` (not `Expr`) → arithmetic ops removed on Selector
  - Fix: use separate `col("a") * lit(x)` expressions or `.as_expr()` if available
- `ethnum 1.5.0` transmute soundness bug fixed by upgrading from 0.43
- chrono version conflict with `datafusion 53.1` (polars needs ≤0.4.41, datafusion needs ≥0.4.44)
  - Fix: exclude `polars/polars_rs` from main workspace; make it a standalone `[workspace]`

### `datafusion` 42 → 53
**Docs:** https://datafusion.apache.org/  |  https://github.com/apache/datafusion/blob/main/CHANGELOG.md
- `DataFusionError` enum gained new variants (`Diagnostic`, `Collection`, `Shared`, etc.)
  - Fix: add `_ => {}` catch-all to exhaustive match arms

### `syn` 1.0 → 2.0
**Docs:** https://docs.rs/syn/  |  https://github.com/dtolnay/syn/blob/master/CHANGELOG.md
- `syn::parse_macro_input!` unchanged but internal APIs reorganized
- `DeriveInput`, `Data`, `Fields` structs restructured
- Error reporting: `syn::Error::new_spanned` API unchanged

### `proc-macro-error` → `proc-macro-error2`
**Docs:** https://docs.rs/proc-macro-error2/  
- Drop-in replacement crate; just change crate name in Cargo.toml and `use` statements

### `wasm-bindgen` 0.2.92 → latest 0.2.x
**Docs:** https://rustwasm.github.io/wasm-bindgen/  |  https://github.com/rustwasm/wasm-bindgen/blob/main/CHANGELOG.md
- Minor patch; API backwards-compatible within 0.2.x

---

## README Update Recommendations

### Root `README.md`
- Mark Polars/DataFusion/effective_rust/WebGPU sections as `✅` (completed, not in-progress)
- Add `Effective Rust` link entry
- Update study plan dates (2023-2025 → 2026)
- Add `Data Fusion` entry to code list

### `bevy/README.md`
- Note bevy version is 0.13.2 (not yet migrated to 0.18; migration requires GPU)

### `rust_book/README.md`, `async/README.md`, `tokio/README.md`
- Note edition 2024 upgrade

### `webdev/leptos/README.md`
- Note leptos 0.6 is commented out; 0.8 migration in progress

### `sea_orm/README.md`
- Note MySQL must be running locally (or via Docker); sea-orm 0.12.15 still used

---

## Cargo Quality Recommendations

### Formatting
```bash
# Format all files in a workspace
cargo fmt --all

# Check-only (for CI)
cargo fmt --all -- --check
```

### Linting
```bash
# Warn-as-error in CI
cargo clippy --all-targets --all-features -- -D warnings

# Fix auto-fixable lint violations
cargo clippy --all-targets --fix --allow-dirty
```

### Recommended `.cargo/config.toml` additions
```toml
[build]
# Show warnings from dependencies too
rustflags = ["-W", "rust-2024-compatibility"]
```

### Recommended Clippy lints to address across the repo
- `clippy::unwrap_used` — replace `unwrap()` with `?` or `expect()` in library code
- `clippy::needless_pass_by_value` — prefer `&T` over `T` for large types
- `clippy::missing_errors_doc` — add `# Errors` sections to public `Result`-returning fns
- `clippy::must_use_candidate` — add `#[must_use]` to pure functions
