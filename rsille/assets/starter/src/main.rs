use rsille::tui::prelude::*;

#[derive(Default)]
struct State {
    name: String,
    count: u32,
}

enum Msg {
    Rename(String),
    Increment,
    Quit,
}

fn update(state: &mut State, message: Msg, ctx: &mut UpdateCtx<Msg>) {
    match message {
        Msg::Rename(name) => state.name = name,
        Msg::Increment => state.count = state.count.saturating_add(1),
        Msg::Quit => ctx.exit(),
    }
}

fn view(state: &State) -> impl Widget<Msg> {
    let name = if state.name.is_empty() {
        "world"
    } else {
        &state.name
    };
    col()
        .gap(1)
        .child(label(format!("Hello, {name}! Count: {}", state.count)))
        .child(
            text_input()
                .key("name")
                .placeholder("Your name")
                .value(&state.name)
                .on_change(Msg::Rename),
        )
        .child(
            row()
                .gap(1)
                .child(button("Count +1").key("count").on_click(|| Msg::Increment))
                .child(button("Quit").key("quit").on_click(|| Msg::Quit)),
        )
        .child(label("Tab: focus | Enter/Space: activate | Esc: exit"))
}

fn main() -> WidgetResult<()> {
    let exit = App::new(State::default())
        .mode(Mode::Inline)
        .update(update)
        .view(view)
        .run()?;
    println!("Final count: {}", exit.state.count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::empty()))
    }

    #[test]
    fn edit_count_resize_and_quit() -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = App::new(State::default())
            .mode(Mode::Inline)
            .update(update)
            .view(view)
            .build()
            .size(60, 14);

        runtime.step(&[Event::Paste("中文👨‍👩‍👧‍👦".into())])?;
        assert_eq!(runtime.focused_key(), Some("name"));
        assert_eq!(runtime.state().name, "中文👨‍👩‍👧‍👦");

        // Backspace removes one grapheme, including a multi-codepoint emoji.
        runtime.step(&[key(KeyCode::Backspace)])?;
        assert_eq!(runtime.state().name, "中文");

        runtime.step(&[key(KeyCode::Tab)])?;
        assert_eq!(runtime.focused_key(), Some("count"));
        runtime.step(&[key(KeyCode::Enter)])?;
        assert_eq!(runtime.state().count, 1);
        let heading: String = (0..runtime.frame().size().width)
            .filter_map(|x| runtime.frame().get((x, 0).into()))
            .map(|cell| cell.symbol().into_owned())
            .collect();
        assert_eq!(heading.trim_end(), "Hello, 中文! Count: 1");

        runtime.step(&[Event::Resize(32, 14)])?;
        assert_eq!(runtime.frame().size().width, 32);
        assert_eq!(runtime.focused_key(), Some("count"));
        assert_eq!(runtime.state().name, "中文");

        runtime.step(&[key(KeyCode::Tab)])?;
        assert_eq!(runtime.focused_key(), Some("quit"));
        assert!(runtime.step(&[key(KeyCode::Enter)])?.should_quit);
        let exit = runtime.into_exit();
        assert_eq!(exit.reason, ExitReason::Requested);
        assert_eq!(exit.state.count, 1);
        Ok(())
    }
}
