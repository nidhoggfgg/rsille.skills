# Application development

These contracts match rsille 3.0.4. Source links below use the upstream default
branch; check the application's resolved rsille version before adopting a newer
API. In a checkout, the linked paths also identify the local implementation.

## Build the application around messages

Import `rsille::tui::prelude::*`. Use
`App::new(state).update(update).view(view)` and configure mode, theme, hotkeys,
subscriptions, clocks, and executors **before** `.update(...)`.

- `update(&mut State, Msg, &mut UpdateCtx<Msg>)` changes application state and
  queues work with `ctx.emit`, `ctx.after`, `ctx.debounce`, `ctx.spawn`, or
  `ctx.request`. It returns `()`. `Effect` is useful for composing child updates;
  apply it with `ctx.dispatch(effect)`. `CommandRouter` describes application
  commands and key bindings; it is not an asynchronous effect return type.
- `view(&State) -> impl Widget<Msg>` describes the current tree. Keep I/O and
  state mutation out of view. Returned widgets are `'static`: own or clone values
  captured by callbacks instead of borrowing the view's `&State`.
- Messages must be `Send + 'static` for execution; neither `Clone` nor `Debug`
  is required. Handler callbacks can construct messages directly.

`.run()?` owns terminal setup, cleanup, and a private Tokio runtime, then returns
`AppExit { state, reason }`. Print normal output after it returns. Inside an
existing Tokio runtime, use `.run_async().await` with its time driver enabled.
Do not nest `.run()` inside Tokio or add a competing terminal event loop.
`.build()` creates a runtime without opening a terminal and requires owned,
`'static` update and view closures.

Fullscreen is the default. `.mode(Mode::Inline)` uses the normal terminal screen.
Escape is the default quit key after widget handling. Use `ctx.exit()` for an
explicit application action. To reserve Escape for navigation, configure a
different `.with_quit_key(...)` or `.with_quit_key_event(...)`, or disable the
built-in key and provide an explicit exit action.

Source: [app.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/app.rs).

## Layout, controlled state, and identity

Compose `col().child(...)`, `row().child(...)`, and `panel().title(...).child(...)`.
Use `grid().columns("24 1fr").rows("auto 1fr")` for track sizing and
`GridPlacement::new().area(1, 1)` for explicit, one-based placement. Fixed tracks,
padding, and gaps are terminal cells. Grid templates support fixed, fractional,
and auto tracks, not the complete CSS grammar. Check concrete widget methods;
`.width()`, `.height()`, and `.flex()` are not universal widget extensions.

A text input is controlled by application state:

```rust
// In view; Msg::Edit carries String and update stores it in state.text.
text_input().key("editor").value(&state.text).on_change(Msg::Edit)
```

Feed every accepted edit back through `.value(...)` on the next view rebuild.
The widget manages cursor and editing details; do not slice UTF-8 by character
indices. Use the built-in Unicode-aware controls for ordinary text editing.

Give interactive or reordered widgets stable `.key(...)` values. Keys must be
unique within the nearest keyed ancestor, not just among direct siblings. This
preserves focus, widget-local state, and animation when siblings reorder or
unkeyed layout wrappers change. Avoid keys derived from current list positions.
For branches returning different concrete widgets, return
`Box<dyn Widget<Msg>>` and use `.into_widget()` on each branch.

Source: [widget.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/widget.rs),
[text_input.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/widgets/controls/text_input.rs),
[grid_track.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/layout/grid_track.rs).

## Background work and deterministic verification

`Task::new` accepts a synchronous `Fn(TaskContext<Msg>) -> TaskOutcome`, not an
async block. It runs on the configured blocking executor, so clone inputs from
state before spawning. The closure must be `Send + Sync + 'static` and callable
again for retries. Use `ctx.emit(...)` to deliver results back to update.

This complete, terminal-free example replays Unicode input, advances a debounce,
runs its background job, and checks the resulting application state:

