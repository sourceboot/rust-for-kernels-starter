# rust-for-kernels-starter

The starter workspace for **Rust for Kernels: Build the Kernel Kit** — a
[SourceBoot](https://sourceboot.com) course in the Rust that exists only because
there is no operating system under the program. You lay a struct out byte for byte
so another program can hand it over, reach through raw pointers and volatile
registers on purpose, share state with no `static mut`, print with no heap, queue
without allocating, write one instruction by hand, and build the build for a target
with nothing under it. Every piece goes into one crate, `kit`, and from lab 04 on a
freestanding binary links it.

Nothing here starts a virtual machine: every lab runs on your own computer, as
ordinary tests and a cross-build.

This repo is exactly what a learner's workspace starts as. The lessons, the per-lab
tests and the grader are deliberately **not** in here — they live on
[sourceboot.com](https://sourceboot.com) and arrive through the `sboot` CLI, into a
separate cache directory. A repo created from this template stays your code and
nothing else, which is what makes it worth showing people.

## Use it

Two ways in; both give you the same tree.

**With GitHub** — your workspace starts life as a private repo you own:

```sh
gh repo create my-kit --private --template sourceboot/rust-for-kernels-starter --clone
cd my-kit
```

Then install `sboot` and work from inside the clone:

```sh
curl -fsSL https://sourceboot.com/install.sh | sh
sboot login                   # connects this machine, in your browser
sboot test 00-welcome         # fetches the lab's tests + grader, runs them, grades
```

`sboot` recognises the repo by its `sboot.toml` and downloads each lab's tests on
first use (`sboot where` prints where they live — outside this repo). Running
`sboot start rust-for-kernels` inside the clone is safe: it puts back anything
that is missing and never touches a file you have edited. With the template you
already have the tree, so you don't need it.

**Without GitHub:**

```sh
sboot start rust-for-kernels
```

materialises this same tree into `./sourceboot-rust-for-kernels/` — named after the
course — makes it a git repository and commits it,
asking once for the name and email git stamps on your commits. No `gh` and no
template involved; `--dir <name>` picks a different folder. When you want it on
GitHub, `sboot repo` creates the private repo and pushes it.

## What's in the tree

```
metal/                  the cargo workspace you own
  kit/                  the library you write, one module per lab — `no_std`
                        at the top of lib.rs, below its `//!` line
    src/                handoff, raw, sync, print, ring, arch and registry:
                        each file holds its lab's test names over `todo!()`
                        bodies, and lib.rs declares a module when its lab
                        tells you to
    tests/no_alloc.rs   lab 05's one integration test; a comment until then
  bare/                 lab 04's freestanding binary, built for
                        x86_64-unknown-none; an empty `main` until then
  xtask/                lab 08's build tool; a placeholder until then
  .cargo/config.toml    lab 08's to write; comments only until then
.cargo/                 the build flags every compile here uses, and the short
                        list of calls the library may not make (clippy.toml);
                        the same file the grader compiles under
rust-toolchain.toml     pinned stable Rust, the x86_64-unknown-none target and
                        clippy — that is the whole toolchain
sboot.toml              tells the sboot CLI which course this repo is for
```

Stable Rust, on your own machine, with **no external crates, ever**, and no heap in
the library — writing the lock, the console and the queue yourself is the course.
A fresh clone is ready in seconds:

```sh
cd metal && cargo test -p kit
```

prints `test result: ok. 0 passed` for each of its sections: the files in
`kit/src/` are not part of the crate until a lab adds their line to `lib.rs`.

## Tests that fail on purpose

When lab 01 has you add `pub mod handoff;`, the four test names in `handoff.rs`
join the build, and each one fails with `not yet implemented: lab 01 task N` until
the task that writes it. Every module after it works the same way. That is the
lab's to-do list, not a broken template.

## The course

https://sourceboot.com — lessons, labs and grading. This template is just the
starting tree.

## License

MIT — see [LICENSE](LICENSE). The scaffold is yours to build on and publish.
The course prose, tests and grader are not in this repo and are not covered by
it.
