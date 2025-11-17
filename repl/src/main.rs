use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use tabula_compiler::Compiler;
use tabula_runtime::VM;

struct ReplState {
    history: Vec<String>,
    current_input: String,
    output: Vec<String>,
    compiler: Compiler,
    vm: VM,
}

impl ReplState {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_input: String::new(),
            output: Vec::new(),
            compiler: Compiler::new(),
            vm: VM::new(),
        }
    }

    fn execute(&mut self, input: &str) {
        if input.trim().is_empty() {
            return;
        }

        self.history.push(input.to_string());
        self.output.push(format!("> {}", input));

        // Try to parse and execute
        match self.compiler.lexer.tokenize(input) {
            Ok(tokens) => {
                match self.compiler.parser.parse(tokens) {
                    Ok(ast) => {
                        // Execute using interpreter
                        use tabula_compiler::codegen::Interpreter;
                        let mut interpreter = Interpreter::new();
                        match interpreter.interpret(&ast) {
                            Ok(_) => {
                                self.output.push("✓ Executed successfully".to_string());
                            }
                            Err(e) => {
                                self.output.push(format!("✗ Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.output.push(format!("✗ Parse error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.output.push(format!("✗ Lex error: {}", e));
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ReplState::new();
    state.output.push("Tabula REPL v0.1.0".to_string());
    state.output.push("Type 'exit' to quit".to_string());
    state.output.push("".to_string());

    loop {
        terminal.draw(|f| ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Enter => {
                        if !state.current_input.trim().is_empty() {
                            if state.current_input.trim() == "exit" {
                                break;
                            }
                            state.execute(&state.current_input);
                            state.current_input.clear();
                        }
                    }
                    KeyCode::Char(c) => {
                        state.current_input.push(c);
                    }
                    KeyCode::Backspace => {
                        state.current_input.pop();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, state: &ReplState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Output area
    let output_items: Vec<ListItem> = state
        .output
        .iter()
        .map(|line| ListItem::new(line.as_str()))
        .collect();
    let output_list = List::new(output_items)
        .block(Block::default().borders(Borders::ALL).title("Output"))
        .style(Style::default().fg(Color::White));
    f.render_widget(output_list, chunks[0]);

    // Input area
    let input = Paragraph::new(state.current_input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);
    f.render_widget(input, chunks[1]);
    f.set_cursor(
        chunks[1].x + state.current_input.len() as u16 + 1,
        chunks[1].y + 1,
    );

    // Status
    let status = Paragraph::new(format!(
        "History: {} | Ready",
        state.history.len()
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .style(Style::default().fg(Color::Green));
    f.render_widget(status, chunks[2]);
}

