# The Bloop language

Minimal abstraction, massively parallel compile, fast execution, linear and
affine types. Memory safe. Rust compatible. C compatible.

In usus a custom parallel parser, all pases are parallel, incremental,
typechecking, resolution, and parsing, on demand, and considers the full
complitaion dag. Code lowering is delaid until after the entire dag is
typechecked. Code generation is with Cretonne.

There is minimal library integration, and the library stack is completely
seperate and well layered. The language knows nothing about allocators or
standard libraries.

There is no GC, local heaps for non-Send kinds.

Bloop features:

* fast development, fast code
* ahead-of-time compile or jit compile
* explicit unsafe, mostly unsafe (for now)
* tunable tradeoffs for generic translation and dispatch, error
  representation, existential representation
* opinionater error handling
* limited region checking
* parallel parsing
* parallel pipeline

* unified declaration for structs and enums
* modules are types, functions are trait methods on modules
- modules have speceial syntex? for when they are in another file
* unified funcion call syntax, unified traits and impls

* owned value types
* algebraic data types and matching
* types with implicit or explicit destruction (affine vs linear types)
* modules and libraries with precise view control
* markdown documentation

* static linking, no dynamic linking yuck it suck
* lazy, parallel compilation model
* whole-program analysis
* existentials c
* integrated package manager
* two-way cargo integration
* global optional unwinding
* toml attributes

* custom allocators
* undecided about GC and safety generally
* pinned types
* send and non-send types
* type anotition

* let bindings ref by default
* let bindings are mutable
* function bindings are move by default

* UFCS - all impls are traits

* newmod - module that can't referece parents
* extmod - external module found through resolution

* lexir lexes into syntax tree
- producural macros

## Tour

A Bloop program:

```blop
import std;

ty World;

fn main() {
  let w = World;
  println!("Hello, {}", w);
}
```

## Compile / execution model

I should say, the "compilesecution" model of Bloop, rather.

## Architecture

Lalrpop and Cretonne.

## Unified type declaration

```
// unit types
ty TypeX;
ty TypeY;

// tuple-struct
ty Foo(TypeX, TypeY);

// struct
ty Bar {
    x: TypeX,
	y: TypeY,
};


// nested type declaration
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

// (discriminated) union
ty Blahz {
    TypeX,
	TypeY,
};

// union of tuple-structs
ty Baz {
    // Baz::A
    ty A(TypeX),
	// Baz::B
	ty B(TypeY),
};

// union of structs
ty Qux {
    // Qux::A
    ty A {
	    x: TypeX,
	},
	// Qux::B
	ty B {
	    y: TypeY,
	},
};

// mixed struct / union
ty Woah {
   a: ty A(TypeX),
   b: ty B(TypeY),
   ty A(TypeX),
   ty B(TypeY),
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

Basic types:

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
ty Isz(prim_isz); "iz"
ty Usz(prim_usz); "uz"
```

## Error handling

Error type is an existential type. It can be 

```
fn foo(a: TypeA) -> Result<Usz, Error> { ... }
```

Which is usually written as

```
fn foo(a: TypeA) -> Usz? { ... }
```

## Modules and types

type Foo; // unary type
exttype Foo; // 
restype Foo; // type that needs to be resolved externally
roottype Foo; // type that can't reference parent types
newtype Foo(bar);
thistype; // the type defined for a module

mod Foo; // module
rootmod Foo; // mod that can't referece parents
resmod Foo; // mod needing external resolve
newmod Foo(bar); // type that inherits it's inner behavior

`mod` is sugar for `type`

## Traits

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

fn foo(bar: ^(Foo + Bar)) { }

by default they are dispatched dynamically, C-like

static dispatch: fns (foo[d]: ^(Foo Bar) { }

or interpreted: fni (foo[i]: ^(Foo Bar) { }

or dynamic: fnd ^(Foo Bar) { }

functions can define their dispatch preference: fn (foo[s]: ^T) { }

fn foo(bar:

## Upward references

^foo denotes a reference up the stack. They cannot be returned and there is no lifetime notion

There are no down-stack references yet.

## Comments

// line comments
/// Markdown comments
//! Inner markdown comments

## Indexing

There is no indexing syntax - indexables ipmlement tha `Call` trait on Ranges.

## About the name "Bloop"

Bloop is short for BlipBloop, which stands for "Binary LIPBLOP".

The LIPBLOP algorithm is rarely seen today, but was widely
celebrated on initial publication in the March 1968, etc.

Exhaustive and non-exhaustive enums

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

^ downward pointer
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

#macros

procuderal

foo!(..)* {
    token tree
}

in-code atrubute

!foo( toml )

item attributes

![ toml ]!

outer item attributes

[! toml !]