```rust
use std::time::Duration;
use rsille::tui::prelude::*;
use rsille::tui::runtime::{ManualClock, ManualExecutor};

#[derive(Default)]
struct State { text: String, saved: String }
enum Msg { Edit(String), Save, Saved(String) }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clock = ManualClock::default();
    let executor = ManualExecutor::default();
    let mut app = App::new(State::default())
        .with_clock(clock.clone())
        .with_executor(executor.clone())
        .update(|state, msg, ctx| match msg {
            Msg::Edit(text) => {
                state.text = text;
                ctx.debounce("save", Duration::from_millis(300), Msg::Save);
            }
            Msg::Save => {
                let text = state.text.clone();
                ctx.spawn(Task::new(move |task| {
                    task.emit(Msg::Saved(text.clone()));
                    TaskOutcome::Complete
                }).key("save"));
            }
            Msg::Saved(text) => state.saved = text,
        })
        .view(|state| {
            text_input().key("editor").value(&state.text).on_change(Msg::Edit)
        })
        .build()
        .size(40, 3);
    app.step(&[Event::Paste("中文👨‍👩‍👧‍👦".into())])?;
    assert_eq!(app.state().text, "中文👨‍👩‍👧‍👦");
    assert_eq!(app.focused_key(), Some("editor"));
    assert!(app.state().saved.is_empty());
    clock.advance(Duration::from_millis(300));
    app.step(&[])?;
    assert_eq!(executor.pending(), 1);
    assert!(executor.run_next());
    app.step(&[])?;
    assert_eq!(app.state().saved, app.state().text);
    Ok(())
}
```

The task models a save result; add real I/O and error messages for persistence.
Spawning a task with the same scoped key cancels the previous one. Cancellation
rejects stale results but does not kill worker code: check `task.is_cancelled()`
or use `task.sleep(...)` in long-running work. `TaskContext::sleep` uses wall-clock
time; `ManualClock` controls runtime timers, not worker sleeps.

For live search, a new edit can arrive while an old task is running and the next
debounce has not fired. Replacing the task key later does not close this window.
Increment a query revision on each edit, include it in task results, and ignore
results whose revision no longer matches state. Test this interval explicitly.

`Subscription::run(key, producer)` describes a continuous source returned from
`.subscriptions(|state| ...)`. Stable scoped keys retain producers, even after
completion. Removing a description cancels it; reintroducing its key starts it
again. Change the key when changed inputs require a new producer. Use
`while ctx.sleep(interval) { ctx.emit(message()); }` and return
`TaskOutcome::Cancelled` when the loop ends. `ManualExecutor::run_next()` runs a
job to completion, so do not run an endless subscription through it synchronously.

Use `child_view.map(Msg::Child).scope("editor")` for component composition.
Map and scope the child's effects and subscriptions separately with the same
resource path. Scoping the widget does not automatically scope those resources.

For interaction tests, `build().size(w, h).step(&[Event])` runs without a TTY.
`step` returns `Result<StepResult, rsille::render::DrawErr>`, while `run` returns
`WidgetResult<AppExit<State>>`. Use `Result<(), Box<dyn std::error::Error>>` in
tests combining these operations; `DrawErr` does not convert into `WidgetError`.
After `handle().send(msg)` or a manual job, call `step(&[])` to process messages.
Inspect `state()`, `frame()`, and `focused_key()`. Replay Tab, Enter, Paste, and
Resize when those behaviors matter; direct message tests alone do not verify
input routing. A real terminal smoke check is still needed for terminal protocol
and font behavior.

Sources: [effect.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/effect.rs),
[subscription.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/tui/subscription.rs),
[workflow example](https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/basics/workflow.rs),
[composition example](https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/basics/composition.rs).
In applications, import TUI APIs through `rsille::tui`; adapt example imports accordingly.

## Input capabilities and diagnosis

`Event` is the re-exported crossterm event. `.on_input(...)` observes
`InputEvent::Capabilities(...)` and `InputEvent::Terminal(Event)` before normal
widget and shortcut handling; producing a message does not consume the input.
`.on_key(...)` accepts Press and Repeat, but ignores Release. Widgets receive
keys before global shortcuts, so a text input may consume a letter shortcut.

For held-key motion, request `.keyboard_mode(KeyboardMode::Auto)` and
`.focus_events(true)`, inspect `InputCapabilities.key_releases`, and clear held
keys on focus loss. Default Legacy input cannot guarantee independent holds or
precise releases; capability negotiation may fall back to it.

| Symptom | Check |
| --- | --- |
| Input reverts or focus moves on redraw | Controlled value round-trip and stable, unique keys. |
| Callback lifetime error | Clone captured values; the widget must own its callbacks. |
| UI freezes during a request | Move blocking work from update/view into a Task. |
| Subscription keeps old parameters or never restarts | Its stable key retains the old producer, including a completed one. |
| Headless task emits but state stays unchanged | Run the manual job, then call `step(&[])`. |
| Letter shortcut never fires | Focused widgets may consume it; choose a suitable hotkey. |
| Arrow/letter action fires twice | An `on_input` observer and widget/shortcut may both emit. |

Source: [input.rs](https://github.com/nidhoggfgg/rsille/blob/main/src/render/input.rs)
and [Minecraft input notes](https://github.com/nidhoggfgg/rsille/blob/main/examples/minecraft/README.md).
