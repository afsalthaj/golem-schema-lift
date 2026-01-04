## Golem Schema Lift

**golem-schema-lift** is an escape-hatch crate for the Golem Rust SDK's schema (`golem_rust::Schema`) derivation.

The sole goal is to unblock users quickly by lifting current limitations in Golem’s agentic schema derivation — 
even if that means aggressive approaches under the hood.


## Why not this in Golem SDK directly?
There are legitimate reasons why Golem developers choose to not include solutions followed by this crate, 
and a high level reason is to keep the SDK simple and maintainable.


## Compatibility
Using `golem-schema-lift` will not make it hard for users once the `golem_rust` SDK resolves these schema related limitations natively. 
It is minimally invasive to your code, such that you hardly change your code once golem SDK matures.

## Features
Currently `golem-schema-lift` lifts only one limitation: the inability to derive Golem schemas for types that include recursive types. 

The way it works is by generating a non-recursive "wrapper" type around the recursive type, deriving the schema for that wrapper type, and then reusing that schema for the original recursive type.
It might look like easy inductive derivation, but it has some caveats to be discussed later. 

## Caveats
You might also think it is just an internal detail and doesn't leak to outside world, but sometimes it does. 
However, not always this is a problem. Not always everyone cares too.

### Example
The following fails to compile with just `golem_rust`, due to the recursive nature of `Expr`:
```rust 

use golem_rust::Schema;

#[derive(Schema)]
enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
}

```

And the following works with `golem-schema-lift`:

```rust
use golem_schema_lift::RecursiveSchema;

#[derive(RecursiveSchema)]
enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
}

```

As simple as that. Now, when golem really supports recursive types natively, you can just replace `golem_schema_lift::RecursiveSchema` with `golem_rust::Schema` and be done with it.
