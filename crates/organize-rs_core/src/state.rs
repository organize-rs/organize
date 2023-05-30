// TODO: Implement typestate for running organize
// https://cliffle.com/blog/rust-typestate/

// States
#[derive(Debug, Clone, Copy, Default)]
pub struct Init;
#[derive(Debug, Clone, Copy, Default)]
pub struct Start;
#[derive(Debug, Clone, Copy, Default)]
pub struct EntriesCollected;

pub trait ProcessingState {}

impl ProcessingState for Init {}
impl ProcessingState for Start {}
impl ProcessingState for EntriesCollected {}

// Flux architecture
// User action
// Dispatcher
// Store
// View
