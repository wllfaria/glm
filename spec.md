# Specification
This is actually not a spec, but rather a document for describing the API
and the interactions between.


## What is the goal with this.
Provide an API to view and handle file system operations compatible with 
any frontend that I wanna make. Initially, I'll make it with with a TUI,
but eventually I want to be able to make a GUI version if I feel like it,
or making it easy enough for anyone who may want to do that.

One of the main goals is to have a totally decoupled API to handle file 
system operations, and, as of this time, two models that would use this 
API and expose two different `states` for the frontend.

## How I want it to work.
I want to provide for the frontend, immutable states that are produced
whenever a new file system operation is requested. I want the usage for 
the frontend to be something like:

```rs
let state = glm.change_dir("path_to_directory")
```

This would produce a new state, in which the frontend could use to display
the files. The initial idea is to either have a Tree, or list view. (I'll
first implement the list view). that is, the frontend would have a list of
files and directories to display, and a set of possible operations that are
available through the common API.

## How the API should work
The user should be able to use the same methods whether it is using the tree
or the list view, and every state update will produce a new state, instead of
mutating the current state, this is so the core API can stay self-contained
and the views, would just exhibit the state that they get and keep its own
state.

For instance, it should be up to the view to implement the state that would
handle file selection in such a way that it would be compatible with the 
commands that can be done in bulk.

### API Details

#### The list model
The list model would be the default model that the file manager would operate,
and as of right now it should be something like the following:

```rs
struct ListState {
    items: Vec<Item>,
    current_dir: PathBuf,
}
```

and as of right now I think that each Item would be something like:

```rs
struct Item {
    file_name: String,
    file_path: PathBuf,
    file_type: FileType // Directory | File | Symlink,
    file_ext: Option<String> // Optional as it could be a directory
    is_dirty: bool // this will be false at startup,
    is_selected: bool,
    is_hidden: false,
}
```

with every state change, a new state would be given to the view so it can 
reflect new changes, such as styling selected items or updating when directory
changes, or even displaying the hidden files when requested.

#### The core model
The core model is where all the file operations are performed, and where the 
new state is produced and dispatched. below is a list of operations available.

```rs
// Disclaimer:
// Every method that uses an `u64` as a parameter can possibly be replaced
// by the canonical path to the actual item, which can be compared.

// This will list all the items inside of a given path. This is mainly useful
// for path completion. This action doesn't produce a new state. instead, it
// returns the items inside a given path.
// fn list_dir(path: PathBuf) -> Vec<Item>;

// This will list a directory and return a new state with the contents added.
// In case the state is a `ListState`, this would be the same as changing the 
// root dir, but in case the state is a `TreeState` this would append the 
// dir to the correct node on the tree.
//
// When listing a directory from the `ListState`, this will also unselect every
// item.
fn change_dir(path: PathBuf) -> State;

// This would change the root directory of the state, which in the `ListState`
// would just produce a new state with a new list o items. But in the 
// `TreeList` it would either discard a whole subtree, or append a new node at
// the top of the tree. (this is more complex, hence why only the list tree 
// will be implemented first)
//
// Changing the root in a `ListState` will unselect every item.
fn change_root(new_root: PathBuf) -> State;

// This will change the state of all the items in the vector and set their
// selected state to the opposite of they currently are.
fn toggle_selection(items: Vec<u64>) -> State;

// This will attempt to delete an item, and update the state accordingly, or
// return a meaningful error to be displayed. This is particularly more complex
// to implement in the `TreeState`. Since it has to update the tree accondingly
fn delete_item(item: u64) -> State;

// This will attempt to delete every selected item on the list or tree, and 
// also return meaningful errors if any happen. This is more complex in the
// `TreeState` since you could possibly have two nodes from the same branch
// selected, and deleting the parent would also delete the children. I still
// have to look more into how I'll make this
fn delete_selected_items() -> State;

// This will attempt to rename an item to `new_name`, and return meaningful
// errors if it fails. renaming an item will remove the selection on given 
// item.
fn rename_item(item: u64, new_name: String) -> State;

// This will refresh the current state, re-fetching every item on the current
// dir. I still have to think if on the `TreeState` this will recurse up to the
// level it was before, or just refresh the root. (probably recurse)
//
// Refreshing will also remove selections.
fn refresh() -> State;

// This will attempt to move the item to the specified path, handling errors
// to give meaningful messages when needed.
//
// Moving items will remove selections.
fn move_item(item: u64, new_path: PathBuf) -> State;

// This will attempt to move all the selected items to the given path, and will
// fail if there are any conflicts. We will check for conflicts before moving,
// so we either move all the files, or don't move any if there are conflicts.
//
// There are also other edge cases in which we can't assert that every file 
// will in fact be moved. such as invalid permissions.
//
// Moving items will remove selections.
fn move_selected_items(item: u64, new_path: PathBuf) -> State;

// I'm still not sure how to implement this. So far i'm wondering about either
// returning a vec of matching paths, or a tree with only the matching pairs.
// depending on the state used. But this is not yet defined.
fn search(needle: String) -> ???;

// Probably missing more stuff, also this don't include any helper methods 
// that will be required. This is just the exposed api.
```
