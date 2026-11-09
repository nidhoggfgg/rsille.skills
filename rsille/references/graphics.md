# Canvas, visual effects, and direct rendering

These patterns target rsille 3.0.2. Use `rsille::tui::prelude::*` for embedded
graphics and widgets; use `rsille::{canvas, render}` for standalone drawing.

## Choose coordinates and encoding

| Space | Units and direction |
| --- | --- |
| Layout and renderer size | Terminal cells; x right, y down |
| Raw `Canvas` drawing | Samples; x right, y up |
| Canvas world viewport | Application units; x right, y up |

| `Encoding` | Samples per cell | Suitable starting point |
| --- | --- | --- |
| `Braille` (default) | 2 × 4 | Curves, wireframes, dense points |
| `HalfBlock` | 1 × 2 | Independently colored upper/lower samples |
| `Quadrant` | 2 × 2 | Block graphics with more horizontal detail |

Terminal cells have one foreground and one background color. Braille and quadrant
encoding fit or combine sample colors; do not promise eight or four independent
colors per cell. `set_background` fills uncovered samples. Raw `Canvas` bounds
are inclusive terminal-cell bounds, not sample bounds; `set_bound((0, 9), (0, 3))`
selects 10 × 4 cells. `fixed_bound(true)` prevents automatic expansion.

## Embed a canvas in an application

The drawing callback receives a fresh, bounded canvas each render. Draw the whole
scene from application state; previous callback output does not persist. Prefer
`ctx.world(surface)` and an explicit viewport for plots or scenes that should keep
the same world extent as the terminal size or encoding changes.

```rust
use rsille::tui::prelude::*;

fn plot(viewport: CanvasViewport) -> impl Widget<CanvasPointer> {
    canvas::<CanvasPointer, _>(|surface, ctx| {
        let mut world = ctx.world(surface).color(Color::Cyan);
        for step in 0..160 {
            let x = step as f64 / 10.0;
            world.point(CanvasPoint::new(x, x.sin()));
        }
        world.line(CanvasPoint::new(0.0, 0.0), CanvasPoint::new(16.0, 0.0));
    })
    .encoding(Encoding::Braille)
    .viewport(viewport)
    .on_pointer(|pointer| pointer)
    .key("plot")
    .min_size(20, 6)
}
```

For example, pass `CanvasViewport::new(0.0, -1.5, 16.0, 3.0)`; its arguments
are lower-left x/y and positive width/height. In a larger app, map the pointer to
an application message, such as `.on_pointer(Msg::PlotPointer)`. `CanvasPointer`
contains `position`, `kind`, and `modifiers`; its position is the terminal-cell
center transformed through the same viewport as drawing. Store the viewport in
app state and handle panning, wheel zoom, and hit testing in `update`.
`viewport.pan(dx, dy)` translates it; `viewport.zoom_at(factor, anchor)` zooms in
for factors above one. Validate user-derived values: invalid, non-finite, or
non-positive viewport extents and zoom factors panic.

For direct sample painting, use `surface.set(...)` or `set_color(...)` with
`ctx.sample_width()`/`sample_height()`. `cell_width()`/`cell_height()` are different
units. Do not replace the supplied surface with a canvas of a different encoding:
`ctx.world(surface)` requires matching encodings. Capture owned scene data in
`move` callbacks when it must outlive `view`.

## Add effects without a second frame loop

Wrap the interactive widget itself so its input behavior remains available:

```rust
use std::time::Duration;
use rsille::tui::prelude::*;

fn animated_button() -> impl Widget<()> {
    visual(button("Run").on_click(|| ()))
        .key("run-effect")
        .gradient(Color::Cyan, Color::Magenta, GradientDirection::Horizontal)
        .highlight_sweep()
        .animation(looping_visual_spec(Duration::from_secs(2)))
}
```

The runtime schedules this animation. For application simulation, use
`App::on_frame` before `.update()` and advance state from `FrameInfo`; do not put
simulation or blocking work in a draw callback. `.progress(value)` drives an
effect manually and disables its `.animation(...)`; the last of these setters
wins. Stable `.key(...)` values preserve identity when dynamic siblings move or
change. Changing `.channel(...)` restarts the visual animation without changing
widget identity.

`.enter(VisualEffect::fade_in())` and `.exit(VisualEffect::fade_out())` attach
lifecycle effects. Keep stable keys for conditional/dynamic widgets so entering,
exiting, and surviving elements remain distinguishable. A frosted
`.backdrop(Backdrop::frosted().blur_radius(2).opacity(0.45))` samples lower layers
already painted into the same target; an outer offscreen wrapper creates a new
target and can change what is visible to that backdrop.

## Render without a widget tree

```rust
use rsille::{canvas::Canvas, render};

fn main() -> std::io::Result<()> {
    let mut surface = Canvas::new();
    for x in 0..160 {
        let x = x as f64;
        surface.set(x, 15.0 + (x / 10.0).sin() * 10.0);
    }
    render::Builder::new()
        .size((80, 10))
        .append_newline(true)
        .build_render(surface, std::io::stdout())
        .render()
}
```

`Builder` starts with an empty buffer: set `.size(...)`. `build_render` writes
output only, with screen clearing enabled by default; `.clear(false)` disables
that initial clear. It does not activate raw mode, capture input, or run a loop.
For append-only canvas output, `surface.print_on(&mut writer, false)` writes rows
with newlines without taking over terminal modes.

For an interactive low-level scene, implement `render::Draw` and `render::Update`,
then use `.size(...).enable_all().frame_limit(60).build_event_loop(scene).run()`.
`frame_limit` controls periodic polling, not a strict redraw cap. The loop owns
terminal setup and cleanup; return `true` from `Update::should_quit` for clean
application-driven exit. Use `run_async().await` inside an existing Tokio runtime
with its time driver enabled. Prefer TUI `App` when controls, focus, or layout
are needed rather than rebuilding those facilities around the low-level loop.

Sources in an rsille checkout: `src/tui/widgets/display/{canvas,world_canvas}.rs`,
`src/tui/widgets/motion/visual.rs`, `src/canvas/{canvas,encoding}.rs`, and
`src/render/{builder,draw_update,event_loop}.rs`. For further patterns, inspect
`packages/tui/examples/widgets/canvas.rs` and `examples/lumen/src/art.rs` in the
[upstream repository](https://github.com/nidhoggfgg/rsille).
