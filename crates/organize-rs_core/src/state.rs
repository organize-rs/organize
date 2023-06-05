// TODO: Implement typestate for running organize
// https://cliffle.com/blog/rust-typestate/

use jwalk::DirEntry;

use crate::{
    rules::Rule,
    actors::location_walker::{DirEntryData},
};

// States
#[derive(Debug, Clone, Copy, Default)]
pub struct Init;

#[derive(Debug, Clone, Copy, Default)]
pub struct Start;

#[derive(Debug, Default)]
pub struct Inspect {
    entries: Vec<(Rule, DirEntryData)>,
}

impl Inspect {
    pub fn with_entries(entries: Vec<(Rule, DirEntryData)>) -> Self {
        Self { entries }
    }

    pub fn print_entries(&self) {
        self.entries.iter().for_each(|(rule, entries)| {
            entries.print_entries();
            println!("Rule: {rule}");
        })
    }

    pub fn entries(self) -> Vec<(Rule, DirEntryData)> {
        self.entries
    }
}

#[derive(Debug, Default)]
pub struct HandleConflicts {
    entries: Vec<(Rule, DirEntryData)>,
    conflicts: Vec<DirEntry<((), ())>>,
}

impl HandleConflicts {
    pub fn with_entries(entries: Vec<(Rule, DirEntryData)>) -> Self {
        Self {
            entries,
            conflicts: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AskConfirmation;

#[derive(Debug, Clone, Copy, Default)]
pub struct Preview;

#[derive(Debug, Clone, Copy, Default)]
pub struct ApplyActions;

#[derive(Debug, Clone, Copy, Default)]
pub struct Report;

pub trait ProcessingState {}

impl ProcessingState for Init {}
impl ProcessingState for Start {}
impl ProcessingState for Inspect {}
impl ProcessingState for HandleConflicts {}
impl ProcessingState for AskConfirmation {}
impl ProcessingState for Preview {}
impl ProcessingState for ApplyActions {}
impl ProcessingState for Report {}
