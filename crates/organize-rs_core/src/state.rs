// TODO: Implement typestate for running organize
// https://cliffle.com/blog/rust-typestate/

use jwalk::DirEntry;

use crate::{rules::Rule, FilteredFileWalker};

// States
#[derive(Debug, Clone, Copy, Default)]
pub struct Init;

#[derive(Debug, Clone, Copy, Default)]
pub struct Start;

#[derive(Debug, Default)]
pub struct EntriesCollected {
    entries: Vec<(Rule, FilteredFileWalker)>,
}

impl EntriesCollected {
    pub fn with_entries(entries: Vec<(Rule, FilteredFileWalker)>) -> Self {
        Self { entries }
    }

    pub fn print_entries(&self) {
        self.entries.iter().for_each(|(rule, walker)| {
            walker.print_entries();
            println!("Rule: {rule}");
        })
    }
}

#[derive(Debug, Default)]
pub struct ConflictHandling {
    entries: Vec<(Rule, FilteredFileWalker)>,
    conflicts: Vec<DirEntry<((), ())>>,
}

pub trait ProcessingState {}

impl ProcessingState for Init {}
impl ProcessingState for Start {}
impl ProcessingState for EntriesCollected {}
impl ProcessingState for ConflictHandling {}

// Flux architecture
// User action
// Dispatcher
// Store
// View
