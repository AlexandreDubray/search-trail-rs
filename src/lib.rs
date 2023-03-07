use paste::paste;
/// This structure keeps track of the length of a given level of the trail as well as the number of
/// managed resources of each kind. This second information is useful in order to truncate the
/// vector in the state manager.
#[derive(Debug, Clone, Copy, Default)]
struct Level {
    /// The length of the trail at the moment this level was started
    trail_size: usize,
    size_u8: usize,
    size_u16: usize,
    size_u32: usize,
    size_u64: usize,
    size_u128: usize,
}

/// An entry that is used to restore data from the trail
#[derive(Debug, Clone, Copy)]
enum TrailEntry {
    Intu8Entry(Stateu8),
    Intu16Entry(Stateu16),
    Intu32Entry(Stateu32),
    Intu64Entry(Stateu64),
    Intu128Entry(Stateu128),
}

#[derive(Debug, Clone)]
pub struct StateManager {
    /// This clock is responsible to tell if a data need to be stored on the trail for restitution
    /// or not. If a managed resource X is changed and X.clock < clock, then it needs to be saved
    /// on the trail for restitution. Once the managed resource is updated, X.clock = clock.
    ///
    /// This clock is incremented at each call to `save_state()`
    clock: usize,
    /// The values that are saved on the trail. These entries are used to restore the managed
    /// resources when `restore_state()` is called
    trail: Vec<TrailEntry>,
    /// Levels of the trail where a level is an indicator of the number of `TrailEntry` for a given
    /// timestamp of `clock`
    levels: Vec<Level>,
    integers_u8: Vec<Stateu8>,
    integers_u16: Vec<Stateu16>,
    integers_u32: Vec<Stateu32>,
    integers_u64: Vec<Stateu64>,
    integers_u128: Vec<Stateu128>,
}

impl Default for StateManager {
    fn default() -> Self {
        Self {
            clock: 0,
            trail: vec![],
            levels: vec![Level {
                trail_size: 0,
                size_u8: 0,
                size_u16: 0,
                size_u32: 0,
                size_u64: 0,
                size_u128: 0,
            }],
            integers_u8: vec![],
            integers_u16: vec![],
            integers_u32: vec![],
            integers_u64: vec![],
            integers_u128: vec![],
        }
    }
}

// --- Save and restore --- //

impl SaveAndRestore for StateManager {
    fn save_state(&mut self) {
        // Increment the clock of the state manager. After this, every managed resource will become
        // "invalid" and will need to be stored on the trail if changed
        self.clock += 1;
        self.levels.push(Level {
            trail_size: self.trail.len(),
            size_u8: self.integers_u8.len(),
            size_u16: self.integers_u16.len(),
            size_u32: self.integers_u32.len(),
            size_u64: self.integers_u64.len(),
            size_u128: self.integers_u128.len(),
        });
    }

    fn restore_state(&mut self) {
        debug_assert!(self.levels.len() > 1);
        let level = self
            .levels
            .pop()
            .expect("Can not pop the root level of the state manager");

        // Before the creation of the current level, the trail was `trail_size` long, so we skip
        // these first elements.
        for e in self.trail.iter().skip(level.trail_size).rev().copied() {
            match e {
                TrailEntry::Intu8Entry(state) => self.integers_u8[state.id.0] = state,
                TrailEntry::Intu16Entry(state) => self.integers_u16[state.id.0] = state,
                TrailEntry::Intu32Entry(state) => self.integers_u32[state.id.0] = state,
                TrailEntry::Intu64Entry(state) => self.integers_u64[state.id.0] = state,
                TrailEntry::Intu128Entry(state) => self.integers_u128[state.id.0] = state,
            }
        }
        self.trail.truncate(level.trail_size);
        self.integers_u8.truncate(level.size_u8);
        self.integers_u16.truncate(level.size_u16);
        self.integers_u32.truncate(level.size_u32);
        self.integers_u64.truncate(level.size_u64);
        self.integers_u128.truncate(level.size_u128);
    }
}

pub trait SaveAndRestore {
    /// Saves the current state of all managed resources
    fn save_state(&mut self);

    /// Restores the previous state of all managed resources
    fn restore_state(&mut self);
}

macro_rules! manage_integers {
    ($($u:ty: $tname:ident,)*) => {
        $(
            paste!{
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub struct [<Reversible $u>](usize);
                
                #[derive(Debug, Clone, Copy)]
                struct [<State $u>] {
                    id: [<Reversible $u>],
                    clock: usize,
                    value: $u,
                }

                pub trait $tname {
                    fn [<manage _ $u>](&mut self, value: $u) -> [<Reversible $u>];
                    fn [<get _ $u>](&self, id: [<Reversible $u>]) -> $u;
                    fn [<set _ $u>](&mut self, id: [<Reversible $u>], value: $u) -> $u;
                    fn [<increment _ $u>](&mut self, id: [<Reversible $u>]) -> $u;
                    fn [<decrement _ $u>](&mut self, id: [<Reversible $u>]) -> $u;
                }
                
                impl $tname for StateManager {
                    fn [<manage _ $u>](&mut self, value: $u) -> [<Reversible $u>] {
                        let id = [<Reversible $u>](self.[<integers _ $u>].len());
                        self.[<integers _ $u>].push([<State $u>]{
                            id,
                            clock: self.clock,
                            value,
                        });
                        id
                    }
                    fn [<get _ $u>](&self, id: [<Reversible $u>]) -> $u {
                        self.[<integers _ $u>][id.0].value
                    }
                    fn [<set _ $u>](&mut self, id: [<Reversible $u>], value: $u) -> $u {
                        let curr = self.[<integers _ $u>][id.0];
                        if value != curr.value {
                            if curr.clock < self.clock {
                                self.trail.push(TrailEntry::[<Int $u Entry>](curr));
                                self.[<integers _ $u>][id.0] = [<State $u>] {
                                    id,
                                    clock: self.clock,
                                    value,
                                };
                            } else {
                                self.[<integers _ $u>][id.0].value = value;
                            }
                        }
                        value
                    }

                    fn [<increment _ $u>](&mut self, id: [<Reversible $u>]) -> $u {
                        self.[<set _ $u>](id, self.[<get _ $u>](id) + 1)
                    }

                    fn [<decrement _ $u>](&mut self, id: [<Reversible $u>]) -> $u {
                        self.[<set _ $u>](id, self.[<get _ $u>](id) - 1)
                    }
                }
            }
        )*
    }
}

manage_integers! {
    u8: ManagerU8,
    u16: ManagerU16,
    u32: ManagerU32,
    u64: ManagerU64,
    u128: ManagerU128,
}