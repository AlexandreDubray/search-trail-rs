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
    size_usize: usize,
    size_i8: usize,
    size_i16: usize,
    size_i32: usize,
    size_i64: usize,
    size_i128: usize,
    size_isize: usize,
    size_f32: usize,
    size_f64: usize,
}

/// An entry that is used to restore data from the trail
#[derive(Debug, Clone, Copy)]
enum TrailEntry {
    U8Entry(StateU8),
    U16Entry(StateU16),
    U32Entry(StateU32),
    U64Entry(StateU64),
    U128Entry(StateU128),
    UsizeEntry(StateUsize),
    I8Entry(StateI8),
    I16Entry(StateI16),
    I32Entry(StateI32),
    I64Entry(StateI64),
    I128Entry(StateI128),
    IsizeEntry(StateIsize),
    F32Entry(StateF32),
    F64Entry(StateF64),
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
    numbers_u8: Vec<StateU8>,
    numbers_u16: Vec<StateU16>,
    numbers_u32: Vec<StateU32>,
    numbers_u64: Vec<StateU64>,
    numbers_u128: Vec<StateU128>,
    numbers_usize: Vec<StateUsize>,
    numbers_i8: Vec<StateI8>,
    numbers_i16: Vec<StateI16>,
    numbers_i32: Vec<StateI32>,
    numbers_i64: Vec<StateI64>,
    numbers_i128: Vec<StateI128>,
    numbers_isize: Vec<StateIsize>,
    numbers_f32: Vec<StateF32>,
    numbers_f64: Vec<StateF64>,
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
                size_usize: 0,
                size_i8: 0,
                size_i16: 0,
                size_i32: 0,
                size_i64: 0,
                size_i128: 0,
                size_isize: 0,
                size_f32: 0,
                size_f64: 0,
            }],
            numbers_u8: vec![],
            numbers_u16: vec![],
            numbers_u32: vec![],
            numbers_u64: vec![],
            numbers_u128: vec![],
            numbers_usize: vec![],
            numbers_i8: vec![],
            numbers_i16: vec![],
            numbers_i32: vec![],
            numbers_i64: vec![],
            numbers_i128: vec![],
            numbers_isize: vec![],
            numbers_f32: vec![],
            numbers_f64: vec![],
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
            size_u8: self.numbers_u8.len(),
            size_u16: self.numbers_u16.len(),
            size_u32: self.numbers_u32.len(),
            size_u64: self.numbers_u64.len(),
            size_u128: self.numbers_u128.len(),
            size_usize: self.numbers_usize.len(),
            size_i8: self.numbers_i8.len(),
            size_i16: self.numbers_i16.len(),
            size_i32: self.numbers_i32.len(),
            size_i64: self.numbers_i64.len(),
            size_i128: self.numbers_i128.len(),
            size_isize: self.numbers_isize.len(),
            size_f32: self.numbers_f32.len(),
            size_f64: self.numbers_f64.len(),
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
                TrailEntry::U8Entry(state) => self.numbers_u8[state.id.0] = state,
                TrailEntry::U16Entry(state) => self.numbers_u16[state.id.0] = state,
                TrailEntry::U32Entry(state) => self.numbers_u32[state.id.0] = state,
                TrailEntry::U64Entry(state) => self.numbers_u64[state.id.0] = state,
                TrailEntry::U128Entry(state) => self.numbers_u128[state.id.0] = state,
                TrailEntry::UsizeEntry(state) => self.numbers_usize[state.id.0] = state,
                TrailEntry::I8Entry(state) => self.numbers_i8[state.id.0] = state,
                TrailEntry::I16Entry(state) => self.numbers_i16[state.id.0] = state,
                TrailEntry::I32Entry(state) => self.numbers_i32[state.id.0] = state,
                TrailEntry::I64Entry(state) => self.numbers_i64[state.id.0] = state,
                TrailEntry::I128Entry(state) => self.numbers_i128[state.id.0] = state,
                TrailEntry::IsizeEntry(state) => self.numbers_isize[state.id.0] = state,
                TrailEntry::F32Entry(state) => self.numbers_f32[state.id.0] = state,
                TrailEntry::F64Entry(state) => self.numbers_f64[state.id.0] = state,
            }
        }
        self.trail.truncate(level.trail_size);
        self.numbers_u8.truncate(level.size_u8);
        self.numbers_u16.truncate(level.size_u16);
        self.numbers_u32.truncate(level.size_u32);
        self.numbers_u64.truncate(level.size_u64);
        self.numbers_u128.truncate(level.size_u128);
        self.numbers_usize.truncate(level.size_usize);
        self.numbers_i8.truncate(level.size_i8);
        self.numbers_i16.truncate(level.size_i16);
        self.numbers_i32.truncate(level.size_i32);
        self.numbers_i64.truncate(level.size_i64);
        self.numbers_i128.truncate(level.size_i128);
        self.numbers_isize.truncate(level.size_isize);
        self.numbers_f32.truncate(level.size_f32);
        self.numbers_f64.truncate(level.size_f64);
    }
}

