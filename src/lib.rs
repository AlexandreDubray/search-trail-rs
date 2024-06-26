//Copyright (c) 2023 X. Gillard, A. Dubray
//
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:
//
//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.
//
//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

use paste::paste;

macro_rules! manage_numbers {
    ($($u:ty),*) => {
        paste!{
            /// This structure keeps track of the length of a given level of the trail as well as the number of
            /// managed resources of each kind. This second information is useful in order to truncate the
            /// vector in the state manager.
            #[derive(Debug, Clone, Copy, Default)]
            struct Level {
                /// The length of the trail at the moment this level was started
                trail_size: usize,
            }

            /// An entry that is used to restore data from the trail
            #[derive(Debug, Clone, Copy)]
            enum TrailEntry {
                $(
                    [<$u:camel Entry>]([<State $u:camel>]),
                    [<Option $u:camel Entry>]([<StateOption $u:camel>]),
                )*
            }

            /// This structure implements a simple manager that can save a state and restore it later.
            /// It is able to store each numeric type as well as booleans.
            /// The states are stored and restored like a stack. This means that when restoring the state of the
            /// manager, all the managed values are restored to their **most recently** saved value.
            ///
            /// #Example
            ///
            /// ```
            /// use search_trail::{StateManager, SaveAndRestore, UsizeManager};
            ///
            /// fn main() {
            ///     let mut mgr = StateManager::default();
            ///     let n = mgr.manage_usize(0);
            ///     assert_eq!(0, mgr.get_usize(n));
            ///
            ///     mgr.save_state();
            ///
            ///     mgr.set_usize(n, 20);
            ///     assert_eq!(20, mgr.get_usize(n));
            ///
            ///     mgr.save_state();
            ///
            ///     mgr.set_usize(n, 42);
            ///     assert_eq!(42, mgr.get_usize(n));
            ///
            ///     mgr.restore_state();
            ///     assert_eq!(20, mgr.get_usize(n));
            ///
            ///     mgr.restore_state();
            ///     assert_eq!(0, mgr.get_usize(n));
            /// }
            /// ```
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
                $(
                    [<numbers _ $u>]: Vec<[<State $u:camel>]>,
                    [<numbers _ option _ $u>]: Vec<[<State Option $u:camel>]>,
                )*
            }

            impl Default for StateManager {
                fn default() -> Self {
                    Self {
                        clock: 0,
                        trail: vec![],
                        levels: vec![Level {
                            trail_size: 0,
                        }],
                        $(
                            [<numbers _ $u>]: vec![],
                            [<numbers_option_ $u>]: vec![],
                        )*
                    }
                }
            }

            pub trait SaveAndRestore {
                /// Saves the current state of all managed resources
                fn save_state(&mut self);

                /// Restores the previous state of all managed resources
                fn restore_state(&mut self);
            }

            impl SaveAndRestore for StateManager {
                fn save_state(&mut self) {
                    // Increment the clock of the state manager. After this, every managed resource will become
                    // "invalid" and will need to be stored on the trail if changed
                    self.clock += 1;
                    self.levels.push(Level {
                        trail_size: self.trail.len(),
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
                                $(
                                    TrailEntry::[<$u:camel Entry>](state) => self.[<numbers _ $u>][state.id.0] = state,
                                    TrailEntry::[<Option $u:camel Entry>](state) => self.[<numbers_option_ $u>][state.id.0] = state,
                                )*
                            }
                        }
                        self.trail.truncate(level.trail_size);
                    }
            }

        $(
            // Can not use format!() in this doc
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[doc="An index of the managed resource type"]
            pub struct [<Reversible $u:camel>](usize);

            #[doc="A state for the managed resource type"]
            #[derive(Debug, Clone, Copy)]
            struct [<State $u:camel>] {
                #[doc="Index of the resource in the asociated vector in the trail"]
                id: [<Reversible $u:camel>],
                #[doc="Clock of the resource. If less than the clock of the manager, the data needs to be saved on the trail if modified"]
                clock: usize,
                #[doc="The value of the managed resource"]
                value: $u,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[doc="An index of the managed resource type"]
            pub struct [<Reversible Option $u:camel>](usize);

            #[doc="A state for the managed resource type"]
            #[derive(Debug, Clone, Copy)]
            struct [<StateOption $u:camel>] {
                #[doc="Index of the resource in the asociated vector in the trail"]
                id: [<ReversibleOption $u:camel>],
                #[doc="Clock of the resource. If less than the clock of the manager, the data needs to be saved on the trail if modified"]
                clock: usize,
                #[doc="The value of the managed resource"]
                value: Option<$u>,
            }

            #[doc="Trait that define what operation can be done on the managed resource type"]
            pub trait [<$u:camel Manager>] {
                #[doc="Creates a new managed ressource.Returns the index of the resource in the corresponding vector"]
                fn [<manage _ $u>](&mut self, value: $u) -> [<Reversible $u:camel>];
                #[doc="Returns the value of the resource at the given index"]
                fn [<get _ $u>](&self, id: [<Reversible $u:camel>]) -> $u;
                #[doc="Sets the resource at the given index to the given value and returns the new value"]
                fn [<set _ $u>](&mut self, id: [<Reversible $u:camel>], value: $u) -> $u;
                #[doc="Increments the value of the resource at the given index and returns the new value"]
                fn [<increment _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u;
                #[doc="Decrements the value of the resource at the given index and returns the new value"]
                fn [<decrement _ $u>](&mut self, id: [<Reversible $u:camel>]) -> $u;
            }

            #[doc="Trait that define what operation can be done on the managed resource type"]
            pub trait [<Option $u:camel Manager>] {
                #[doc="Creates a new managed {}.Returns the index of the resource in the corresponding vector"]
                fn [<manage _ option _ $u>](&mut self, value: Option<$u>) -> [<Reversible Option $u:camel>];
                #[doc="Returns the value of the resource at the given index"]
                fn [<get _ option _ $u>](&self, id: [<Reversible Option $u:camel>]) -> Option<$u>;
                #[doc="Returns true if the managed ressource is not None"]
                fn [<is_option_ $u _some>](&self, id: [<Reversible Option $u:camel>]) -> bool {
                    self.[<get_option_ $u>](id).is_some()
                }
                #[doc="Returns true if the managed ressource is None"]
                fn [<is_option_ $u _none>](&self, id: [<Reversible Option $u:camel>]) -> bool {
                    self.[<get_option_ $u>](id).is_none()
                }
                #[doc="Sets the resource at the given index to the given value and returns the new value"]
                fn [<set _ option _ $u>](&mut self, id: [<Reversible Option $u:camel>], value: Option<$u>) -> Option<$u>;
                #[doc="Increments the value of the resource at the given index and returns the new value. Panic if the option is none."]
                fn [<increment _ option _ $u>](&mut self, id: [<Reversible Option $u:camel>]) -> $u;
                #[doc="Decrements the value of the resource at the given index and returns the new value. Panic if the option is none."]
                fn [<decrement _ option _ $u>](&mut self, id: [<Reversible Option $u:camel>]) -> $u;
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

            impl [<Option $u:camel Manager>] for StateManager {
                fn [<manage_option_ $u>](&mut self, value: Option<$u>) -> [<ReversibleOption $u:camel>] {
                    let id = [<ReversibleOption $u:camel>](self.[<numbers_option_ $u>].len());
                    self.[<numbers_option_ $u>].push([<StateOption $u:camel>]{
                        id,
                        clock: self.clock,
                        value,
                    });
                    id
                }

                fn [<get_option_ $u>](&self, id: [<ReversibleOption $u:camel>]) -> Option<$u> {
                    self.[<numbers_option_ $u>][id.0].value
                }

                fn [<set_option_ $u>](&mut self, id: [<ReversibleOption $u:camel>], value: Option<$u>) -> Option<$u> {
                    let curr = self.[<numbers_option_ $u>][id.0];
                    if value != curr.value {
                        if curr.clock < self.clock {
                            self.trail.push(TrailEntry::[<Option $u:camel Entry>](curr));
                            self.[<numbers_option_ $u>][id.0] = [<StateOption $u:camel>] {
                                id,
                                clock: self.clock,
                                value,
                            };
                        } else {
                            self.[<numbers_option_ $u>][id.0].value = value;
                        }
                    }
                    value
                }

                fn [<increment_option_ $u>](&mut self, id: [<ReversibleOption $u:camel>]) -> $u {
                    let value = self.[<numbers_option_ $u>][id.0].value.unwrap();
                    self.[<set_option_ $u>](id, Some(value + 1 as $u));
                    value + 1 as $u
                }

                fn [<decrement_option_ $u>](&mut self, id: [<ReversibleOption $u:camel>]) -> $u {
                    let value = self.[<numbers_option_ $u>][id.0].value.unwrap();
                    self.[<set_option_ $u>](id, Some(value -1 as $u));
                    value - 1 as $u
                }
            }

            #[cfg(test)]
            mod [<test _ $u>] {

                use crate::{StateManager, SaveAndRestore,[<$u:camel Manager>], [<Reversible $u:camel>]};

                #[test]
                fn manager_return_values() {
                    let mut mgr = StateManager::default();
                    let values: Vec<[<Reversible $u:camel>]> = (0..10).map(|i| mgr.[<manage _ $u>](i as $u)).collect();
                    for i in 0..10 {
                        assert_eq!([<Reversible $u:camel>](i), values[i]);
                        let x = mgr.[<set _ $u>](values[i], i as $u + 1 as $u);
                        assert_eq!(i as $u + 1 as $u, x);
                        assert_eq!(x + 1 as $u, mgr.[<increment _ $u>](values[i]));
                        assert_eq!(x, mgr.[<decrement _ $u>](values[i]));
                    }
                }

                #[test]
                fn set_and_restore() {
                    let mut mgr = StateManager::default();
                    let n = mgr.[<manage _ $u>](10 as $u);
                    assert_eq!(10 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    let x = mgr.[<set _ $u>](n, 20 as $u);
                    assert_eq!(20 as $u, x);
                    assert_eq!(20 as $u, mgr.[<get _ $u>](n));

                    let x = mgr.[<set _ $u>](n, 23 as $u);
                    assert_eq!(23 as $u, x);
                    assert_eq!(23 as $u, mgr.[<get _ $u>](n));

                    mgr.restore_state();

                    assert_eq!(10 as $u, mgr.[<get _ $u>](n));

                    let x = mgr.[<set _ $u>](n, 42 as $u);
                    assert_eq!(42 as $u, x);
                    assert_eq!(42 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    let x = mgr.[<set _ $u>](n, 12 as $u);
                    assert_eq!(12 as $u, x);
                    assert_eq!(12 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    let x = mgr.[<set _ $u>](n, 12 as $u);
                    assert_eq!(12 as $u, x);
                    assert_eq!(12 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    mgr.restore_state();
                    assert_eq!(12 as $u, mgr.[<get _ $u>](n));

                    mgr.restore_state();
                    assert_eq!(12 as $u, mgr.[<get _ $u>](n));

                    mgr.restore_state();
                    assert_eq!(42 as $u, mgr.[<get _ $u>](n));
                }

                #[test]
                fn test_increment() {
                    let mut mgr = StateManager::default();
                    let n = mgr.[<manage _ $u>](30 as $u);
                    assert_eq!(30 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    for i in 0..10 {
                        let x = mgr.[<increment _ $u>](n);
                        assert_eq!((30 + i + 1) as $u, x);
                        assert_eq!((30 + i + 1) as $u, mgr.[<get _ $u>](n));
                    }

                    mgr.restore_state();
                    assert_eq!(30 as $u, mgr.[<get _ $u>](n));

                    mgr.save_state();

                    for i in 0..10 {
                        let x = mgr.[<decrement _ $u>](n);
                        assert_eq!((30 -i -1) as $u, x);
                        assert_eq!((30 -i -1) as $u, mgr.[<get _ $u>](n));
                    }

                    mgr.restore_state();
                    assert_eq!(30 as $u, mgr.[<get _ $u>](n));
                }
            }
        )*
    }
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
    f64
}

/// Index for a managed bool. Note that this only redirect towards a managed usize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReversibleBool(ReversibleUsize);

/// Index for a managed optional bool. Note that this only redirect towards a managed usize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReversibleOptionBool(ReversibleOptionUsize);

/// Trait that define the operation that can be done on a managed boolean.
pub trait BoolManager {
    /// Creates a new managed boolean
    fn manage_bool(&mut self, value: bool) -> ReversibleBool;
    /// Returns the value of a managed boolean
    fn get_bool(&self, id: ReversibleBool) -> bool;
    /// Sets the value of a managed boolean to the given value and returns the new value
    fn set_bool(&mut self, id: ReversibleBool, value: bool) -> bool;
    /// Flips the value of a managed boolean and returns the new value
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

/// Trait that define the operation that can be done on a managed boolean.
pub trait OptionBoolManager {
    /// Creates a new managed boolean
    fn manage_option_bool(&mut self, value: Option<bool>) -> ReversibleOptionBool;
    /// Returns the value of a managed boolean
    fn get_option_bool(&self, id: ReversibleOptionBool) -> Option<bool>;
    /// Sets the value of a managed boolean to the given value and returns the new value
    fn set_option_bool(&mut self, id: ReversibleOptionBool, value: bool) -> bool;
    /// Sets the value of a managed boolean to None
    fn set_option_bool_none(&mut self, id: ReversibleOptionBool);
    /// Flips the value of a managed boolean and returns the new value. Panic if option is none
    fn flip_option_bool(&mut self, id: ReversibleOptionBool) -> bool {
        let value = self.get_option_bool(id).unwrap();
        self.set_option_bool(id, value);
        !value
    }
    /// Return true iff the option is some
    fn is_option_bool_some(&self, id: ReversibleOptionBool) -> bool {
        self.get_option_bool(id).is_some()
    }
    /// Return true iff the option is some
    fn is_option_bool_none(&self, id: ReversibleOptionBool) -> bool {
        self.get_option_bool(id).is_none()
    }
}

impl OptionBoolManager for StateManager {
    fn manage_option_bool(&mut self, value: Option<bool>) -> ReversibleOptionBool {
        if let Some(b) = value {
            ReversibleOptionBool(self.manage_option_usize(Some(b as usize)))
        } else {
            ReversibleOptionBool(self.manage_option_usize(None))
        }
    }

    fn get_option_bool(&self, id: ReversibleOptionBool) -> Option<bool> {
        if let Some(v) = self.get_option_usize(id.0) {
            Some(v != 0)
        } else {
            None
        }
    }

    fn set_option_bool(&mut self, id: ReversibleOptionBool, value: bool) -> bool {
        self.set_option_usize(id.0, Some(value as usize));
        value
    }

    fn set_option_bool_none(&mut self, id: ReversibleOptionBool) {
        self.set_option_usize(id.0, None);
    }
}

#[cfg(test)]
mod test_manager {
    use crate::{BoolManager, SaveAndRestore, StateManager};

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn can_not_pop_root_level() {
        let mut mgr = StateManager::default();
        let a = mgr.manage_bool(true);

        mgr.save_state();
        mgr.set_bool(a, false);
        mgr.restore_state();
        mgr.restore_state();
    }
}

#[cfg(test)]
mod test_manager_bool {

    use crate::{BoolManager, SaveAndRestore, StateManager};

    #[test]
    fn works() {
        let mut mgr = StateManager::default();
        let a = mgr.manage_bool(false);
        assert!(!mgr.get_bool(a));

        mgr.save_state();

        let x = mgr.set_bool(a, true);
        assert!(x);
        assert!(mgr.get_bool(a));

        mgr.restore_state();
        assert!(!mgr.get_bool(a));

        let x = mgr.flip_bool(a);
        assert!(x);
        mgr.save_state();

        let x = mgr.set_bool(a, false);
        assert!(!x);
        let x = mgr.set_bool(a, true);
        assert!(x);
        assert!(mgr.get_bool(a));
        mgr.restore_state();
        assert!(mgr.get_bool(a));
    }
}
