### State-View-Controller

Idea is following:

Controller -(produces)-> State -(that is read-only for)-> View -(sends messages to Controller via)-> Pilot

####View
Simple. Displays State. If there is input to the view, it's packaged
into (most likely) messages to Controller.

- View however have no capacity to update state.
- View does not hold a reference to controller.
- View has a "pilot", which is a packaged message pipe to-and-back to controller. 

####Controller

Controllers produce State. Nevertheless, they are effectively nested Finite State Automata.
To distinguish, while writing about Controller state I will use lowercase letters.

Controller of complex structure can contain controllers of substructures
and generate it's state by aggregating data from sub-states.

The idea is following: Each controller have Stable and Intermediate states.
- Intermediate state is when Controller is transitioning from one Stable state to another one,
but the target one is not necessarily determined yet (it might be querying a substate for determining info).
- While in intermediate state, Controller is not accepting input from Piltos
- Controllers move from Stable to intermediate state stimulated either by parent controller or
in response to Pilot message. 

Update mechanism (current idea):
A controller consumes messages received from Pilot in a "tick".
While "ticking", Controller updates it's children controllers passing itself as mutable reference
(therefore blocking for outside edits).

If controllers use Pilots, it's only to communicate in deferred-update mode, most likely with non-ancestors. 

Considered:
- maybe Controllers need to be Sync. I don't think so yet.

####State

State needs to be Serializable and Send.

State is IMMUTABLE and therefore can be shared around via Arc and share substates between versions / ticks.

This is experimental:
- Can be versioned.
- Serialize PRESERVES versions...
- ...unless you explicitly DROP.
- Drop would produce new, simplified state, since the original is immutable.

#### Pilot

Pilot represents a direct line to controller, via async messages. You can think of it as RPC interface.
Most likely Pilots will offer both blocking and non blocking calls.

One controller can have multiple Pilots. Pilots are numbered. 

They are designed primarily to be passed around to Views as "lightweight in terms of borrowchecking"
references to controllers.

