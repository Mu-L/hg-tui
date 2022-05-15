use crate::app::{App, AppMode};
use crate::app_global::THEME;
use crate::theme::ThemeStyle;
use crate::widget::projectdetail::ProjectDetail;
use crate::widget::{Content, Input, Popup, StatusLine};

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};

use tui::text::Text;
use tui::widgets::{Block, Paragraph};

pub fn redraw(app: &mut App) {
    let terminal = &mut app.terminal;

    terminal
        .draw(|f| {
            let theme_style = THEME.get().unwrap();
            f.render_widget(Block::default().style(theme_style.background), f.size());

            if app.mode == AppMode::Detail {
                let area = centered_rect(80, 50, f.size());
                f.render_stateful_widget(ProjectDetail {}, area, &mut app.project_detail);
            } else {
                // layout[0] => title
                // layout[1] => input
                // layout[2] => content
                // layout[3] => status line
                let layout = Layout::default()
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Max(90),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                f.render_widget(title(theme_style), layout[0]);

                let input_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(40),
                            Constraint::Percentage(30),
                        ]
                        .as_ref(),
                    )
                    .split(layout[1])[1];

                f.render_stateful_widget(Input {}, input_layout, &mut app.input);
                if let AppMode::Search = app.mode {
                    f.set_cursor(
                        input_layout.x + app.input.width() as u16 + 1,
                        input_layout.y + 1,
                    )
                }

                f.render_stateful_widget(Content {}, layout[2], &mut app.content);

                f.render_stateful_widget(StatusLine {}, layout[3], &mut app.statusline);
                // popup
                if app.mode == AppMode::Popup {
                    let area = centered_rect(50, 50, f.size());

                    f.render_stateful_widget(Popup {}, area, &mut app.popup);
                }
            }
        })
        .unwrap();
}

fn title(theme_style: &ThemeStyle) -> Paragraph<'static> {
    Paragraph::new(
        // Text::from(Spans::from(vec![
        // Span::styled("HelloGiHub", Style::default().fg(Color::Yellow)),
        // Span::raw(""),
        Text::styled(
            "HelloGiHub\n分享 GitHub 上有趣、入门级的开源项目",
            theme_style.title,
        ),
    )
    .alignment(Alignment::Center)
    .block(Block::default())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
