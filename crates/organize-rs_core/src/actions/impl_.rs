use std::path::Path;

use jwalk::{ClientState, DirEntry};

use crate::{
    actions::{ActionClosure, ActionKind, ActionResultKind},
    filesystem::{move_to_trash, remove_irrecoverably, symlink_to},
};

impl ActionKind {
    pub fn get_action<C: ClientState>(&self) -> ActionClosure<C> {
        match self {
            ActionKind::NoAction => self.action_no_action(),
            ActionKind::Trash => self.action_move_to_trash(),
            ActionKind::Delete => self.action_delete(),
            ActionKind::Symlink { dst } => self.action_symlink(dst),
            ActionKind::Copy {
                dst: _,
                on_conflict: _,
                rename_template: _,
                filesystem: _,
            } => todo!("not implemented (yet)!"),
            ActionKind::Rename {
                name: _,
                on_conflict: _,
                rename_template: _,
            } => todo!("not implemented (yet)!"),
            ActionKind::Move {
                dst: _,
                on_conflict: _,
                rename_template: _,
                filesystem: _,
            } => todo!("not implemented (yet)!"),
            ActionKind::Confirm { text: _, vars: _ } => todo!("not implemented (yet)!"),
            ActionKind::Echo { msg: _ } => todo!("not implemented (yet)!"),
            ActionKind::Write {
                txt: _,
                file: _,
                mode: _,
                newline: _,
                clear_before_first_write: _,
                filesystem: _,
            } => todo!("not implemented (yet)!"),
            ActionKind::Shell {
                command: _,
                run_in_simulation: _,
                ignore_errors: _,
                simulation_output: _,
                simulation_returncode: _,
            } => todo!(),
            #[cfg(target_os = "osx")]
            ActionKind::MacOsTags { tags } => todo!(),
        }
    }

    fn action_no_action<C: ClientState>(&self) -> ActionClosure<C> {
        Box::new(|entry, _preview| {
            Ok(ActionResultKind::Preview {
                path: entry.path(),
                action: self.to_owned(),
            })
        })
    }

    fn action_move_to_trash<C: ClientState>(&self) -> ActionClosure<C> {
        Box::new(|entry, preview| {
            if preview {
                Ok(ActionResultKind::Preview {
                    path: entry.path(),
                    action: self.to_owned(),
                })
            } else {
                move_to_trash(entry.path())
                    .map_err(std::convert::Into::into)
                    .map(|_| ActionResultKind::Successful)
            }
        })
    }

    fn action_delete<C: ClientState>(&self) -> ActionClosure<C> {
        Box::new(|entry, preview| {
            if preview {
                Ok(ActionResultKind::Preview {
                    path: entry.path(),
                    action: self.to_owned(),
                })
            } else {
                remove_irrecoverably(entry.path())
                    .map_err(std::convert::Into::into)
                    .map(|_| ActionResultKind::Successful)
            }
        })
    }

    fn action_symlink<'a, C: ClientState>(&'a self, dst: &'a str) -> ActionClosure<'a, C> {
        Box::new(move |entry, preview| {
            if preview {
                Ok(ActionResultKind::Preview {
                    path: entry.path(),
                    action: self.to_owned(),
                })
            } else {
                symlink_to(entry.path(), dst)
                    .map_err(std::convert::Into::into)
                    .map(|_| ActionResultKind::Successful)
            }
        })
    }
}
