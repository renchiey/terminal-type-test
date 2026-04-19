use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

use crate::app::{App, Word};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "▗▄▄▄▖▗▄▄▄▖▗▄▄▖ ▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖ ▗▄▖ ▗▖       ▗▄▄▄▖▗▖  ▗▖▗▄▄▖ ▗▄▄▄▖
  █  ▐▌   ▐▌ ▐▌▐▛▚▞▜▌  █  ▐▛▚▖▐▌▐▌ ▐▌▐▌         █   ▝▚▞▘ ▐▌ ▐▌▐▌   
  █  ▐▛▀▀▘▐▛▀▚▖▐▌  ▐▌  █  ▐▌ ▝▜▌▐▛▀▜▌▐▌         █    ▐▌  ▐▛▀▘ ▐▛▀▀▘
  █  ▐▙▄▄▖▐▌ ▐▌▐▌  ▐▌▗▄█▄▖▐▌  ▐▌▐▌ ▐▌▐▙▄▄▖      █    ▐▌  ▐▌   ▐▙▄▄▖",
        Style::default().fg(Color::White),
    ))
    .block(title_block)
    .centered();

    frame.render_widget(title, chunks[0]);
    render_input_box(frame, chunks[1], app);
}

fn render_input_box(frame: &mut Frame, area: Rect, app: &App) {
    let text_box = Block::bordered()
        .border_type(BorderType::Thick)
        .title("Type Here");

    let mut spans: Vec<Span> = Vec::new();
    let mut check = false;

    for (i, word) in app.words.iter().enumerate() {
        if i > 0 {
            if !check {
                spans.push("  ".into());
            } else {
                spans.push(" ".into());
                check = false;
            }
        }

        if i == app.current_word_index {
            let mut temp = render_word(&word);

            if word.input.len() >= word.actual.len() {
                temp.push(Span::from(" ").bg(Color::White));
                check = true;
            } else {
                temp[word.input.len()] = temp[word.input.len()].clone().black().bg(Color::White);
            }

            spans.extend(temp);
            continue;
        }
        spans.extend(render_word(&word));
    }
    let text = Text::from(Line::from(spans));

    let block = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .block(text_box);

    frame.render_widget(block, area);
}

fn render_word<'a>(word: &Word) -> Vec<Span<'a>> {
    let mut res: Vec<Span> = Vec::new();
    let mut actual_chars = word.actual.chars();

    for input_char in word.input.chars() {
        match actual_chars.next() {
            Some(a) if a == input_char => res.push(a.to_string().white()),
            Some(a) => res.push(a.to_string().red()),
            None => res.push(input_char.to_string().red()),
        }
    }

    for c in actual_chars {
        res.push(c.to_string().dark_gray());
    }

    res
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
