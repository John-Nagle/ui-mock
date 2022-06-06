# ui-mock - mockup of a game-type user interface

A mockup and test fixture for a game-type user interface.
Intended for use in a metaverse client.

It's a 2D menu interface on top of a 3D world. The 2D
interface is intended to be unobtrusive, so it disappears when
not in use. Moving the mouse pointer into or out of the window,
into the top bar, or pressing ESC, will make it reappear.

In full screen mode, moving the mouse point to the top or bottom of the screen
will make the menus reappear.

This mockup shows the user interface, but doesn't have any content behind it.

Uses Rend3 for 3D and Egui for the 2D menu overlay.

100% safe Rust.

## Features
* Windowed or full screen mode.
* Clean screen when possible.

## Platforms

* Linux (tested)
* Windows (tested under Wine 6)
* Mac (future)
