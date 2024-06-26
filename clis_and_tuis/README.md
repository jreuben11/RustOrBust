
# basics 
## grss
```bash
cargo add clap --features derive
...
env RUST_LOG=info cargo run clap Cargo.toml 
```
### [Cargo.toml](basics/grrs/Cargo.toml)
```toml
[dependencies]
anyhow = "1.0.81"
clap = { version = "4.5.4", features = ["derive"] }
env_logger = "0.11.3"
indicatif = "0.17.8"
log = "0.4.21"

[dev-dependencies]
assert_cmd = "2.0.14"
assert_fs = "1.1.1"
predicates = "3.1.0"
```
### [main.rs](basics/grrs/src/main.rs)
- imports
```rust
use clap::Parser;
use anyhow::{Context, Result};
use std::io::{Write};
use std::{thread, time::Duration};
use log::{info, warn};
```
- CLI arg parser
```rust
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```
- main
```rust
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), CustomError> {
fn main() -> Result<()> {
    ...
    Ok(())
}
```
- grep functionality: read file from path and write out matched lines
```rust
// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
let args = Cli::parse();
let content = std::fs::read_to_string(&args.path)
    //.map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
let handle = std::io::stdout().lock();
find_matches(&content, &args.pattern, handle)?;
```
- progress bar:
```rust
let n = 10;
let pb = indicatif::ProgressBar::new(n);
for i in 0..n {
    thread::sleep(Duration::from_millis(n * 50));
    pb.println(format!("[+] finished #{}", i));
    pb.inc(1);
}
pb.finish_with_message("done");
```
- logger:
```rust
env_logger::init();
info!("shutting down");
warn!("blah!");
```
### [tests/cli.rs](basics/grrs/tests/cli.rs) - integration tests
```rust
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;
...
let file = assert_fs::NamedTempFile::new("sample.txt")?;
file.write_str("A test\nActual content\nMore content\nAnother test")?;

let mut cmd = Command::cargo_bin("grrs")?;
cmd.arg("test").arg(file.path());
cmd.assert()
    .success()
    .stdout(predicate::str::contains("test\nAnother test"));
```

## misc
### [Cargo.toml](basics/misc/Cargo.toml)
```toml
[dependencies]
anyhow = "1.0.81"
clap = { version = "4.5.4", features = ["derive"] }
config = "0.14.0"
crossbeam-channel = "0.5.12"
ctrlc = "3.4.4"
lazy_static = "1.4.0"
serde = {version="1.0.197", features = ["derive"]}
serde_derive = "1.0.197"
serde_json = "1.0.115"
signal-hook = "0.3.17"
```
### [main.rs](basics/misc/src/main.rs)
- imports:
```rust
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;
use config::{Config};
use std::collections::HashMap;
use clap::Parser;
use serde_json::json;
```
- config:
```rust
lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("APP_NAME").separator("_"))
        .build()
        .unwrap();
}
pub fn get_from_config<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    // You shouldn't probably do it like that and actually handle that error that might happen
    // here, but for the sake of simplicity, we do it like this here
    CONFIG.get::<T>(key).unwrap()
}

...

 // Print out our settings (as a HashMap)
println!("{:?}",&<Config as Clone>::clone(&CONFIG).try_deserialize::<HashMap<String, String>>().unwrap());
let duration = get_from_config::<u64>("duration");
```
- signal handler with crossbeam:
```rust
fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

...

let ctrl_c_events = ctrl_channel()?;
let ticks = tick(Duration::from_secs(get_from_config::<u64>("duration")));
loop {
    select! {
        recv(ticks) -> _ => {
            println!("working!");
        }
        recv(ctrl_c_events) -> _ => {
            println!();
            println!("Goodbye!");
            break;
        }
    }
}
```
# clap
## [quickstart](clap/quickstart/src/main.rs)
```bash
cargo run -- --help
cargo run -- --name=bob --count=3
cargo run -- -n=bob -c=3
```
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```
## [derive](clap/derive/src/main.rs)
- `cargo run -- test --list` - subcommands
## [builder](clap/builder/src/main.rs)
- `cargo run -- test --list` - subcommands

# RataTUI
## hello-ratatui
- [Cargo.toml](ratatui/hello-ratatui/Cargo.toml)
```toml
[dependencies]
crossterm = "0.27.0"
ratatui = "0.26.1"

