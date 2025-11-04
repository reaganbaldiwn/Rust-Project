use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

mod virtual_machine;
use virtual_machine::{Chunk, OpCode, Value, VirtualMachine};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    // Create sample bytecode
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpConstant(Value::ValNumber(3.0)), 1);
    chunk.write(OpCode::OpConstant(Value::ValNumber(4.0)), 1);
    chunk.write(OpCode::OpAdd, 1);
    chunk.write(OpCode::OpReturn, 1);

    let app_result = App::new(chunk).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    vm: VirtualMachine,
    exit: bool,
}

impl App {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            vm: VirtualMachine::new(chunk),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Right => self.vm.step_once(),
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Virtual Machine ".bold());
        let instructions = Line::from(vec![
            " Step ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines = vec![
            Line::from(format!("Instruction Pointer: {}", self.vm.ip)),
            Line::from(format!("Stack: {:?}", self.vm.stack)),
            Line::from("Chunk Code:".to_string()),
        ];

        for (i, op) in self.vm.chunk.code.iter().enumerate() {
            let pointer = if self.vm.ip > 0 && i == self.vm.ip - 1 { "→" } else { " " };
            lines.push(Line::from(format!(" {} {:?}", pointer, op)));
        }

        Paragraph::new(Text::from(lines))
            .block(block)
            .render(area, buf);
    }
}