pub trait SaveAndRestore {
    /// Saves the current state of all managed resources
    fn save_state(&mut self);

    /// Restores the previous state of all managed resources
    fn restore_state(&mut self);
}

macro_rules! manage_numbers {
    ($($u:ty,)*) => {
        $(
            paste!{
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub struct [<Reversible $u:camel>](usize);
                
                #[derive(Debug, Clone, Copy)]
                struct [<State $u:camel>] {
                    id: [<Reversible $u:camel>],
                    clock: usize,
                    value: $u,
                }

                pub trait [<$u:camel Manager>] {
                    fn [<manage _ $u>](&mut self, value: $u) -> [<Reversible $u:camel>];
                    fn [<get _ $u>](&self, id: [<Reversible $u:camel>]) -> $u;
                    fn [<set _ $u>](&mut self, id: [<Reversible $u:camel>], value: $u) -> $u;
                    fn [<increment _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u;
                    fn [<decrement _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u;
                }
                
                impl [<$u:camel Manager>] for StateManager {
                    fn [<manage _ $u>](&mut self, value: $u) -> [<Reversible $u:camel>] {
                        let id = [<Reversible $u:camel>](self.[<numbers _ $u>].len());
                        self.[<numbers _ $u>].push([<State $u:camel>]{
                            id,
                            clock: self.clock,
                            value,
                        });
                        id
                    }
                    fn [<get _ $u>](&self, id: [<Reversible $u:camel>]) -> $u {
                        self.[<numbers _ $u>][id.0].value
                    }
                    fn [<set _ $u>](&mut self, id: [<Reversible $u:camel>], value: $u) -> $u {
                        let curr = self.[<numbers _ $u>][id.0];
                        if value != curr.value {
                            if curr.clock < self.clock {
                                self.trail.push(TrailEntry::[<$u:camel Entry>](curr));
                                self.[<numbers _ $u>][id.0] = [<State $u:camel>] {
                                    id,
                                    clock: self.clock,
                                    value,
                                };
                            } else {
                                self.[<numbers _ $u>][id.0].value = value;
                            }
                        }
                        value
                    }

                    fn [<increment _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u {
                        self.[<set _ $u>](id, self.[<get _ $u>](id) + 1 as $u)
                    }

                    fn [<decrement _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u {
                        self.[<set _ $u>](id, self.[<get _ $u>](id) - 1 as $u)
                    }
                }
                
                #[cfg(test)]
                mod [<test _ $u>] {

                }
            }
        )*
    }
}

manage_numbers! {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReversibleBool(ReversibleUsize);

pub trait BoolManager {
    fn manage_bool(&mut self, value: bool) -> ReversibleBool;
    fn get_bool(&self, id: ReversibleBool) -> bool;
    fn set_bool(&mut self, id: ReversibleBool, value: bool) -> bool;
    fn flip_bool(&mut self, id: ReversibleBool) -> bool {
        self.set_bool(id, !self.get_bool(id))
    }
}

impl BoolManager for StateManager {

    fn manage_bool(&mut self, value: bool) -> ReversibleBool {
        ReversibleBool(self.manage_usize(value as usize))
    }

    fn get_bool(&self, id: ReversibleBool) -> bool {
        self.get_usize(id.0) != 0   
    }

    fn set_bool(&mut self, id: ReversibleBool, value: bool) -> bool {
        self.set_usize(id.0, value as usize) != 0
    }
}