// TODO: Implement typestate for running organize
// https://cliffle.com/blog/rust-typestate/

use jwalk::DirEntry;

use crate::{
    actions::conflicts::ConflictResolutionKind, actors::location_walker::DirEntryData,
    rules::SingleRule,
};

// States
#[derive(Debug, Clone, Copy, Default)]
pub struct Initialize;

#[derive(Debug, Clone, Copy, Default)]
pub struct Filtering;

#[derive(Debug, Default)]
pub struct Inspection {
    entries: Vec<(SingleRule, DirEntryData)>,
}

#[derive(Debug, Default)]
pub struct ConflictHandling {
    entries: Vec<(SingleRule, DirEntryData)>,
    conflicts: Vec<DirEntry<((), ())>>,
}

impl Inspection {
    pub fn with_entries(entries: Vec<(SingleRule, DirEntryData)>) -> Self {
        Self { entries }
    }

    pub fn print_entries(&self) {
        self.entries.iter().for_each(|(rule, entries)| {
            entries.print_entries();
            println!("Rule: {rule}");
        })
    }

    pub fn entries(self) -> Vec<(SingleRule, DirEntryData)> {
        self.entries
    }
}

impl ConflictHandling {
    pub fn with_entries(entries: Vec<(SingleRule, DirEntryData)>) -> Self {
        Self {
            entries,
            conflicts: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct AskConfirmation;

#[derive(Debug, Default)]
pub struct ActionPreview {
    entries: Vec<(SingleRule, DirEntryData)>,
    conflicts: Option<Vec<ConflictResolutionKind>>,
}

impl ActionPreview {
    pub fn with_entries(entries: Vec<(SingleRule, DirEntryData)>) -> Self {
        Self {
            entries,
            conflicts: None,
        }
    }
    pub fn entries(self) -> Vec<(SingleRule, DirEntryData)> {
        self.entries
    }
}

#[derive(Debug, Default)]
pub struct ActionApplication {
    entries: Vec<(SingleRule, DirEntryData)>,
    conflicts: Option<Vec<ConflictResolutionKind>>,
}

impl ActionApplication {
    pub fn with_entries(entries: Vec<(SingleRule, DirEntryData)>) -> Self {
        Self {
            entries,
            conflicts: None,
        }
    }
    pub fn entries(self) -> Vec<(SingleRule, DirEntryData)> {
        self.entries
    }
}

#[derive(Debug, Clone, Default)]
pub struct Reporting;

pub trait ProcessingStage {}

impl ProcessingStage for Initialize {}
impl ProcessingStage for Filtering {}
impl ProcessingStage for Inspection {}
impl ProcessingStage for ActionPreview {}
impl ProcessingStage for AskConfirmation {}
impl ProcessingStage for ActionApplication {}
impl ProcessingStage for ConflictHandling {}
impl ProcessingStage for Reporting {}
