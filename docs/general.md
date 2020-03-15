### How I plan to structure code

I want to follow Model-View-Controller design pattern.
However I will call Model a State, as it sounds more
natural to me.

#### Cascading state

A component can be composed of subcomponents, therefore
a state can contain substates. I call this "cascading state".

However, I will nest controllers, delegating the responsibility to
construct / reconstruct State to implementations, allowing some polymorphism.  

There are some contracts I want to impose (WIP)

1) The hierarchy composes a tree.