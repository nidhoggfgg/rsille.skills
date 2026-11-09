---
name: rsille
description: Build, extend, debug, and test Rust terminal applications with the rsille framework, including widgets, layouts, keyboard and mouse input, background work, animation, and Canvas graphics. Use when the user asks to develop with rsille or the application already depends on rsille.
---

# Develop with rsille

Build applications against rsille's public Rust API. These instructions and the
bundled starter target **rsille 3.0.2 (v3)**. Inspect the application's `Cargo.toml`
and lockfile first; preserve its dependency choices and verify differences when
it uses another version.

## Choose the entry point

| Application needs | Start with |
| --- | --- |
| Controls, forms, dashboards, editors, interactive tools | `rsille::tui::prelude::*` and `App` |
| Graphics embedded beside controls | TUI `canvas(...)` and its viewport |
| A drawing or custom whole-scene renderer | `rsille::canvas` and `rsille::render::Builder` |

Depend on `rsille = "3.0.2"` and use its public modules: `rsille::tui` for
applications and widgets, `rsille::render` for rendering, and `rsille::canvas`
for graphics. Use these module paths when adapting repository example imports.
Default features are empty; add `features = ["syntax-highlighting"]` only when
Syntect code highlighting is needed.

## Discover components before implementing UI

When choosing or adding a UI component, first read the
[component catalog](references/components.md). It maps user-facing needs to the
built-in controls, collections, content viewers, navigation, layouts, windows,
and visual wrappers, with constructors, important callbacks, and example links.
It also distinguishes drawable widgets from state and command helpers.

Match the requested behavior to a catalog entry, then consult its example or
source for the methods you need. For example, searchable choices use `select`,
command search uses `command_palette`, tabular data uses `data_table`, and large
custom row sets use `VirtualList`. The starter is only one composition of the
available widgets, not the inventory.

If no entry matches, check the resolved version's public exports before concluding
that a component is unavailable. Compose existing widgets or implement the missing
behavior with Canvas/a custom `Widget` as appropriate; do not guess a constructor
from another framework. The catalog records useful composition routes too.

## Start or extend an application

For a new application, copy [assets/starter](assets/starter) into the requested
project directory and adapt its package name, state, messages, and view. It
contains a controlled text field, buttons, an explicit exit action, and a
headless interaction test. It uses a registry dependency and does not require
the rsille repository. Its empty `[workspace]` makes it an independent project;
remove that section when deliberately adding it to an existing Cargo workspace.
In an existing application, borrow only the relevant pieces.

Keep application data in state, describe widgets in `view(&State)`, and handle
messages in `update(&mut State, Msg, &mut UpdateCtx<Msg>)`. Use `ctx` to schedule
work and request exit; update does not return a command. Configure services,
subscriptions, input, theme, and mode **before** `.update(...)`:

```rust
use rsille::tui::prelude::*;

fn main() -> WidgetResult<()> {
    let exit = App::new(0_u32)
        .mode(Mode::Inline)
        .update(|count, (), _ctx| *count += 1)
        .view(|count| button(format!("Count: {count}")).on_click(|| ()))
        .run()?;
    println!("Final count: {}", exit.state);
    Ok(())
}
```

Fullscreen is the default. Use `Mode::Inline` for small interfaces in the normal
screen. Let the runtime own terminal setup, input, and cleanup; print final
output after `run()` returns. Use `.run_async().await` inside an existing Tokio
runtime with the time driver enabled.

## Read the relevant reference

- For widgets, layouts, identity, background tasks, subscriptions, input routing,
  and deterministic tests, read [references/application.md](references/application.md).
- For canvas encodings, coordinate systems, world/pointer transforms, visual
  effects, and direct rendering, read [references/graphics.md](references/graphics.md).

Do not assume APIs from other TUI frameworks: composition uses
`col().child(...)`, `row().child(...)`, and `panel().child(...)`; sizing methods
depend on the concrete widget. Persistent inputs need stable keys and must
receive their changed values back from application state. Closures stored in
widgets own their captures instead of borrowing the transient `&State`.

## Verify the user flow

Run `cargo fmt --check`, `cargo check`, and `cargo test` in the application.
Adapt checks to its enabled features. With the bundled starter, `cargo test`
replays Unicode paste, focus movement, button activation, resize, and exit
without opening a terminal.

Use `.build().size(columns, rows)` and `step(&[Event])` to test meaningful input
paths and inspect state, focus, and rendered cells. Use `ManualClock` and
`ManualExecutor` for scheduled work; see the application reference. Exercise
relevant narrow layouts and Unicode data. When a TTY is available, also check
the actual interaction and terminal restoration. Headless checks do not prove
that a particular terminal supports key releases, colors, or glyph widths;
report what was actually verified.

## Resolve unfamiliar APIs from source

The bundled material is sufficient to start without a framework checkout. For
an unfamiliar widget or advanced behavior, inspect source matching the resolved
dependency version (a local checkout, Cargo's downloaded sources, or the
[upstream repository](https://github.com/nidhoggfgg/rsille)). Prefer actual
signatures over guessing builder methods. Without a framework checkout, run
`cargo metadata --format-version 1` in the consumer project, find the resolved
`rsille` entry in `packages`, and use the parent of its `manifest_path` as the
source root. The published crate includes `src/`; the example projects
are available through the catalog's upstream links or a repository checkout.

Useful paths within the framework source:

- `src/tui/prelude.rs`: public names and imports.
- `src/tui/widgets/mod.rs`, `src/tui/layout/mod.rs`: available widgets and layouts.
- `src/tui/app.rs`, `src/tui/effect.rs`, `src/tui/subscription.rs`: lifecycle and work.
- `src/tui/widgets/`, `src/tui/layout/`: widget-specific builders and sizing.
- `packages/tui/examples/README.md`: small examples indexed by feature.
- `packages/tui/examples/basics/workflow.rs`: input, debounce, task, and exit.
- `examples/lumen/`, `examples/orbit-files/`, `examples/minecraft/`: larger apps.

In a framework checkout, `cargo run -p tui --example workflow -- --headless`
runs the small workflow example; it is not a command for a consumer project.
Standalone projects under `examples/` require their own `--manifest-path`.
