use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::form_data::{FocusState, FormData};
use crate::ui::{get_orange_accent, get_orange_color};

pub struct EnvSetupView<'a> {
    pub form_data: &'a FormData,
}

pub fn render_env_setup(frame: &mut Frame, view: &EnvSetupView<'_>) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    let data = view.form_data;

    let title = Paragraph::new("🔧 HV Collector Configuration")
        .style(
            Style::default()
                .fg(get_orange_color())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(get_orange_accent())),
        )
        .centered();
    frame.render_widget(title, chunks[0]);

    let mut form_lines = vec![];

    // Helper function to render a field
    let render_field = |idx: usize| -> Line {
        let is_focused = matches!(&data.focus_state, FocusState::Field(i) if *i == idx);
        let field_style = if is_focused {
            Style::default()
                .fg(Color::Black)
                .bg(get_orange_color())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let label = data.get_field_label(idx);

        // Get the appropriate field value based on index
        let value = match idx {
            0 => &data.postgres_host,
            1 => &data.postgres_user,
            2 => &data.postgres_password,
            3 => &data.postgres_port,
            4 => &data.postgres_db,
            5 => &data.postgres_schema,
            6 => &data.hypervisor_host,
            7 => &data.hypervisor_user,
            8 => &data.hypervisor_password,
            9 => &data.interval_seconds,
            10 => &data.prometheus_url,
            11 => &data.log_retention_days,
            12 => &data.data_retention_days,
            _ => &data.postgres_host,
        };

        let is_password = data.is_password_field(idx);
        let is_required = data.is_field_required(idx);

        let display_value = if is_password && !value.is_empty() {
            "*".repeat(value.len())
        } else if value.is_empty() {
            "<empty>".to_string()
        } else {
            value.clone()
        };

        let cursor = if is_focused { "▶" } else { " " };
        let required_mark = if is_required { " *" } else { "" };

        Line::from(vec![
            Span::styled(cursor, field_style),
            Span::raw(" "),
            Span::styled(format!("{}{}", label, required_mark), field_style),
            Span::raw(": "),
            Span::styled(display_value, field_style),
        ])
    };

    // PostgreSQL Section
    form_lines.push(Line::from(Span::styled(
        "━━━ PostgreSQL Database ━━━",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    form_lines.push(Line::from(""));
    for i in 0..6 {
        form_lines.push(render_field(i));
    }
    form_lines.push(Line::from(""));

    // Hypervisor Section
    form_lines.push(Line::from(Span::styled(
        "━━━ Hypervisor/SSH Settings ━━━",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )));
    form_lines.push(Line::from(""));
    for i in 6..9 {
        form_lines.push(render_field(i));
    }
    form_lines.push(Line::from(""));

    // Collector Section
    form_lines.push(Line::from(Span::styled(
        "━━━ Collector Settings ━━━",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));
    form_lines.push(Line::from(""));
    for i in 9..13 {
        form_lines.push(render_field(i));
    }
    form_lines.push(Line::from(""));

    if !data.error_message.is_empty() {
        form_lines.push(Line::from(Span::styled(
            &data.error_message,
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        form_lines.push(Line::from(""));
    }

    let form = Paragraph::new(form_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(get_orange_accent()))
            .title("Configuration Fields (* = required)")
            .title_style(
                Style::default()
                    .fg(get_orange_color())
                    .add_modifier(Modifier::BOLD),
            ),
    );
    frame.render_widget(form, chunks[1]);

    // Buttons
    let save_focused = matches!(&data.focus_state, FocusState::SaveButton);
    let cancel_focused = matches!(&data.focus_state, FocusState::CancelButton);

    let save_style = if save_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    };

    let cancel_style = if cancel_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    };

    let button_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(" Save ", save_style),
        Span::raw("  "),
        Span::styled(" Cancel ", cancel_style),
        Span::raw("  "),
        Span::styled(
            "↑↓ Tab to navigate | Type to edit",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let buttons = Paragraph::new(button_line).centered();
    frame.render_widget(buttons, chunks[2]);
}
