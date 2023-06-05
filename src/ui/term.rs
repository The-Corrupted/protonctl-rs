use std::{io, thread, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
