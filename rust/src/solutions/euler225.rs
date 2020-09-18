use std::collections::HashSet;

fn test(&n: &u32) -> bool {
    let mut set = HashSet::new();
    let mut state = (1, 1, 1);
    loop {
        if state.2 == 0 {
            return false;
        } else if set.contains(&state) {
            return true;
        } else {
            set.insert(state);
            state = (state.1, state.2, (state.0 + state.1 + state.2) % n);
        }
    }
}

pub fn euler225() -> String {
    (27..).step_by(2).filter(test).nth(123).unwrap().to_string()
}
