# Bloop base language

This is the minimal high-level language that the compiler understands. It is not intended to be written by users. The full loop language is desugared to the base language by a series of procedural macros. Like other bloop intermediate languages it can be parsed from token trees. Its purpose is comparable to the Rust HIR.


## Example

```
//! An example Bloop base language module

// Plugin imports. These are attributes because they must
// be interpreted prior to parsing.

// Allow various shorthand attribute syntax, like omitting
// the `bloop` namespace, omitting `value =`, and interpreting
// paths as strings.
#![bloop::require(value = "bloop::attribute_desugaring")]
#![bloop::attribute_desugaring(default_namespace = "bloop")]

// Interpret `//!` comments as doc attributes
#![require(doc_comments)]

import bloop::cmp;
import bloop::cmp::Ord;
import bloop::cmp::Ordering;
import bloop::cmp::Eq;
import bloop::ref::Ref;

#[visibility(pub)]
struct DataStructure<TFoo: Foo> {
    fields {
        field1: Int32,
        field2: TFoo,
	}
	variants {
	    Variant1 { _0: Int32, },
		Variant2 { _0: TFoo, },
	}
}

#[visibility(pub)]
trait Foo: Ord {
    // Provided method. Cannot be overridden.
    fn is_group(Ref<self>,
	            other1: Ref<self>,
				other2: Ref<self>) -> Bool
	{
	    let cmp1 = self.cmp(other1);
		let cmp2 = self.cmp(other2);
		let cool1 = cmp1.is_cool();
		let cool2 = cmp2.is_cool();

		// Method chaining
		let res = cmp1.eq(cmp2).and(cool1).and(cool2);

		// Again, with type ascription
		let res: Bool = cmp1:Ordering.eq(cmp2:Ordering):Bool
		                         .and(cool1:Bool):Bool
								 .and(cool2:Bool):Bool;

        return res;
	}

    // Required method
	fn is_cool(Ref<self>) -> Nil;
}

// TODO: is it better for the trait name or the struct
// name to come first here?
impl Foo for DataStructure {
    // TODO
}
```


## Comments

Comments are line comments, beginning with `//`.


## Static paths


## Attributes

Attributes are metadata used to communicate a wide variety of information to the compiler and its plugins. Much information that would be reserved syntax in typical high level languages is represented as attributes in Bloop base language.

Attributes may appear anywhere in the lexical structure and it is up to the compiler and its plugins to validate them.

Attributes exist in all levels of the bloop language and are interpreted identically in each. They drive the configuration of the compiler, particularly its plugins, like macros.

An attribute consists of:

- A static path to a plugin that will interpret the attribute
- Optional boolean logic on configuration definitions
- New configuration definitions

Configuration definitions are in a format isomorphic to TOML.


## Modules

Modules have no declaration syntax. Every compilation unit is a module and every module is a compilation unit. Modules may depend on other modules through `import` statements. The module dependency graph must form a DAG. Unlike Rust there can be no mutual dependencies between modules. As such they are more like Rust crates, and the Rust module concept does not exist in Bloop.

The intent is to encourage decomposition into small compilation units,
and thus heavy compile-time parallelism, and to encourage abstraction boundaries. Creating modules is easy &mdash; there is no additional configuration file (i.e. `Cargo.toml`) required to create a module.


## Module structure

Modules contain a series of static _declarations_, which may be _imports_, _structs_, _functions_, _traits_, or _implementations_.


## Imports



## Data structures


## Functions


## Visibility


## Type ascription

