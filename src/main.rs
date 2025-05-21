use hyprland::data::Clients;
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch;
use hyprland::dispatch::{
    Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprData;
use std::cell::Cell;

fn main() -> hyprland::Result<()> {
    let mut listener = EventListener::new();

    let workspace_num = Cell::new(1);

    listener.add_workspace_changed_handler(move |state| {
        let id: i32 = state.id;
        let old_workspace: Vec<Workspace> = Workspaces::get()
            .expect("womp womp")
            .into_iter()
            .filter(|work| work.id == workspace_num.get())
            .collect::<Vec<Workspace>>();
        if old_workspace.is_empty() || old_workspace[0].windows == 0 {
            move_down_from(workspace_num.get(), id)
        }
        workspace_num.set(id);
    });

    listener.start_listener()?;

    Ok(())
}

fn move_down_from(from: i32, current_workspace: i32) {
    Clients::get()
        .unwrap()
        .into_iter()
        .filter(|client| client.workspace.id >= from)
        .for_each(|client| {
            if client.workspace.id == current_workspace {
                dispatch!(
                    MoveToWorkspace,
                    WorkspaceIdentifierWithSpecial::Id(client.workspace.id - 1),
                    Some(WindowIdentifier::Address(client.address))
                )
                .expect("dispatch failed");
            } else {
                dispatch!(
                    MoveToWorkspaceSilent,
                    WorkspaceIdentifierWithSpecial::Id(client.workspace.id - 1),
                    Some(WindowIdentifier::Address(client.address))
                )
                .expect("dispatch failed?");
            }
        });
}
