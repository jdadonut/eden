# eden is a rust library used for native java ffi
by using a custom jvm, we're able to call java functions directly from Rust!

```rs
use eden::prelude::*;
use eden::j_invokestatic;

fn main() {
    prelude::init!();

    j_invokestatic("java.lang.System.out", "println", JObject::utf8("Hello World!"));
}
```

## why?
i do what i want
