use ratatui::{
    widgets::{Borders},
    prelude::{Direction,Constraint,Layout},
    layout::Alignment, style::{Color, Style}, symbols, widgets::{canvas::*, Block, BorderType, Paragraph}, Frame
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    ).split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );


    let inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(70),
        Constraint::Percentage(30),
    ])
    .split(main_layout[1]);
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                What you guys are referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.
                Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX.
                Many computer users run a modified version of the GNU system every day, without realizing it. 
                Through a peculiar turn of events, the version of GNU which is widely used today is often called Linux and many of its users are not aware that it is basically the GNU system, developed by the GNU Project.\n\
                There really is a Linux, and these people are using it, \n
                but it is just a part of the system they use. \n\
                Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run.
                The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. 
                Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. 
                All the so-called Linux distributions are really distributions of GNU/Linux.
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .block(
            Block::bordered()
                .title("Hello Welcome to 1337")
                .title_alignment(Alignment::Center)
                .border_set(symbols::border::DOUBLE)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightRed).bg(Color::Black))
        .centered(),
        inner_layout[0],
    );
   // let area = Rect::new(0, 0, 5, 5);
    frame.render_widget(
        Canvas::default()
        .block(Block::bordered().title("Canvas"))
        
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();
            ctx.draw(&Line {
                x1: 0.0,
                y1: 10.0,
                x2: 10.0,
                y2: 10.0,
                color: Color::White,
            });
            ctx.draw(&Rectangle {
                x: 10.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
                color: Color::Red,
            });
        }),inner_layout[1]);
}
