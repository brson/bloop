# Bloop fundamental types

Bloop has no type keywords &mdash; all types are defined in libraries. Bloop does have "fundamental" types that the compiler or its plugins know about. These are connected to each other through a combination of _intrinsics_ and _extrinsics_.

Fundamental types are known to the compiler and may be implemented through intrinsics if supported by the platform.

A simple example of how the `Int128` type is defined:

```
// intrinsics/num/int128.bloop
#![cfg(have_feature(compiler_intrinsics::int128))]
#![feature(intrinsics::int128)]

import bloop::intrinsics::down::DownRef;
import bloop::intrinsics::bool::Bool;

#[bloop::intrinsic(type, int128)]
pub struct Int128;

#[bloop::intrinsic(type, int128_add)]
pub extern fn add(DownRef<Int128>, DownRef<Int128>) -> (Int128, Bool);
```

```
// crt/num/int128.bloop
```

```
// crt/num/int128-native.bloop
```

```
// crt/num/int128-fill.bloop
```

```
// extrinsics/num/int128.bloop
#![feature(num::int128)]

import bloop::intrinsics::num::int128 as i;
import bloop::intrinsics::down::DownRef;
import bloop::extrinsics::math::{
    Math, Add, Sub, Div, Mul, OverflowError
};

#[bloop::replace_module]
#[bloop::extrinsic(type, int128)]
pub struct Int128(i::Int128);

impl Math<Rhs = Int128> { }

impl Add<Rhs = Int128> for Int128 {
    fn add(DownRef<self>, other: DownRef<self>)
        -> Result<DownRef<self>, OverflowError>
    {
        i:add(self, other)
    }
}
```


## Fundamental types

- `Nil`
- `Never`
- `Bool`

- `Int8`
- `Int16`
- `Int32`
- `Int128`
- `Int128`
