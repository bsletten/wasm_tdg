# 01 : WASI Preview 2 as a Component

Chapter 11 built WASI **Preview 1** core modules. Look at `ch11/hello.wat`: it
imports `wasi_unstable`/`fd_write` and pokes an iovec into linear memory by
hand. That is the whole of Preview 1 — a flat list of POSIX-ish functions.

Preview 2 replaces that with the component model. Same Rust source you would
write for a native binary; what changes is the target and the artifact.

```
> make run                # no capabilities granted
> make run-granted        # --env and --dir
> make wit                # the component's actual contract
```

`make wit` is the point of the exercise:

```
world root {
  import wasi:cli/environment@0.2.6;
  import wasi:clocks/wall-clock@0.2.6;
  import wasi:filesystem/types@0.2.6;
  ...
  export wasi:cli/run@0.2.6;
}
```

Typed, namespaced, *versioned* interfaces instead of loose function imports.
The component declares what it needs and exports `wasi:cli/run` to say "I am a
command."

Note also what `make run` prints without any flags:

```
cannot read '.': failed to find a pre-opened file descriptor ...
```

That is the capability model from Chapter 11 unchanged — a WASI module gets
only what the host hands it. Preview 2 made the interfaces richer, not the
sandbox weaker.
