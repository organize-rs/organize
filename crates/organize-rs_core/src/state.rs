// TODO: Implement typestate for running organize
// https://cliffle.com/blog/rust-typestate/

// States
struct Start;
struct FilterRunning;
struct CollectingEntries;

trait ProcessingState {}

impl ProcessingState for Start {}
impl ProcessingState for FilterRunning {}
impl ProcessingState for CollectingEntries {}

// Flux architecture
// User action
// Dispatcher
// Store
// View
