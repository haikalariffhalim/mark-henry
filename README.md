# Mark Henry

## A standalone and robust markdown parser to satisfy its own ego

src/
├── lib.rs           
├── config.rs        
├── types.rs         
├── parser.rs        
├── utils.rs         
└── app.rs           

### some really good notes

1. for web-sys, setter-style methods mutate self and return &mut Self.

2. This is a closure, sort of a hook in the case where

```rust
   || → a closure that takes no arguments
   () → returns the unit type
```
“A function that takes and return but it return nothing and does nothing.”

3. The expected signature (conceptually). 
   
```rust

   FnOnce() -> impl FnOnce()

 ```
 
Meaning:

  The effect runs once (or when deps change)
  It returns a cleanup function
  The cleanup function runs when:
   - the component unmounts, or
   - the dependencies change

When NO cleanup to perform, there are something must be return.

That “something” is:

```rust

|| ()

```

example in the case of hook cases 

 ```rust

 use_effect(move || {
     let observer = create_observer();
 
     // with cleanup
     || {
         observer.disconnect();
     }
 });

```

``` rust

use_effect(move || {
    setup_observer();

    // no cleanup 
    || ()
});

```

Rust will shout error if it doent return. 
but omit something will cause error as well as it will not return expected types.

```rust

use_effect(move || {
    //try to omit value from closure 
    setup_observer();
});

```

So somehow something must be return which is nothing

It satisfies the type system by saying:

“Here i shall return it back to you, but its nothing, and doesnt do anothing. But atleast you know"
