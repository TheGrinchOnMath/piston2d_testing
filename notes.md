current state: massive mess.

todo: 
- convert the intersections to use Vector2d for all positions and vectors.
- setup piston2d boilerplate
- port over physics and I/O functions from main branch code.
- get MVP render box working
- setup again: keyboard input
- event loop
- basic render functions

future features:
- concurrent processing
- curved mirrors (bezier curve, ellipse, circle)

# Plans
The bulk of the work can be split into the following parts:
- getting boilerplate event loop done
- getting boilerplate render done

- function to render line, given ends, thickness, color
    *note: this function may want to take advantage of any possible optimizations when making many draw calls of the same type*

- mathematical calculation backend, based on previous work
- IO functions, refactor existing code

# Status:
## Mirror.rs:
partway through a refactor: move to Vector2D for all points and Vectors

then, smooth out all errors / warnings

## Main.rs:
smooth out all errors

then,
setup basic test: render single line