```
- [main.rs](ratatui/hello-ratatui/src/main.rs)
```rust
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main loop
    loop {
        // draw the UI
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        // handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
```

## counter app
- **ALOT TO ABSORB !!!**
- [Cargo.toml](ratatui/ratatui-counter-app/Cargo.toml)
```toml
[dependencies]
color-eyre = "0.6.3"
crossterm = "0.27.0"
ratatui = "0.26.1"
```
- [main.rs](ratatui/ratatui-counter-app/src/main.rs)
```rust
// use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use color_eyre::{
    eyre::{bail, WrapErr},
};
mod errors;
mod tui;
```
- [tui.rs](ratatui/ratatui-counter-app/src/tui.rs)
```rust
use std::io::{self, stdout, Stdout};
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
```
- [errors.rs](ratatui/ratatui-counter-app/src/errors.rs)
```rust
use std::panic;
use color_eyre::{config::HookBuilder, eyre};
use crate::tui;
```

## json editor
### [Cargo.toml](ratatui/ratatui-json-editor/Cargo.toml)
### [main.rs](ratatui/ratatui-json-editor/src/main.rs)
- dependencies:
```rust
use std::{error::Error, io};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};
```
- main:
```rust
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app); // app loop: draw and set state                   <--


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
```
```rust
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match // app states and KeyCode::xxx ...
```
### [app.rs](ratatui/ratatui-json-editor/src/app.rs)
- state:
```rust
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}
```
### [ui.rs](ratatui/ratatui-json-editor/src/ui.rs)
- the magic !
```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
    ...
    // layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3),Constraint::Min(1),Constraint::Length(3),])
        .split(f.size());
    // render title block widget
    let title_block = Block::default().borders(Borders::ALL).style(Style::default());
    let title = Paragraph::new(Text::styled("xxx",Style::default().fg(Color::Green),))block(title_block);
    f.render_widget(title, chunks[0]);
    // render list items widget
    let mut list_items = Vec::<ListItem>::new();
    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }
    let list = List::new(list_items);
    f.render_widget(list, chunks[1]);
}


```
## Async Ratatui Counter
### [Cargo.toml](ratatui/async-ratatui-counter/Cargo.toml)
```toml
[dependencies]
better-panic = "*"
clap = { version = "*", features = [
    "derive",
    "cargo",
    "wrap_help",
    "unicode",
    "string",
    "unstable-styles",
] }
color-eyre = "*"
config = "*"
crossterm = { version = "*", features = ["serde", "event-stream"] }
derive_deref = "*"
directories = "*"
futures = "*"
human-panic = "*"
json5 = "*"
lazy_static = "*"
libc = "*"
log = "*"
pretty_assertions = "*"
ratatui = { version = "*", features = ["serde", "macros"] }
serde = { version = "*", features = ["derive"] }
serde_json = "*"
signal-hook = "*"
strip-ansi-escapes = "*"
tokio = { version = "*", features = ["full"] }
tokio-util = "*"
tracing = "*"
tracing-error = "*"
tracing-subscriber = { version = "*", features = ["env-filter", "serde"] }
```
### [tu.rs](ratatui/async-ratatui-counter/src/tui.rs)
- imports:
```rust
use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste,
        EnableMouseCapture, Event as CrosstermEvent, KeyEvent, KeyEventKind,
        MouseEvent,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
```
- logic
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event { ... }

pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
    pub frame_rate: f64,
    pub tick_rate: f64,
    // pub mouse: bool,
    // pub paste: bool,
}

impl Tui {
    pub fn new() -> Result<Self> {
        ...
        let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let task = tokio::spawn(async {});
        Ok(Self { ... })
    }
    pub fn frame_rate(mut self, frame_rate: f64) -> Self { ... }
    pub fn tick_rate(mut self, tick_rate: f64) -> Self { ... }

    pub fn start(&mut self) {
        ...
        let _event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            _event_tx.send(Event::Init).unwrap();
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    ...
                    match ...
                        CrosstermEvent::XXX(xxx) =>  _event_tx.send(Event::YYY(xxx)).unwrap(),
                        ...
                }
            }
        }
    }
    pub fn stop(&self) -> Result<()> { }

        pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!( std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
        ...
        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
            ...
        }
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.event_rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;
    fn deref(&self) -> &Self::Target {...}
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {...}
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
```
### [main.rs](ratatui/async-ratatui-counter/src/main.rs)
- imports:
```rust
mod tui;

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{self, UnboundedSender};
use tui::Event;
```
- logic:
```rust
// App state
struct App {
    counter: i64,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
}

// App actions
#[derive(Clone)]
pub enum Action { ... }

// App ui render function
fn ui(f: &mut Frame, app: &mut App) { 
    let area = f.size();
    f.render_widget( ... );
    ... 
}

fn get_action(_app: &App, event: Event) -> Action {
    match event { ... }
}

fn update(app: &mut App, action: Action) {
    match action { ... }
}


async fn run() -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel(); // new

    // ratatui terminal
    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
    tui.enter()?;

    // application state
    let mut app = App {
        ...
        action_tx: action_tx.clone(),
    };

    loop {
        let e = tui.next().await?;
        match e {
            ...
            tui::Event::Key(_) => {
                let action = get_action(&app, e);
                action_tx.send(action.clone())?;
            }
            _ => {}
        };

        while let Ok(action) = action_rx.try_recv() {
            // application update
            update(&mut app, action.clone());
            // render only when we receive Action::Render
            if let Action::Render = action {
                tui.draw(|f| {
                    ui(f, &mut app);
                })?;
            }
        }

        // application exit
        if app.should_quit {
            break;
        }
    }
    tui.exit()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;
    result?;
    Ok(())
}

```

