### State-View-Controller

Idea is following:

Controller -(produces)-> State -(that is read-only for)-> View

####View
Simple. Displays State. If there is input to the view, it's packaged
into (most likely) messages to Controller.

- View however have no capacity to update state. 

Dropped:
- State can be updated outside of View, so View should not assume it's immutability.


####Controller

Controllers are used to do interaction. They produce state.

Controller of complex structure can contain controllers of substructures
and generate it's state by aggregating data from sub-states.

Considered:
controllers might need an update mechanism, like top down one. Not sure yet. The initial idea was an update method with contract that once called, the parent is already updated. Removed now.

####State

State needs to be Serializable and Send.

Here are some ideas I have:
- State should be a tree of (possibly shared) substates. No point in copying them every use, state of MOST components will remain same frame-to-frame. 
- Controller can DROP the previous state (and start from scrach or not) but 
the dropped state will continue to persist read-only until last of it's clients drops it.

This is experimental:
- Can be versioned.
- Serialize PRESERVES versions...
- ...unless you explicitly DROP.

#### Pilot

In order to avoid passing around mutable references to Controllers, I will default
to async messages for passing information from tui/gui implementation back to controller.

To avoid routing messages from root back to subcontrollers and allow moving responsibility down to subtrees, I will create DIRECT lines from views to controllers.

I will call these lines Pilots.

As for now, I make no assumptions on number of Pilots per Component or their lifetimes.

I also have not decided if Pilots are one way or two ways (like blocking RPC or "meetings" in CSP).

see thought experiment 1. 