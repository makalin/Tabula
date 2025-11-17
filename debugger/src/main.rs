use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use std::path::PathBuf;
use tabula_compiler::{ast::*, Compiler};

#[derive(Parser)]
#[command(name = "tabula-debug")]
#[command(about = "Tabula Debugger")]
#[command(version)]
struct Cli {
    /// Source file to debug
    file: PathBuf,
}

struct DebuggerState {
    source: String,
    ast: Program,
    breakpoints: Vec<usize>,
    current_line: usize,
    variables: Vec<(String, String)>,
    call_stack: Vec<String>,
    compiler: Compiler,
}

impl DebuggerState {
    fn new(file: PathBuf) -> anyhow::Result<Self> {
        let source = std::fs::read_to_string(&file)?;
        let compiler = Compiler::new();
        let tokens = compiler.lexer.tokenize(&source)?;
        let ast = compiler.parser.parse(tokens)?;

        Ok(Self {
            source,
            ast,
            breakpoints: Vec::new(),
            current_line: 0,
            variables: Vec::new(),
            call_stack: Vec::new(),
            compiler,
        })
    }

    fn toggle_breakpoint(&mut self, line: usize) {
        if self.breakpoints.contains(&line) {
            self.breakpoints.retain(|&x| x != line);
        } else {
            self.breakpoints.push(line);
        }
    }

    fn step_over(&mut self) {
        self.current_line += 1;
    }

    fn step_into(&mut self) {
        // TODO: Implement step into
        self.current_line += 1;
    }

    fn continue_execution(&mut self) {
        // TODO: Continue until next breakpoint
        self.current_line += 1;
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut state = DebuggerState::new(cli.file)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => state.step_over(),
                    KeyCode::Char('i') => state.step_into(),
                    KeyCode::Char('c') => state.continue_execution(),
                    KeyCode::Char('b') => {
                        state.toggle_breakpoint(state.current_line);
                    }
                    KeyCode::Esc => break,
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

fn ui(f: &mut Frame, state: &DebuggerState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    let tabs = Tabs::new(vec!["Source", "Variables", "Call Stack", "Breakpoints"])
        .block(Block::default().borders(Borders::ALL).title("Debugger"))
        .style(Style::default().fg(Color::White))
        .select(0)
        .divider("|");
    f.render_widget(tabs, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // Source code view
    let source_lines: Vec<ListItem> = state
        .source
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let style = if i == state.current_line {
                Style::default().bg(Color::Blue)
            } else if state.breakpoints.contains(&i) {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("{} {}", i + 1, line)).style(style)
        })
        .collect();
    let source_list = List::new(source_lines)
        .block(Block::default().borders(Borders::ALL).title("Source"))
        .style(Style::default().fg(Color::White));
    f.render_widget(source_list, main_chunks[0]);

    // Variables view
    let var_items: Vec<ListItem> = state
        .variables
        .iter()
        .map(|(name, value)| ListItem::new(format!("{} = {}", name, value)))
        .collect();
    let var_list = List::new(var_items)
        .block(Block::default().borders(Borders::ALL).title("Variables"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(var_list, main_chunks[1]);

    // Help text
    let help = Paragraph::new("q: quit | s: step | i: step into | c: continue | b: breakpoint")
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(help, chunks[1]);
}

