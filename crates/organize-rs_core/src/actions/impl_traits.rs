use std::fmt::Display;

use crate::actions::{ActionKind, ActionResultKind};

impl Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionKind::NoAction => {
                write!(
                    f,
                    "
    Filter: NoAction
    "
                )
            }
            ActionKind::Confirm { msg: text, vars } => {
                write!(
                    f,
                    "
    Filter: Confirm
    
    Arguments:
    text: {text:?},
    vars: {vars:?}
            "
                )
            }
            ActionKind::Copy {
                dst,
                on_conflict,
                rename_template,
                filesystem,
            } => {
                write!(
                    f,
                    "
    Filter: Copy
                
    Arguments: 
    dst: {},
    on_conflict: {on_conflict},
    rename_template: {rename_template:?},
    filesystem: {filesystem:?}
            ",
                    dst.display()
                )
            }
            ActionKind::Delete {} => {
                write!(
                    f,
                    "
    Filter: Delete 
                
    Arguments: 
            "
                )
            }
            ActionKind::Echo { msg } => {
                write!(
                    f,
                    "
    Filter: Echo 
                
    Arguments: 
    msg: {msg}
            "
                )
            }
            ActionKind::Move {
                dst,
                on_conflict,
                rename_template,
                filesystem,
            } => {
                write!(
                    f,
                    "
    Filter: Move
                
    Arguments: 
    dst: {},
    on_conflict: {on_conflict},
    rename_template: {rename_template:?},
    filesystem: {filesystem:?}
            ",
                    dst.display()
                )
            }
            ActionKind::Rename {
                name,
                on_conflict,
                rename_template,
            } => {
                write!(
                    f,
                    "
    Filter: Rename 
                
    Arguments: 
    name: {name},
    on_conflict: {on_conflict},
    rename_template: {rename_template:?}
            "
                )
            }
            ActionKind::Symlink { dst } => {
                write!(
                    f,
                    "
    Filter: Symlink 
                
    Arguments: 
    dst: {},
            ",
                    dst.display()
                )
            }
            ActionKind::Trash => {
                write!(
                    f,
                    "
    Filter: Symlink 
            "
                )
            }
            ActionKind::Write {
                txt,
                file,
                mode,
                newline,
                clear_before_first_write,
                filesystem,
            } => write!(
                f,
                "
    Filter: Write

    Arguments:
    txt: {txt},
    file: {},
    mode: {mode},
    newline: {newline:?},
    clear_before_first_write: {clear_before_first_write:?},
    filesystem: {filesystem:?}
            ",
                file.display()
            ),
            ActionKind::Shell {
                command,
                run_in_simulation,
                ignore_errors,
                simulation_output,
                simulation_returncode,
            } => write!(
                f,
                "
    Filter: Shell

    Arguments:
    command: {command},
    run_in_simulation: {run_in_simulation},
    ignore_errors: {ignore_errors},
    simulation_output: {simulation_output:?},
    simulation_returncode: {simulation_returncode},
            "
            ),
        }
    }
}

impl Default for ActionKind {
    fn default() -> Self {
        Self::NoAction
    }
}

impl Default for ActionResultKind {
    fn default() -> Self {
        Self::Successful
    }
}
