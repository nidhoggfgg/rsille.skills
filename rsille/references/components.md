# Component and layout catalog

Use this catalog **before implementing a UI feature** to find an existing rsille
component. It covers the public widget and layout constructors in **rsille 3.0.1**,
plus `VirtualList::new`. Import `rsille::tui::prelude::*` for the constructors below.
Names in constructor arguments describe the required input; they are not literal
variables supplied by the framework. Add `::<Msg>` when message inference needs help.

The linked examples and source files are in the upstream `main` branch. For exact
method signatures, consult the source matching the application's resolved rsille
version. In applications, import TUI APIs through `rsille::tui`; adapt example
imports accordingly. This catalog remains usable when installed without a
framework checkout.

## Find a component by need

- [Controls](#controls): button, text field, multiline editor, checkbox, switch,
  radio group, searchable select, date picker. 输入、表单、选择。
- [Collections](#collections): list, table, tree, filesystem entries, virtual rows.
  列表、表格、树、文件浏览、大数据。
- [Navigation](#navigation): tabs, menu, command palette. 标签页、菜单、命令面板。
- [Content and feedback](#content-and-feedback): text, code, logs, diffs, Markdown,
  progress, spinner. 文本、代码、日志、差异、进度。
- [Containers and layout](#containers-and-layout): panels, dialogs, flex, grid,
  scrolling, splits, overlays, portals, floating windows. 布局、弹窗、滚动、浮窗。
- [Graphics and motion](#graphics-and-motion): canvas, visual effects, transitions,
  captured dragging. 绘图、动画、拖拽。
- [Composition and missing names](#composition-and-missing-names): charts, toasts,
  custom controls, and helper types that do not draw themselves.

## Controls

| Need and entry point | Key configuration, messages, and boundaries | Example |
| --- | --- | --- |
| Action: `button(label)` | `.on_click(|| Msg::Action)`, `.disabled(bool)`, `.variant(ButtonVariant::Primary)`; keyboard and pointer activation. | [button][button] |
| Single-line editor: `text_input()` | `.value(text).on_change(Msg::Edit)`, `.on_submit(Msg::Submit)`, `.placeholder(...)`; `TextInputVariant::Password` masks display, not callback values. | [text_input][text_input] |
| Multiline editor: `textarea()` | `.value(text).on_change(Msg::Edit)`, `.height(rows)`, `.on_submit(...)`; Unicode editing and scrolling. `TextAreaVariant::Borderless` removes its border. | [textarea][textarea] |
| Boolean option: `checkbox(label)` | `.checked(bool).on_change(Msg::Checked)`; callback receives the new `bool`. | [checkbox][checkbox] |
| Boolean switch: `switch(label)` or `toggle(label)` | Both construct `Switch`; `.checked(bool).on_change(...)`. `toggle` is an alias, not a different widget. | [switch][switch] |
| Visible single choice: `radio_group()` | `.options([RadioOption::new(value, label), ...]).selected(value).on_change(...)`; callback receives `String`. | [radio_group][radio_group] |
| Expandable single choice: `select()` | `.options([SelectOption::new(value, label), ...])`, `.searchable(true)`, `.search_mode(SelectSearchMode::Fuzzy)`, `.on_change(...)`; options open inline. | [select][select] |
| Date navigation: `calendar(initial_date)` | `CalendarDate::new(year, month, day)` returns `Option<CalendarDate>`; `.on_change(...)` and `.on_submit(...)` receive dates. | [calendar][calendar] |

Text fields, checkboxes, and switches need changed values written into application
state and supplied back on the next view. Selection APIs are not interchangeable:
`select` stores its committed value internally and has no `.selected(...)` builder;
`radio_group` and `tabs` accept `.selected(...)` as a preference, with existing
internal navigation state taking precedence. `calendar` takes an initial date.
Give these controls stable keys; inspect their state APIs before implementing a
programmatic selection reset.

## Collections

| Need and entry point | Key configuration, messages, and boundaries | Example |
| --- | --- | --- |
| Navigable rows: `list()` | `.items([ListItem::new(id, label), ...])`, `.height(...)`, `.on_change(...)`, `.on_submit(...)`; `.multi_select(true).on_selection_change(...)` reports selected IDs separately from the navigation cursor. | [list][list] |
| Columns and rows: `data_table()` | `.columns([DataTableColumn::new(title).id(id), ...]).rows([DataTableRow::new(id, cells), ...])`; filtering, sorting, hidden columns, row/cell navigation and multi-selection. | [data_table][data_table] |
| Hierarchy: `tree()` | `.items([TreeItem::new(id, label).child(...), ...])`; expansion, `.on_change(...)`, `.on_submit(...)`, and optional `.on_selection_change(...)`. | [tree][tree] |
| File hierarchy: `file_explorer()` | `FileExplorerItem::file`, `directory`, or `lazy_directory`; `.items(...)`, `.on_open(...)`, `.on_load_children(...)`, multi-selection. The application supplies entries and performs filesystem I/O. | [file_explorer][file_explorer] |
| Large or lazy datasets: `VirtualList::new(source, &state, config, row_builder)` | `source: Arc<S>` implements `DataSource`; use `VirtualListState`, `VirtualListConfig`, and `RowData::{Ready,Loading,Failed}`. `.on_change(...)` returns the next state. | [extensions][extensions] |

For tables, `.filter_query(query)` searches filterable columns, or all cells when
none are marked filterable. `.sort(DataTableSort::new(column_id, direction))`
compares cell text lexicographically; sort numeric/domain values in application
code when needed. `.navigation_mode(DataTableNavigationMode::Cell)` enables cell
navigation with `.on_cell_change(...)`. `.active_row(...)` and `.selected_ids(...)`
support controlled state; keep row IDs stable through filtering and sorting.

Use `VirtualList` when building every row would be expensive. Its row builder is
`|index, RowData<S::Item>, selected| -> widget`; configure fixed `row_height`,
viewport `height`, and `overscan`. Store the state received by `.on_change(...)`
before rebuilding. Rows outside the retained window unmount. A `log_viewer`
accepts all supplied log lines; it is not a replacement for virtualized loading.

## Navigation

| Need and entry point | Key configuration, messages, and boundaries | Example |
| --- | --- | --- |
| Tab strip: `tabs()` | `.tabs([TabItem::new(value, label), ...]).selected(value).on_change(...)`; the application renders the active page separately. | [tabs][tabs] |
| Action menu: `menu()` | `.items([MenuItem::new(id, label), ...])`, `.on_select(...)`, `.on_close(...)`; positioning or modal behavior comes from a container. | [menu][menu] |
| Searchable commands: `command_palette()` | `.items([CommandItem::new(id, label).keywords(...), ...])`, `.search_mode(...)`, `.on_submit(...)`, `.on_close(...)`; filtering is built in. | [command_palette][command_palette] |

To share commands between hotkeys and a palette, use `CommandRouter` with
`App::with_commands_from(...)`; build palette items from the same router and
dispatch the selected ID in update. The [extensions example][extensions] also
shows a palette anchored to a button using `PortalLayer`.

## Content and feedback

| Need and entry point | Key configuration and boundaries | Example |
| --- | --- | --- |
| Text: `label(text)` | `.fg(...)`, `.bold()`, `.align(...)`, `.wrap_word()` or `.wrap_char()`; sizing uses terminal cells. Takes a string, not a `Text` tree. | [label][label] |
| Code: `code_viewer(code)` | `.language("rust")`, `.line_numbers(true)`, `.height(...)`; scrollable, read-only. Enable `syntax-highlighting` for Syntect colors. | [code_viewer][code_viewer] |
| Logs: `log_viewer()` | `.lines([LogLine::new(LogLevel::Info, message), ...]).height(...)`; read-only scrolling with level styling. The application appends/collects log data. | [log_viewer][log_viewer] |
| Existing diff: `diff_viewer(diff_text)` | `.height(...)`; read-only styling for additions, deletions, and hunk headers. Does not calculate a diff between files. | [diff_viewer][diff_viewer] |
| Markdown: `markdown_viewer(markdown)` | `.height(...)`; lightweight line-based styling for headings, lists, quotes, and backtick lines. Delimiters remain visible; not a full Markdown renderer. | [markdown_viewer][markdown_viewer] |
| Completion fraction: `progress_bar(value)` | `value: f64` is a zero-to-one fraction; `.label(...)`, `.width(...)`, and variants through `ProgressBarVariant`. | [progress][progress] |
| Busy indicator: `loading_indicator()` | `.label(...).frame(index)`; the application advances the frame, e.g. with `App::on_frame`. It does not animate itself. | [loading_indicator][loading_indicator] |
| Separator: `divider()` | `.horizontal()` or `.vertical()`, optional `.text(...)` and `DividerVariant`. | [divider][divider] |
| Empty space: `spacer()` | `.fixed(width, height)`, `.min(...)`, `.flex(...)`, or `.fill()` for layout spacing. | [spacer][spacer] |

## Containers and layout

Sizes, padding, gaps, and offsets are terminal cells. Containers accept children
with `.child(...)` unless the entry point below explicitly takes a base/child.
Sizing builders vary by concrete widget; do not assume every widget has `.width()`.

| Need and entry point | Key configuration and boundaries | Example/source |
| --- | --- | --- |
| Linear flex: `row()` and `col()` | `.child(...)`, `.gap(...)`, `.padding(...)`, `.align_items(...)`, `.justify_content(...)`. | [flex][flex] |
| Two-dimensional tracks: `grid()` | `.columns("24 1fr").rows("auto 1fr")`, `GridPlacement` for one-based explicit positions; a subset of CSS grid track syntax. | [grid][grid] |
| Shared-area layers: `stack()` | `.child(...)`; paints children in insertion order in the same allocation. | [stack][stack] |
| Bordered section: `panel()` | `.title(...).padding(...).child(...)`, `.gap(...)`; not `panel(child)`. | [panel][panel] |
| Dialog content: `dialog()` | `.title(...).size(...).child(...)`; includes a Tab focus trap. Use overlay for placement or a modal portal layer for popup isolation. | [dialog][dialog] |
| Expand/collapse: `collapsible(title)` | `.child(...)`, `.expanded(bool).on_toggle(...)`, `.retain_state(bool)`; supply the changed expanded value back from state. | [collapsible][collapsible] |
| Local popup: `overlay(base)` | Add `OverlayLayer` with `.layer(...)`; position relative to the base's allocation. | [overlay][overlay] |
| Root or anchored popup: `portal(base)` | Add `PortalLayer::new(widget)` with `.layer(...)`; `.anchor(WidgetId)`, `.modal(true)`, `.on_dismiss(...)`. | [extensions][extensions] |
| Scrollable subtree: `scroll_view(child)` | Set height/axes and `.focusable(true)` for keyboard scrolling; built-in scrollbars handle pointer dragging. | [scroll][scroll] |
| Scrollable text rows: `scroll_lines(lines)` | Convenience `ScrollView` from strings; also use `.focusable(true)` for keyboard scrolling. | [scroll][scroll] |
| Standalone scrollbar: `scrollbar()` | `.content_size(...).viewport_size(...).offset(...)`; display only, no pointer input. Use `ScrollView` for interactive bars. | [scroll source][scroll-source] |
| Resizable two panes: `split(first, second)` | `.horizontal()` for left/right or `.vertical()` for top/bottom; `.sidebar(width)`, `.ratio(...)`, `.min_first(...)`, `.min_second(...)`. Retained split state takes over after initialization. | [split][split] |
| Desktop-like window host: `window_host(base)` | `.window(floating_window(...).key(...))`; manages window activation, ordering, and constrained geometry. Window keys must be stable and unique. | [windows][windows] |
| Movable/resizable window: `floating_window(content)` | `.default_rect(WindowRect)` or controlled `.rect(...)`; `.on_rect_change(...)`, `.on_visibility_change(...)`, `.on_close(...)`. Use `WindowHost` for a managed set. | [windows][windows] |

`PortalLayer::on_dismiss` and window close callbacks emit messages; the application
must update state and remove the relevant description to close it. `OverlayLayer`
positions within its base; use a root `Portal` when a popup must escape a nested
scroll viewport's clipping.

## Graphics and motion

| Need and entry point | Key configuration and boundaries | Example/source |
| --- | --- | --- |
| Graphics in layout: `canvas(draw_callback)` | Callback receives `&mut Canvas` and `CanvasContext`; `.encoding(...)`, `.viewport(...)`, `.on_pointer(...)`. Use world coordinates for encoding-independent plots. | [canvas][canvas] |
| Widget effects: `visual(child)` | Gradients, sweeps, masks, backdrop effects, `.animation(...)`, `.enter(...)`, `.exit(...)`; the child retains its input behavior. | [visual][visual] |
| Layout/presence animation: `animate(child)` | `.layout(AnimationSpec)`, `.layout_transition(...)`, `.shared(id)`, `.enter(Timeline)`, `.exit(Timeline)`. Use stable keys for identity. | [animated source][animated-source] |
| Captured pointer gestures: `drag_region(content)` | `.on_drag(...)` receives `DragEvent`; use for custom drag handles or draggable content, with cancellation handled explicitly. | [drag source][drag-source] |

Read [graphics.md](graphics.md) for sample/cell/world coordinates, visual timing,
and direct rendering without widgets. `looping_visual_spec(duration)` builds an
`AnimationSpec`; it is not a widget. `CanvasViewport` and `WorldCanvas` transform
and draw graphics but are not separate layout nodes.

## Composition and missing names

Check the catalog and the version-matched public exports before deciding a
component is missing. In this version there are no exported `slider`, `chart`,
or `toast` widget constructors. Use these composition routes when they fit:

- Charts: `canvas` plus `label` for axes/legends and `row`/`grid` for layout.
  `progress_bar` is for one completion fraction, not a general chart.
- Notifications/toasts: store messages in `NotificationCenter` and render them
  with `panel`/`label` in an `overlay` or `portal` layer. The center stores data;
  it does not render a toast automatically.
- Searchable tables: `text_input` plus `data_table().filter_query(...)`.
- File browser with preview: `split(file_explorer().items(entries), code_viewer(code))`,
  with background tasks for I/O.
- Custom controls: compose existing widgets; implement `Widget<Msg>` only when
  the interaction/measurement/drawing requirements need a new primitive.

`SelectionState`, `ListState`, `ContentViewerState`, and similar
state structs support widgets; they are not widgets themselves. `ModalManager`,
`Navigator`, `NotificationCenter`, `CommandRouter`, `FormState`, and `Store` are
application helpers. `Theme`, `Style`, and `Text`/`Line`/`Span` provide style or
text data rather than mounted UI. Public names are listed in
[prelude.rs][prelude-source], [widgets/mod.rs][widgets-source], and
[layout/mod.rs][layout-source].

[button]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/button.rs
[text_input]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/text_input.rs
[textarea]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/textarea.rs
[checkbox]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/checkbox.rs
[switch]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/switch.rs
[radio_group]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/radio_group.rs
[select]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/select.rs
[calendar]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/calendar.rs
[list]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/list.rs
[data_table]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/data_table.rs
[tree]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/tree.rs
[file_explorer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/file_explorer.rs
[extensions]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/basics/extensions.rs
[tabs]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/tabs.rs
[menu]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/menu.rs
[command_palette]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/command_palette.rs
[label]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/label.rs
[code_viewer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/code_viewer.rs
[log_viewer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/log_viewer.rs
[diff_viewer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/diff_viewer.rs
[markdown_viewer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/markdown_viewer.rs
[progress]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/progress.rs
[loading_indicator]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/loading_indicator.rs
[divider]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/divider.rs
[spacer]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/spacer.rs
[flex]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/flex.rs
[grid]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/grid.rs
[stack]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/stack.rs
[panel]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/panel.rs
[dialog]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/dialog.rs
[collapsible]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/collapsible.rs
[overlay]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/overlay.rs
[scroll]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/scroll.rs
[scroll-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/layout/scroll.rs
[split]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/split.rs
[windows]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/layout/windows.rs
[canvas]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/widgets/canvas.rs
[visual]: https://github.com/nidhoggfgg/rsille/blob/main/packages/tui/examples/motion/visual.rs
[animated-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/widgets/motion/animated.rs
[drag-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/widgets/layout/drag_region.rs
[prelude-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/prelude.rs
[widgets-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/widgets/mod.rs
[layout-source]: https://github.com/nidhoggfgg/rsille/blob/main/src/tui/layout/mod.rs
