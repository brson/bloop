# The Bloop language

Minimal abstraction, massively parallel distributed compile, fast compile, fast
execution; jit and repl, linear and affine types. Memory safe. Rust compatible.
C compatible.

In usus a custom parallel parser build with LALRPOP, all pases are parallel,
incremental, typechecking, resolution, and parsing, on demand, and considers the
full compilation dag. Code lowering is delaid until after the entire dag is
typechecked. Code generation is with Cranelift.

There is minimal library integration, and the library stack is completely
separate and well layered. The language knows nothing about allocators or
standard libraries.

There is no GC, local heaps for non-Send kinds. Local ECS registries maybe.

Bloop design goals:

* fast development, fast code, <1s compilation times
- Bare metal, no C dependencies in std
- Rust but with alternate hindsight priorities

* use best-in-class language tooling in Rust
  - lalrpop
  - cranelift
  - datafrog / chalk / differential-dataflow
  - specs

* ahead-of-time compile or jit compile
* explicit unsafe, mostly unsafe (for now)
* tunable tradeoffs for generic translation and dispatch, error
  representation, existential representation
  - preference toward dynamic dispatch, less mental overhead,
    less code overhead
* opinionated error handling
* limited region checking
* parallel parsing
* parallel pipeline
- distributed pipeline
- ECS compiler design (specs)?
- differential dataflow compiler design?

- parser via formal grammar

* unified declaration for structs and enums
  - modules have speceial syntex? for when they are in another file
* unified funcion call syntax, unified traits and impls

* owned value types
* algebraic data types and matching
* types with implicit or explicit destruction (affine vs linear types)
- no interior mutability!
  - what about atomics, or volatile?
* modules and libraries with precise visibility control
* commonmarkgfm documentation

* toml attributes
* unified cfg attributes and crate attributes
- maybe cfg syntax is toml

* static linking, no dynamic linking yuck it suck
* lazy, parallel compilation model
- _distributed_ compilation model
* parallel and distributed _doc_ compilation
* whole-program analysis
* existentials with known (union) size
* integrated package manager
* two-way package-manager integration
* global optional unwinding

* custom allocators
* undecided about GC and safety generally
* pinned types
* send and non-send types
* type anotition

* let bindings ref by default
* let bindings are mutable
* function bindings are move by default
* typechecking in chalk, from datalog syntax

* UFCS - all impls are traits (?)
  - imports of functions and types from types and traits
  - types, modules and traits may all be similar
  - modules are types, functions are static methods on modules
    - modules can contain labeled fields - they can be 'struct-types'
    - modules without fields can't be instantiated
	- each import is instantiated, but immediately deduplicated
    - that means imports can be generic-instantiated
	- two namespaces - type and value, no module namespace

- relative paths only - no absolute paths - just import again
  - this may be in tension with the disire for mods to be explicit dogs for refactoring
- module types
  - import - dag-forming, package -resolved source code
  - mod - default module that can't read from its parents, dag-forming,
    like import but for internal modules
	- mods are compiled exactly like imports
    - should there be a difference between `import` and `mod`? yeah, for explicitness and intent
  - depmod - mod that can import from parents, public and private, parent
    - imports must be explicit and relative there are no 'super::' paths?

- reduce import stuttering
  - anytime the path to a type contains a stutter, "Option::Option", the two
    types (because modules are types) can optionally be merged, if conflict free.
  - if mods take fields and accept associated functions then the module
    (type) can be used to define struct-types that would otherwise stutter in Rust.

- fully cached at a fine level, locally and/or remotely
- easy to set up build clusters, with cache storage clusters
- global build cache
  - trust ideas
    - we can have an official build cluster with trusted keys and security audits
	  if people do their builds there they will be counted 

* lexer lexes into syntax tree
- tree is parsed in parallel, with block indexing
  - just for fun

- producural macros

- coercions
  - no implicit copies
  - no explicit coercions
  - dedicated lossless coercion op
  - deadicated clone op
  - implicit copy types?
  - defense against large copies, moves

- borrowing
  - local bendings are ref by default
  - argument bindings are move by default
  - explicit moves?

- error handling
  - dedicated error handling op `?`
  - use something like failure

- concurrency
  - channels 

- full instrumentation and aop
  - io mocking
- integrated repl and debugger
  - lazy debuginfo via the distributed cache

- abort/unwind

- libs
  - clean std facade
  - int types can coerce upward
  - either type

- arch
  - bloop is rustc + cargo + cargo --test + goma + swappable code generators
  - integrated package manager, communicates to compiler-service via rpc
  - incremental builds and reporting, language service built-in, communicates via rpc
  - integrated testing, benchmarking and doc testing, via rpc
  - code lowering via rpc, first cranelift, then llvm
  - all rpc components technically swappable

- intrinsics are compiler provided functions for the core library to declare
- runtime-calls are library provided functions for the core library to call

- todo
  - doc comments
  - incremental compilation
  - consider ecs _runtime_ model
    - maybe an ecs datatype, kind of like jai's auto-soa
	- no global allocators - allocation is done via ecs
  - synta choices: <> ambiguity
  - :: as namespace separator
  - closures
  - exhaustive and non-exhaustive enums
  - allocator - local heaps. auto-Send analysis

- wishlist
  - async i/o, futures, async / await
  - detection of changes to apis per semver
  - mass scale testing via the build server network
  - profile-based optimization
  - traceable roots and pointers (even unsafe pointers)
    - for split stacks or gc

## Tour

A Bloop program:

```bloop
im std;

ty World;

fn main() {
  let w = World;
  println!("Hello, {}", w);
}
```

