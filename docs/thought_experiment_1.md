Some time ago I wrote that a wonderful save_as_procedure would
communicate with interface via blocking RPC-like manner.

something like this:

```
let result = save_as_dialog.execute();

match result { 
    Cancelled => ...
    Selected(path) => ...
}
 ```
(like in Delphi)

the reason for it is that I'd prefer to keep a state of multi step procedure inside this procedure, 
and not stored somewhere "on the object". That is:
 
Object's state before and after procedure is the same ("ready", "neutral")

This particular (example) procedure would block on execute, waiting for 
feedback from interface (result of dialog).

SaveAsDialog would probably be instantiated by/in context of parent window.

Now let's assume this idea is compatible with my "two way Pilot" idea.

Then:

let's determine parent window to be text edit view.
We would have a Pilot to TV (text view, not television).

so we'd have something like:

```
let save_as_dialog : SaveAsDialogPilot = tv.show_save_as_dialog()
```

If save as dialog is instantiated, the parent window
is most probably blocked (like we're not accepting writes to buffer we're considering to save).

So it seems like:
1) pilots, if there are many, behave like writers in writers-readers classic concurrency problem, that is are mutually exclusive.
2) acquiring pilot to subwindow might imply that parent... hmm. Not sure.

That actually sounds a lot like ownership of lock. I lock a subtree (acquire mutable reference).
Runtime checked.