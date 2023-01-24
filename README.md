# Peanut

Peanut is an x86 emulator written for educational purposes and is currently in a
woefully incomplete state. As this is my way of learning x86, there are many
portions of the project which are not yet clear in my head and almost certainly
equally many misunderstandings/errors in the already completed components.

## Goals (Vaguely Prioritised)

- Core x86 architecture for parsing instructions (e.g. SIB, ModRM, memory).
  These should stick as closely as possible to the actual mechanisms by which
  they work in reality, even if this is not the most elegant, efficient, or
  optimal solution. The point here is to learn about how x86 operates, not make
  the emulator run fast.
- Common x86 instructions (and eventually full instruction set).
- More complex x86 features such as signals, interrupts, privilege levels,
  memory models etc.
- Assembler.
- Decompiler.
- Large scale testing, hopefully automated, to test our implementation vs
  running on actual hardware. For example, taking the same NASM code, running it
  step-by-step and inspecting that the resulting state is identical.
