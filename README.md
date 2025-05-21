# hyprdyn

Automatically organizes hyprland workspaces by shifting workspaces when moving away from an empty workspace.

The program is bare bones and cannot keep track of positions/tiles of windows because the hyprland IPC does not have this possibility.
Instead of actually changing the workspaces, all windows further than the empty workspace are moved to a workspace below causing hyprland to retile, losing the order of tiles.
