# search_trail

search_trail provides a data structure to manage values of primitive types.
It means that you're able to
- create a new managed resource (of type `usize`, `isize`, `f32`, etc.)
- Save the state of the manager (i.e. a "snapshot" of all the values of the managed resources)
- Restore the manager to its previous state (i.e. set the values of the managed resources to the previous snapshot)

The code was originally developped by [@xgillard](https://github.com/xgillard/) on [maxi-cp-rs](https://github.com/xgillard/maxicp-rs).
This repo only adds the support for other type.


# Example

```rust
use search_trail::{StateManager, SaveAndRestore, UsizeManager};

fn main() {
 let mut mgr = StateManager::default();
 let n = mgr.manage_usize(0);
 assert_eq!(0, mgr.get_usize(n));
 
 mgr.save_state();
 
 mgr.set_usize(n, 20);
 assert_eq!(20, mgr.get_usize(n));
 
 mgr.save_state();

 mgr.set_usize(n, 42);
 assert_eq!(42, mgr.get_usize(n));
 
 mgr.restore_state();
 assert_eq!(20, mgr.get_usize(n));
 
 mgr.restore_state();
 assert_eq!(0, mgr.get_usize(n));
}
```