## Compile / execution model

I should say, the "compilesecution" model of Bloop, rather.


## Unified type declaration

```
// unit types
ty TypeX.;
ty TypeY.;

// named-tuple types
ty Foo(TypeX, TypeY);

// record type
ty Bar {
    x: TypeX,
	y: TypeY,
};

// enum type
ty Blahz {
    A,
	B,
};

// nested structs
ty Wut {
    // Wut::A
    a: ty A {
	   x: TypeX
	},
	// Wut::B
	b: ty B {
	   y: TypeY
	},
}

// union of tuple-structs
ty Baz {
    // Baz::A
    A(TypeX),
	// Baz::B
	B(TypeY),
};

// union of structs
ty Qux {
    // Qux::A
    A {
	    x: TypeX,
	},
	// Qux::B
	B {
	    y: TypeY,
	},
};

// mixed struct / union
//
// fields are type references by default
// or type constructors with the `tc` keyword
//
// tuple items are type references by default
// or type constructors with the `tc` keyword
//
// enum alternates are type constructors by
// default or type references with the `tr` keyword
tc Woah {
   x: TypeX,
   y: TypeY,
   a: tc A(TypeX),
   b: tc B { y: TypeY },
   C(TypeX),
   D(tc TypeY2),
   tr TypeX,
   tr TypeY,
}
```

// fields are priv by default,
// alts are pub by default,
```
tc Woah {
   x: TypeX, // priv
   pub y: TypeY,
   a: tc A(TypeX), // priv
   pub b: tc B { y: TypeY },
   priv C(TypeX), // p
   priv D(tc TypeY2),
   tr TypeX,
   tr TypeY,
}
```

// tcpe constructors in fields and alts
// have the same visibility as their
// field or alt.
```
tc Woah {
   x: TypeX,
   pub y: TypeY,
   a: tc A(TypeX),
   pub b: tc B { y: TypeY },
   C(TypeX),
   D(tc TypeY2),
   tr TypeX,
   tr TypeY,
}
```

Interior types are namable subtypes:

```
ty Foo {
    ty Bar,
	ty Baz,
}

let x: Foo::Bar = Foo::Bar;
let y: Foo = x;
```

## Intrinsic types

Similar to Rust.

```
ty Bool(prim_bool);
ty I8(prim_i8);
ty I16(prim_i16);
ty I32(prim_i32);
ty I64(prim_i64);
ty U8(prim_u8);
ty U16(prim_u16);
ty U32(prim_u32);
ty U64(prim_u64);
ty F32(prim_f32);
ty F64(prim_f64);
ty Iz(prim_iz);
ty Uz(prim_uz);
```

I don't want code to end up using isize by default everywhere, like it so often
does in rust because of painful conversions. Lossless conversions will be
everywhere, perhaps with an (optional) single-char coercion op.

## Error handling

Error type is an existential type. It can be 

```
fn foo(a: TypeA) -> Result<Usz, @Error> { ... }
```

The @ means 'existential', but it can be known-size existential.

Which is usually written as

```
fn foo(a: TypeA) -> Usz? { ... }
```

Existentials have 3 translation strategies that can be controlled
per call-site: monomorphization, dynamic dispatch, enumeration

## Panic handling

Panic types are @Error types. No boxing if @Error is known-size.

abort/unwind


## Modules and types

Types are modules, functions are types, but they each are defined
in different ways and expose different features of types.

import Foo;    // external module
mod Foo { }    // dag-friendly module that can't depend on its parent
depmod Foo { } // dag-unfriendly module that can depend on its parent
mod Foo;       // local module
depmod Foo;    // local module

type Foo; // unary-type
deptype Foo; // unary-type thaat can depond on its parent
type Foo(Bar, Baz); // tuple-type
type Foo { bar: Bar, baz: Baz } // struct-type

newtype Foo(bar) // deriving?

fn foo() { } // function type


## Traits

TODO wtf


All funtions are UCF traits

mod Foo;

trait Foo {
	fn foo() { }
}

can be imported as Foo.foo and called as Foo.foo() or foo()

atraits can be defined on types

type Foo;

trait Foo {
    fn foo(^self)
}

on alone

trait Bar {
    fn foo(^self)
}

traits are limitede such that they can e dispatched dynamicall, statically or interpreted

fn foo(bar: @(Foo, Bar)) { }

by default they are dispatched dynamically, C-like

static dispatch: fns (foo[s]: @(Foo, Bar) { }

or interpreted: fni (foo[i]: @(Foo, Bar) { }

or dynamic: fnd (foo: @(Foo Bar)) { }

functions can define their dispatch preference: fn (foo[s]: ^T) { }

fn foo(bar:

## Upward references

^foo denotes a reference up the stack. They cannot be returned and there is no lifetime notion

There are no down-stack references yet.


## Comments

// line comments
/// Markdown comments
//! Inner Markdown comments


## Indexing

There is no indexing syntax - indexables ipmlement tha `Call` trait on Ranges.


## About the name "Bloop"


## TODO

look into that video game language jai

## keywords

if
let
while
loop
move
match
return
continue
break
pub

## tokens

{ } [ ] < >

+ - / * = <= >= <. >. !

.. .._

. method / model delimiter
/ newmod top level escape

; terminator

^ upward pointer
* unsafe poitner, unsafe deferecnce

_ - wildcard match

<-
->

//
///

"strings"

'c'

r#"strungs#

b"strings"

b'c'

# macros

procuderal

foo!(..)* {
    token tree
}

item attributes

[ toml ]

inner item attributes

![ toml ]