# Project Documentation Guide

Welcome to the mark-henry project! This guide helps you navigate the documentation and understand the codebase.

##  Documentation Files

This project includes several comprehensive documentation files:

### 1. **BORROWING_AND_CLOSURES.md** - Rust Fundamentals
**For:** Understanding Rust's ownership system and how the bug was fixed

- **Covers:**
  - Ownership rules and borrowing concepts
  - The borrow checker and lifetime scopes
  - How closures capture variables
  - The exact bug that was fixed
  - Three different solutions with examples
  - 4 interactive exercises with answers

- **Best for:**
  - Learning Rust's ownership system
  - Understanding why the app.rs error occurred
  - Improving your Rust fundamentals
  - Testing your knowledge with exercises

- **Quick Links:**
  - [The Bug We Fixed](#the-bug-we-fixed) - See what went wrong
  - [Solutions and Examples](#solutions-and-examples) - Learn how to fix it
  - [Testing Your Understanding](#testing-your-understanding) - Practice with exercises

### 2. **PARSER_DOCUMENTATION.md** - Markdown Parsing
**For:** Understanding how markdown is converted to HTML

- **Covers:**
  - Parser architecture and design
  - The parsing algorithm step-by-step
  - HTML structure generation
  - Table of contents creation
  - State management patterns
  - Integration with JavaScript

- **Best for:**
  - Understanding how markdown is processed
  - Learning about event-driven parsing
  - Understanding HTML structure generation
  - Seeing real examples of input/output
  - Integration with scroll-spy functionality

- **Quick Links:**
  - [Core Algorithm](#core-algorithm) - How parsing works
  - [HTML Structure](#html-structure) - What HTML is generated
  - [Examples](#examples) - Real input/output examples
  - [Integration with JavaScript](#integration-with-javascript) - How JS uses the HTML

---

##  Navigation Guide

### Starting Fresh?
1. **First, read:** [README.md](README.md) for project overview
2. **Then, understand:** [BORROWING_AND_CLOSURES.md](#ownership-overview) for Rust concepts
3. **Finally, explore:** [PARSER_DOCUMENTATION.md](#architecture) for how parsing works

### Learning Rust Borrowing?
1. Start with [Ownership Overview](#ownership-overview)
2. Progress through [Borrowing Basics](#borrowing-basics)
3. Jump to [The Bug We Fixed](#the-bug-we-fixed)
4. Try the [Testing Your Understanding](#testing-your-understanding) exercises

### Understanding the Parser?
1. Read [Architecture](#architecture) for design principles
2. Study [Core Algorithm](#core-algorithm) for step-by-step logic
3. Review [Examples](#examples) for concrete input/output
4. Check [Integration with JavaScript](#integration-with-javascript) to see it in action

### Fixing a Bug?
1. Check [src/parser.rs](#) for parsing logic
2. Check [src/app.rs](#) for React component logic
3. Refer to [Rendering Notes](#rendering-notes) for HTML generation
4. Use [Testing](#testing) to verify changes

---

##  Document Overviews

### BORROWING_AND_CLOSURES.md

This document explains Rust's ownership and borrowing system through the lens of the bug we fixed.

```
Overview:
├── Ownership System
│   ├── Three ownership rules
│   └── Basic examples
├── Borrowing
│   ├── Immutable borrowing
│   ├── Mutable borrowing
│   └── Mixing rules
├── The Borrow Checker
│   └── Lifetime scopes
├── Closures
│   ├── Capturing by reference
│   ├── Capturing by move
│   └── Mutable capture
├── The Bug We Fixed
│   ├── What went wrong (the error)
│   ├── Visualization (diagrams)
│   └── Step-by-step breakdown
├── Three Solutions
│   ├── Solution 1: Cloning  (What we used)
│   ├── Solution 2: Borrowing
│   └── Solution 3: Rc/Arc smart pointers
├── Practical Example
│   └── Web DOM observation code
└── Exercises
    ├── Exercise 1: Multiple references
    ├── Exercise 2: Non-lexical lifetimes
    ├── Exercise 3: Closure capture
    └── Exercise 4: Mutable closure borrow
```

**Key Sections:**
- [Ownership Overview](BORROWING_AND_CLOSURES.md#ownership-overview)
- [Borrowing Basics](BORROWING_AND_CLOSURES.md#borrowing-basics)
- [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed)
- [Solutions](BORROWING_AND_CLOSURES.md#solutions-and-examples)
- [Exercises](BORROWING_AND_CLOSURES.md#testing-your-understanding)

### PARSER_DOCUMENTATION.md

This document explains the markdown parser in detail.

```
Overview:
├── Architecture
│   ├── Design principles (5 key principles)
│   └── Dependencies
├── Function Signature
├── Core Algorithm
│   ├── Step 1: Parse to events
│   ├── Step 2: Process events
│   └── Step 3: Match on events
├── HTML Structure
│   ├── H1 headings (standalone)
│   ├── H2 headings (section openers)
│   ├── H3 headings (subsections)
│   └── H4-H6 headings (simple)
├── Table of Contents
│   ├── TOC structure
│   └── CSS classes
├── Examples
│   ├── Simple document example
│   └── Complex multi-section example
├── State Management
│   ├── Key variables
│   └── State transitions (flowchart)
├── Patterns
│   ├── Extracting heading text
│   ├── Event buffering
│   └── Position advancement
├── Rendering Notes
│   ├── Manual heading rendering (why)
│   └── Quote escaping
├── Edge Cases
│   ├── Empty headings
│   ├── Special characters
│   └── Consecutive headings
├── Performance
│   ├── Time/space complexity
│   ├── Optimizations applied
│   └── Potential improvements
├── Testing
│   └── Test cases to consider
└── JavaScript Integration
    ├── Scroll-spy with IntersectionObserver
    └── Permalink functionality
```

**Key Sections:**
- [Architecture](PARSER_DOCUMENTATION.md#architecture)
- [Core Algorithm](PARSER_DOCUMENTATION.md#core-algorithm)
- [HTML Structure](PARSER_DOCUMENTATION.md#html-structure)
- [Examples](PARSER_DOCUMENTATION.md#examples)
- [JavaScript Integration](PARSER_DOCUMENTATION.md#integration-with-javascript)

---

## Quick Reference

### Rust Concepts Quick Links

| Concept | Link | Purpose |
|---------|------|---------|
| Ownership | [Ownership Overview](BORROWING_AND_CLOSURES.md#ownership-overview) | Understand the three rules |
| Borrowing | [Borrowing Basics](BORROWING_AND_CLOSURES.md#borrowing-basics) | Mutable vs immutable refs |
| The Borrow Checker | [The Borrow Checker](BORROWING_AND_CLOSURES.md#the-borrow-checker) | How it works at compile time |
| Closures | [Closures and Capturing](BORROWING_AND_CLOSURES.md#closures-and-capturing) | Three capture strategies |
| The Bug | [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed) | Actual error in app.rs |
| Solutions | [Solutions](BORROWING_AND_CLOSURES.md#solutions-and-examples) | Three different fixes |
| Exercises | [Testing](BORROWING_AND_CLOSURES.md#testing-your-understanding) | Practice problems |

### Parser Concepts Quick Links

| Concept | Link | Purpose |
|---------|------|---------|
| Design | [Architecture](PARSER_DOCUMENTATION.md#architecture) | 5 key principles |
| Algorithm | [Core Algorithm](PARSER_DOCUMENTATION.md#core-algorithm) | Step-by-step process |
| HTML Output | [HTML Structure](PARSER_DOCUMENTATION.md#html-structure) | What gets generated |
| Examples | [Examples](PARSER_DOCUMENTATION.md#examples) | Input/output pairs |
| State | [State Management](PARSER_DOCUMENTATION.md#state-management) | Variables and flow |
| Patterns | [Patterns](PARSER_DOCUMENTATION.md#important-patterns) | Common code patterns |
| JavaScript | [JS Integration](PARSER_DOCUMENTATION.md#integration-with-javascript) | DOM observation |

---

## Learning Paths

### Path 1: "I want to understand this bug"
**Time:** 30-45 minutes

1. [The Problem](BORROWING_AND_CLOSURES.md#the-problem) - See the error
2. [The Visualization](BORROWING_AND_CLOSURES.md#the-visualization) - Understand it
3. [Solution 1](BORROWING_AND_CLOSURES.md#solution-1-clone-the-value-what-we-used) - See the fix
4. Read [src/app.rs](#) - View the fixed code

### Path 2: "I want to learn Rust borrowing"
**Time:** 2-3 hours

1. [Ownership Overview](BORROWING_AND_CLOSURES.md#ownership-overview)
2. [Borrowing Basics](BORROWING_AND_CLOSURES.md#borrowing-basics)
3. [The Borrow Checker](BORROWING_AND_CLOSURES.md#the-borrow-checker)
4. [Closures and Capturing](BORROWING_AND_CLOSURES.md#closures-and-capturing)
5. Work through all [Exercises](BORROWING_AND_CLOSURES.md#testing-your-understanding)

### Path 3: "I want to understand the parser"
**Time:** 1-2 hours

1. [Architecture](PARSER_DOCUMENTATION.md#architecture) - Principles
2. [Core Algorithm](PARSER_DOCUMENTATION.md#core-algorithm) - How it works
3. [Examples](PARSER_DOCUMENTATION.md#examples) - See it in action
4. Read [src/parser.rs](#) - View the code
5. [State Management](PARSER_DOCUMENTATION.md#state-management) - Deep dive

### Path 4: "I want to understand the whole project"
**Time:** 4-6 hours

1. Start with [BORROWING_AND_CLOSURES.md](#) (complete)
2. Continue with [PARSER_DOCUMENTATION.md](#) (complete)
3. Read source code:
   - [src/parser.rs](#) - Markdown parsing
   - [src/app.rs](#) - React component
   - [assets/main.js](#) - Scroll-spy JavaScript
4. Review fixed bugs in each file

---

## Document Features

### BORROWING_AND_CLOSURES.md Features
[] Multiple real-world examples  
[] Visual diagrams and flowcharts  
[] Comparison with other languages  
[] Interactive exercises with answers  
[] Link to Rust book resources  

### PARSER_DOCUMENTATION.md Features
[] Step-by-step algorithm explanation  
[] Complete input/output examples  
[] State transition flowchart  
[] Performance analysis  
[] Test case suggestions  

---

##  All Document Links

### Borrowing & Closures Main Sections
- [Ownership Overview](BORROWING_AND_CLOSURES.md#ownership-overview)
- [Borrowing Basics](BORROWING_AND_CLOSURES.md#borrowing-basics)
- [The Borrow Checker](BORROWING_AND_CLOSURES.md#the-borrow-checker)
- [Closures and Capturing](BORROWING_AND_CLOSURES.md#closures-and-capturing)
- [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed)
- [Solutions and Examples](BORROWING_AND_CLOSURES.md#solutions-and-examples)
- [Key Takeaways](BORROWING_AND_CLOSURES.md#key-takeaways)
- [Testing Your Understanding](BORROWING_AND_CLOSURES.md#testing-your-understanding)

### Parser Documentation Main Sections
- [Architecture](PARSER_DOCUMENTATION.md#architecture)
- [Function Signature](PARSER_DOCUMENTATION.md#function-signature)
- [Core Algorithm](PARSER_DOCUMENTATION.md#core-algorithm)
- [HTML Structure](PARSER_DOCUMENTATION.md#html-structure)
- [Table of Contents Generation](PARSER_DOCUMENTATION.md#table-of-contents-generation)
- [Helper Functions](PARSER_DOCUMENTATION.md#helper-functions)
- [Examples](PARSER_DOCUMENTATION.md#examples)
- [State Management](PARSER_DOCUMENTATION.md#state-management)
- [Important Patterns](PARSER_DOCUMENTATION.md#important-patterns)
- [Integration with JavaScript](PARSER_DOCUMENTATION.md#integration-with-javascript)

---

##  Quick Start

### If you have 5 minutes:
Read: [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed)

### If you have 15 minutes:
Read:
1. [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed)
2. [Solution 1](BORROWING_AND_CLOSURES.md#solution-1-clone-the-value-what-we-used)

### If you have 1 hour:
Read:
1. [Borrowing Basics](BORROWING_AND_CLOSURES.md#borrowing-basics)
2. [Closures and Capturing](BORROWING_AND_CLOSURES.md#closures-and-capturing)
3. [The Bug We Fixed](BORROWING_AND_CLOSURES.md#the-bug-we-fixed)
4. [Solutions and Examples](BORROWING_AND_CLOSURES.md#solutions-and-examples)

### If you have a full day:
1. Complete [BORROWING_AND_CLOSURES.md](#)
2. Complete [PARSER_DOCUMENTATION.md](#)
3. Read all source code

---

## How to Use These Docs

### While Reading Code
If you see something in `src/app.rs` or `src/parser.rs` that confuses you, look it up:
- Rust concepts → [BORROWING_AND_CLOSURES.md](#)
- Parser logic → [PARSER_DOCUMENTATION.md](#)

### Before Making Changes
1. Find the relevant section in the docs
2. Understand the design principles
3. Check the state management section
4. Review examples of similar changes

### When Debugging
1. Check [State Management](#state-management) in parser docs
2. Check [Testing](#testing) for test cases
3. Refer to [Edge Cases](#edge-cases)

---

## Summary

| Document | Best For | Time | Difficulty |
|----------|----------|------|------------|
| BORROWING_AND_CLOSURES.md | Learning Rust, understanding the bug | 2-3 hrs | Intermediate |
| PARSER_DOCUMENTATION.md | Understanding markdown parsing | 1-2 hrs | Intermediate |
| This Guide | Navigation and quick reference | 15 min | Easy |

---

## Key Takeaways

1. **The Bug:** The `doc` variable was moved into a closure, then we tried to use it again
2. **The Fix:** Clone `doc` before moving it into the inner closure
3. **Why It Matters:** Rust's ownership system prevents data races at compile time
4. **The Parser:** Uses event-driven approach to transform markdown into semantic HTML
5. **JavaScript Integration:** Parser generates IDs and structure that JavaScript uses for scroll-spy

---

**Happy learning! If you have questions, refer to the specific documentation sections using the TOC and links above.**
