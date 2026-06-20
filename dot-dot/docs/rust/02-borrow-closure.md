# Understanding Rust Borrowing and Closures

This document explains the borrow checker issues that were fixed in this project and provides practical examples to help you understand these fundamental Rust concepts.

## Table of Contents

### Main Sections
1. [Ownership Overview](#ownership-overview)
2. [Borrowing Basics](#borrowing-basics)
3. [The Borrow Checker](#the-borrow-checker)
4. [Closures and Capturing](#closures-and-capturing)
5. [The Bug We Fixed](#the-bug-we-fixed)
6. [Solutions and Examples](#solutions-and-examples)
7. [Key Takeaways](#key-takeaways)
8. [When Errors Occur](#when-errors-occur)
9. [Testing Your Understanding](#testing-your-understanding)
10. [Further Resources](#further-resources)

### Borrowing Basics Subsections
- [Immutable Borrowing (Multiple Readers)](#immutable-borrowing-multiple-readers)
- [Mutable Borrowing (Single Writer)](#mutable-borrowing-single-writer)
- [Key Rule: No Mixing Readers and Writers](#key-rule-no-mixing-readers-and-writers)

### Borrow Checker Subsections
- [Lifetime Scope](#lifetime-scope)

### Closures and Capturing Subsections
- [1. Capturing by Immutable Reference (Fn)](#1-capturing-by-immutable-reference-fn)
- [2. Capturing by Mutable Reference (FnMut)](#2-capturing-by-mutable-reference-fnmut)
- [3. Capturing by Move (FnOnce)](#3-capturing-by-move-fnonce)

### The Bug We Fixed Subsections
- [The Problem](#the-problem)
- [The Visualization](#the-visualization)

### Solutions and Examples Subsections
- [Solution 1: Clone the Value (What We Used)](#solution-1-clone-the-value-what-we-used)
- [Solution 2: Use Borrowing Instead of Moving](#solution-2-use-borrowing-instead-of-moving)
- [Solution 3: Use Shared Ownership with Rc and Arc](#solution-3-use-shared-ownership-with-rc-and-arc)
- [Practical Example: Web DOM Observation](#practical-example-web-dom-observation)

### Testing Your Understanding Subsections
- [Exercise 1: Will this compile?](#exercise-1-will-this-compile)
- [Exercise 2: Will this compile? (2)](#exercise-2-will-this-compile-1)
- [Exercise 3: Will this compile? (3)](#exercise-3-will-this-compile-2)
- [Exercise 4: Will this compile? (4)](#exercise-4-will-this-compile-3)

---

## Ownership Overview

Rust's ownership system is based on three rules:

1. **Each value has one owner**
2. **A value can be borrowed by multiple readers OR one mutable writer**
3. **When the owner goes out of scope, the value is dropped**

```rust
fn main() {
    let s = String::from("hello");  // s owns the String
    println!("{}", s);               // s is still valid here
    
    // At the end of main(), s goes out of scope and the String is dropped
}
```

---

## Borrowing Basics

Instead of transferring ownership, we can **borrow** values using references (`&`).

### Immutable Borrowing (Multiple Readers)

```rust
fn main() {
    let s = String::from("hello");
    
    let r1 = &s;  // Immutable borrow
    let r2 = &s;  // Another immutable borrow - OK!
    let r3 = &s;  // And another - OK!
    
    println!("{}, {}, {}", r1, r2, r3);  // All can read simultaneously
    
    println!("{}", s);  // Original owner can still use it
}
```

### Mutable Borrowing (Single Writer)

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;  // Mutable borrow
    // let r2 = &mut s;  // ERROR! Can't have two mutable borrows
    
    r1.push_str(" world");
    println!("{}", r1);  // OK
}
```

### Key Rule: No Mixing Readers and Writers

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // Immutable borrow
    let r2 = &s;      // Another immutable borrow
    // let r3 = &mut s;  // ERROR! Can't have mutable borrow while immutable borrows exist
    
    println!("{}, {}", r1, r2);  // Use immutable borrows
    
    // Now mutable borrow is OK (no more immutable borrows in use)
    let r3 = &mut s;
    r3.push_str("!");
    println!("{}", r3);
}
```

---

## The Borrow Checker

The borrow checker is Rust's compile-time system that enforces borrowing rules. It prevents data races and use-after-free bugs at compile time, before your code even runs.

### Lifetime Scope

A borrow is **in scope** from the point it's created until its last use.

```rust
fn main() {
    let s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    
    println!("{}", r1);  // r1's last use is here
    println!("{}", r2);  // r2's last use is here
    
    // After this point, no immutable borrows are in scope
    let r3 = &mut s;     // OK! No immutable borrows are active
    r3.push_str("!");
    println!("{}", r3);
}
```

---

## Closures and Capturing

Closures are functions that can capture variables from their surrounding scope. They have different ways of capturing:

### 1. Capturing by Immutable Reference (Fn)

```rust
fn main() {
    let x = 5;
    let y = 10;
    
    // Closure captures x and y by immutable reference
    let add = |a| {
        println!("x is {}", x);  // Reads x
        a + x + y
    };
    
    println!("{}", add(3));  // 3 + 5 + 10 = 18
    
    // Can still use x and y here
    println!("x is still {}", x);  // OK!
}
```

### 2. Capturing by Mutable Reference (FnMut)

```rust
fn main() {
    let mut count = 0;
    
    // Closure captures count by mutable reference
    let mut increment = || {
        count += 1;  // Mutates count
        count
    };
    
    println!("{}", increment());  // 1
    println!("{}", increment());  // 2
    
    // Can't use count here while closure might use it
    // println!("{}", count);  // ERROR!
}
```

### 3. Capturing by Move (FnOnce)

```rust
fn main() {
    let s = String::from("hello");
    
    // Closure takes ownership of s
    let consume = move || {
        println!("{}", s);  // Consumes s
    };
    
    // println!("{}", s);  // ERROR! s has been moved
    
    consume();  // s is consumed here
    // consume();  // ERROR! Can't call again, s is gone
}
```

---

## The Bug We Fixed

### The Problem

In `src/app.rs`, we had code like this:

```rust
use_effect_with(content_data.clone(), move |_| {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();  // doc is created here

    // This closure captures doc by move (because of the outer `move` keyword)
    let cb = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _| {
        // Inside this closure, we try to use doc
        let selector = format!("#TableOfContents a[href='#{}']", target_id);
        if let Ok(Some(link)) = doc.query_selector(&selector) {  //  ERROR!
            // ...
        }
    }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

    // After creating cb, we try to use doc again
    if let Ok(sections) = doc.query_selector_all("section[id]") {  //  ERROR!
        // ...
    }
});
```

**What went wrong:**

1. The outer `move` closure captured `doc`
2. Inside that closure, we created an inner `move` closure (`cb`)
3. The inner closure tried to capture `doc` again
4. But `doc` was already moved by the outer closure!
5. Then we tried to use `doc` after it was moved into `cb`

This is a **"borrow of moved value"** error.

### The Visualization

```
Step 1: doc is created
    let doc = window.document().unwrap();
    ↓
    doc (owner)

Step 2: doc is moved into cb closure
    let cb = Closure::wrap(Box::new(move |entries| {
        doc.query_selector(&selector)  // doc moved here
    }));
    ↓
    doc (now owned by cb)

Step 3: Try to use doc again
    if let Ok(sections) = doc.query_selector_all(...) {  // ERROR!
        // doc was already moved to cb in Step 2
    }
```

---

## Solutions and Examples

### Solution 1: Clone the Value (What We Used)

**When to use:** When cloning is cheap or you need multiple independent copies

```rust
use_effect_with(content_data.clone(), move |_| {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    
    // Clone doc so inner closure can have its own copy
    let doc_clone = doc.clone();  //  Key fix!

    let cb = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _| {
        // Inner closure uses the clone
        if let Ok(Some(link)) = doc_clone.query_selector(&selector) {  //  OK!
            // ...
        }
    }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

    // Outer closure still has the original doc
    if let Ok(sections) = doc.query_selector_all("section[id]") {  // OK!
        // ...
    }
});
```

**Why this works:**
- `doc` is wrapped in a `Rc` (reference counted smart pointer) from web-sys
- Cloning `Rc` doesn't clone the underlying data, just increments a counter
- Both the original and clone point to the same JavaScript Document object
- The closure gets its own reference that keeps the document alive

### Solution 2: Use Borrowing Instead of Moving

**When to use:** When you don't need to move ownership

```rust
use_effect_with(content_data.clone(), move |_| {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();

    // Don't use 'move' if you don't need to move doc
    let cb = Closure::wrap(Box::new(|entries: Vec<JsValue>, _| {
        // Borrow doc instead of moving it
        // This requires doc to be accessible from the outer scope
        // and stay alive long enough
    }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);
});
```

However, this won't work here because the closure needs to outlive the scope where doc is defined.

### Solution 3: Use Shared Ownership with Rc and Arc

**When to use:** For complex ownership scenarios with multiple threads or lifetimes

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn example() {
    let doc = Rc::new(RefCell::new(/* some data */));
    let doc_clone = doc.clone();  // Shared ownership
    
    let closure = move || {
        // Both doc and closure can access the data
        let borrowed = doc_clone.borrow();
        // Use borrowed data
    };
}
```

---

## Practical Example: Web DOM Observation

Here's a complete example that mirrors your fixed code:

```rust
fn setup_observer() {
    // Create the document reference
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Clone it for the closure to use
    let document_for_closure = document.clone();

    // Create the intersection observer callback
    let callback = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _| {
        // This closure has its own reference to the document
        for entry in entries {
            let target_id = entry.target().get_attribute("id").unwrap_or_default();
            let selector = format!("a[href='#{}']", target_id);
            
            // Use document_for_closure safely
            if let Ok(Some(link)) = document_for_closure.query_selector(&selector) {
                if entry.intersection_ratio() > 0.0 {
                    link.parent_element().unwrap().class_list().add_1("visible").ok();
                }
            }
        }
    }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

    // Create the observer
    let mut options = IntersectionObserverInit::new();
    options.set_root_margin("0px 0px -70% 0px");

    if let Ok(observer) = IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) {
        callback.forget();  // Prevent the callback from being dropped
        
        // Now we can use the original document reference here
        if let Ok(sections) = document.query_selector_all("section[id]") {
            for i in 0..sections.length() {
                observer.observe(&sections.item(i).unwrap().unchecked_into());
            }
        }
    }
}
```

---

## Key Takeaways

| Concept | Rule | Example |
|---------|------|---------|
| **Ownership** | One owner per value | `let s = String::from("hello");` |
| **Immutable Borrow** | Multiple readers allowed | `let r1 = &s;` |
| **Mutable Borrow** | Only one writer allowed | `let r = &mut s;` |
| **Move** | Ownership transferred | `let s2 = s;` (s no longer valid) |
| **Closure Capture** | Can capture by ref or move | `\|x\| x + captured_var` |
| **Clone** | Create independent copy | `let copy = original.clone();` |

---

## When Errors Occur

### Compile-Time vs Runtime

 **Rust catches these at compile time:**
- Borrow of moved value
- Use of mutable reference while immutable references exist
- Multiple mutable references

 **These would happen in other languages (runtime errors):**
- Data races
- Use-after-free
- Null pointer dereferences

---

## Testing Your Understanding

Try to predict what each code snippet will do:

### Exercise 1: Will this compile?

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // ?
    println!("{}", r1);
}
```

<details>
<summary>Answer</summary>
**No!** Error: "cannot borrow `s` as mutable because it is also borrowed as immutable"

The immutable borrows `r1` and `r2` are still in scope when we try to create a mutable borrow `r3`.
</details>

### Exercise 2: Will this compile?

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);  // Last use of r1 and r2
    
    let r3 = &mut s;  // ?
    r3.push_str("!");
    println!("{}", r3);
}
```

<details>
<summary>Answer</summary>
**Yes!** The immutable borrows `r1` and `r2` ended after the println, so `r3` can be created.

This is called **non-lexical lifetimes (NLL)** - borrows are released at their last use, not at end of scope.
</details>

### Exercise 3: Will this compile?

```rust
fn main() {
    let x = 5;
    let closure = || {
        x + 1  // Captures x by immutable reference
    };
    
    println!("{}", x);
    println!("{}", closure());
}
```

<details>
<summary>Answer</summary>
**Yes!** The closure captures `x` immutably, so `x` can still be used in the main function.
</details>

### Exercise 4: Will this compile?

```rust
fn main() {
    let mut x = 5;
    let mut closure = || {
        x += 1;  // Captures x by mutable reference
    };
    
    println!("{}", x);  // ?
    closure();
}
```

<details>
<summary>Answer</summary>
**No!** Error: "cannot borrow `x` as immutable because it is also borrowed as mutable"

The mutable closure holds a mutable borrow of `x`, so we can't borrow it immutably with `println!`.
</details>

---

## Further Resources

- [The Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [The Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
