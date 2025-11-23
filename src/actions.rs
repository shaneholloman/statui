use crate::state::App;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    ToggleInspect,
    CycleSort,
    ToggleSortDirection,
    Search,
    Help,
}

/// Returns true if the action is 'Quit', returns false and
/// handles the action otherwise.
pub fn handle_action(action: &Action, app: &mut App) -> bool {
    use Action::*;

    match action {
        Quit => {
            return true;
        }
        MoveUp => {
            app.next_row();
        }
        MoveDown => {
            app.previous_row();
        }
        // TODO: Implement the following functions
        ToggleInspect => {
            app.toggle_inspect();
        }
        CycleSort => {
            return false;
        }
        ToggleSortDirection => {
            return false;
        }
        Search => {
            return false;
        }
        Help => {
            return false;
        }
    }
    false
}
