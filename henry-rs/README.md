# Mark Henry

This document explains the markdown parser refactoring and provides detailed notes on how it works.

## Overview

The `parse_markdown` function converts raw markdown text into structured HTML and a table of contents (TOC), with proper semantic section nesting.

**Location:** `src/parser.rs`

---

## Table of Contents

### Core Sections
- [Architecture](#architecture)
  - [Design Principles](#design-principles)
  - [Key Dependencies](#key-dependencies)
- [Function Signature](#function-signature)
- [Core Algorithm](#core-algorithm)
  - [Step 1: Parse Markdown to Events](#step-1-parse-markdown-to-events)
  - [Step 2: Process Events](#step-2-process-events)
  - [Step 3: Match on Events](#step-3-match-on-events)
- [HTML Structure](#html-structure)
  - [Heading Structure](#heading-structure)
  - [Complete Example Output](#complete-example-output)
- [Table of Contents Generation](#table-of-contents-generation)
  - [TOC Structure](#toc-structure)
  - [CSS Classes](#css-classes)
- [Helper Functions](#helper-functions)
- [Examples](#examples)
  - [Example 1: Simple Document](#example-1-simple-document)
  - [Example 2: Multiple H3s Under One H2](#example-2-multiple-h3s-under-one-h2)

### Advanced Topics
- [State Management](#state-management)
  - [Variables](#variables)
  - [State Transitions](#state-transitions)
- [Important Patterns](#important-patterns)
  - [Pattern 1: Extracting Heading Text](#pattern-1-extracting-heading-text)
  - [Pattern 2: Event Buffering](#pattern-2-event-buffering)
  - [Pattern 3: Advancing Event Position](#pattern-3-advancing-event-position)
- [Rendering Notes](#rendering-notes)
  - [Why Manual Heading Rendering?](#why-manual-heading-rendering)
  - [Quote Escaping](#quote-escaping)
- [Edge Cases](#edge-cases)
- [Performance Considerations](#performance-considerations)
  - [Complexity](#complexity)
  - [Optimizations Applied](#optimizations-applied)
  - [Potential Improvements](#potential-improvements)
- [Testing](#testing)
- [Integration with JavaScript](#integration-with-javascript)
- [Summary](#summary)

---

## Architecture

### Design Principles

The parser follows these design principles:

1. **Synchronous Processing** - No async/await needed. Parsing is CPU-bound and fast.
2. **Two-Pass Approach** - Events are collected first, then processed
3. **Event-Driven** - Uses pulldown_cmark's event stream
4. **Semantic HTML** - Generates proper heading levels and section nesting
5. **Accessibility** - Includes ID anchors and permalink support

### Key Dependencies

```rust
use pulldown_cmark::{Parser, Options, Event, Tag, HeadingLevel};
use std::fmt::Write;
use crate::types::RenderedPage;
```

---

## Function Signature

```rust
pub fn parse_markdown(markdown_input: &str) -> RenderedPage {
    // ...
}

// Returns:
pub struct RenderedPage {
    pub content_html: String,    // Full rendered HTML with sections
    pub toc_html: String,        // Navigation menu/table of contents
}
```

**Note:** The function is **public** and **synchronous** (not async) so it can be called directly from the app component.

---

## Core Algorithm

### Step 1: Parse Markdown to Events

```rust
let mut options = Options::empty();
options.insert(Options::ENABLE_TABLES);
options.insert(Options::ENABLE_FOOTNOTES);

let parser = Parser::new_ext(markdown_input, options);
let events: Vec<Event> = parser.collect();
```

This converts raw markdown into a vector of events like:
- `Event::Start(Tag::Heading(...))`
- `Event::Text("My Heading")`
- `Event::End(Tag::Heading(...))`
- `Event::Start(Tag::Paragraph)`
- `Event::Text("Some text")`
- etc.

### Step 2: Process Events

The algorithm iterates through events with a state machine approach:

```rust
let mut content_html = String::new();
let mut toc_html = String::from("<ul>");
let mut pending: Vec<Event> = Vec::new();  // Buffer for non-heading events
let mut h2_open = false;                   // Track if H2 section is open
let mut last_h2_text = String::new();      // Remember parent heading for H3s
```

### Step 3: Match on Events

When a `Heading` event is found:

1. **Flush pending content** - Render any buffered non-heading events
2. **Extract heading text** - Collect all text within the heading
3. **Generate ID** - Convert "My Title" → "my-title" via slugify
4. **Handle heading level** - H1, H2, H3, etc. have different behaviors
5. **Update TOC** - Add entry to table of contents
6. **Skip heading event** - Heading markup is handled manually

For non-heading events:

1. **Buffer them** - Store in `pending` vector
2. **Flush when needed** - Before processing next heading
3. **Use pulldown_cmark rendering** - Let the library render these

---

## HTML Structure

### Heading Structure

#### H1 Headings
```html
<h1 id='my-title'>My Title<a href='#my-title' class='permalink'></a></h1>
```

- Top-level, not wrapped in sections
- Standalone anchor links

#### H2 Headings (Section Openers)

```html
<section id='h2-title'>
  <h2>H2 Title<a href='#h2-title' class='permalink'></a></h2>
  <!-- Content here -->
</section>
```

- Open a semantic `<section>` element
- All following H3s are children of this section
- State tracked with `h2_open` flag

#### H3 Headings (Subsections)

```html
<section id='h3-title' data-parent-heading='H2 Title'>
  <h3>H3 Title<a href='#h3-title' class='permalink'></a></h3>
</section>
```

- Own `<section>` element
- Include `data-parent-heading` attribute for JavaScript
- Closes immediately (not a container)

#### H4-H6 Headings

```html
<h4 id='h4-title'>H4 Title<a href='#h4-title' class='permalink'></a></h4>
```

- Simple headings with IDs
- No section wrapping
- No TOC entries

### Complete Example Output

```html
<h1 id='introduction'>Introduction</h1>
<p>Some intro text...</p>

<section id='philosophy'>
  <h2>Philosophy<a href='#philosophy' class='permalink'></a></h2>
  <p>Philosophy section content...</p>

  <section id='human-first-design' data-parent-heading='Philosophy'>
    <h3>Human-first design<a href='#human-first-design' class='permalink'></a></h3>
    <p>Details about human-first design...</p>
  </section>

  <section id='simplicity' data-parent-heading='Philosophy'>
    <h3>Simplicity<a href='#simplicity' class='permalink'></a></h3>
    <p>Details about simplicity...</p>
  </section>
</section>

<section id='guidelines'>
  <h2>Guidelines<a href='#guidelines' class='permalink'></a></h2>
  <p>Guidelines content...</p>
</section>
```

---

## Table of Contents Generation

### TOC Structure

```html
<ul>
  <li><a href='#introduction'>Introduction</a></li>
  <li><a href='#philosophy'>Philosophy</a></li>
  <li class='toc-h3'><a href='#human-first-design'>Human-first design</a></li>
  <li class='toc-h3'><a href='#simplicity'>Simplicity</a></li>
  <li><a href='#guidelines'>Guidelines</a></li>
</ul>
```

### CSS Classes

- `toc-h3` - Mark H3 entries for CSS styling (indentation, etc.)
- `visible` - Added by JavaScript when section is in viewport
- `active` - Highlight the currently active section

---

## Helper Functions

### `slugify(s: &str) -> String`

Converts heading text to URL-safe identifiers:

```rust
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .trim()
        .replace(|c: char| c.is_whitespace(), "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}
```

Examples:
- "My Title" → "my-title"
- "Hello World!" → "hello-world"
- "What?" → "what"
- "C++" → "c"

---

## Examples

### Example 1: Simple Document

**Input:**
```markdown
# Introduction

Some intro text.

## Getting Started

First, install the package.

### Installation

```bash
npm install
```
```

**Output HTML:**
```html
<h1 id='introduction'>Introduction<a href='#introduction' class='permalink'></a></h1>
<p>Some intro text.</p>

<section id='getting-started'>
  <h2>Getting Started<a href='#getting-started' class='permalink'></a></h2>
  <p>First, install the package.</p>

  <section id='installation' data-parent-heading='Getting Started'>
    <h3>Installation<a href='#installation' class='permalink'></a></h3>
    <pre><code class='language-bash'>npm install
</code></pre>
  </section>
</section>
```

**Output TOC:**
```html
<ul>
  <li><a href='#introduction'>Introduction</a></li>
  <li><a href='#getting-started'>Getting Started</a></li>
  <li class='toc-h3'><a href='#installation'>Installation</a></li>
</ul>
```

### Example 2: Multiple H3s Under One H2

**Input:**
```markdown
## Chapter 1

Intro text.

### Section A

Content A.

### Section B

Content B.

## Chapter 2

Chapter 2 intro.
```

**Output HTML:**
```html
<section id='chapter-1'>
  <h2>Chapter 1<a href='#chapter-1' class='permalink'></a></h2>
  <p>Intro text.</p>

  <section id='section-a' data-parent-heading='Chapter 1'>
    <h3>Section A<a href='#section-a' class='permalink'></a></h3>
    <p>Content A.</p>
  </section>

  <section id='section-b' data-parent-heading='Chapter 1'>
    <h3>Section B<a href='#section-b' class='permalink'></a></h3>
    <p>Content B.</p>
  </section>
</section>

<section id='chapter-2'>
  <h2>Chapter 2<a href='#chapter-2' class='permalink'></a></h2>
  <p>Chapter 2 intro.</p>
</section>
```

---

## State Management

### Variables

| Variable | Type | Purpose |
|----------|------|---------|
| `content_html` | String | Accumulated HTML output |
| `toc_html` | String | Navigation menu |
| `pending` | Vec<Event> | Buffer for non-heading events |
| `i` | usize | Current position in events |
| `last_h2_text` | String | Parent heading for H3 attributes |
| `h2_open` | bool | Whether H2 section needs closing |

### State Transitions

```
┌─────────────────┐
│  Start parsing  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│  Iterate through events                 │
└────────┬────────────────────────────────┘
         │
    ┌────┴─────┐
    │           │
    ▼           ▼
 HEADING    NON-HEADING
    │           │
    │      ┌────┴─────────────┐
    │      │  Buffer in       │
    │      │  pending vector  │
    │      └──────────────────┘
    │
    ├─────────────────────────┐
    │ Flush pending buffer    │
    │ (if not empty)          │
    └────────────┬────────────┘
                 │
    ┌────────────┴────────────┐
    │ Extract heading text    │
    └────────────┬────────────┘
                 │
    ┌────────────┴────────────┐
    │ Generate ID via slugify │
    └────────────┬────────────┘
                 │
         ┌───────┴────────┐
         │                │
    ┌────▼───┐  ┌────────▼────────┐
    │   H2   │  │  H3 or other    │
    └────┬───┘  └────────┬────────┘
         │               │
    ┌────▼───────────┐   │
    │ Close H2?      │   │
    │ Open new <sec> │   │
    └────┬───────────┘   │
         │                │
    ┌────▼────────────────▼────┐
    │ Generate HTML for heading │
    └────┬───────────────────────┘
         │
    ┌────▼──────────────────────┐
    │ Add entry to TOC           │
    └────┬──────────────────────┘
         │
    ┌────▼──────────────────────┐
    │ Move to next event         │
    └────┬──────────────────────┘
         │
    ┌────▴──────────────────────┐
    │ More events?               │
    └────┬──────────┬────────────┘
         │ YES      │ NO
         │          └─────┐
         └────────┬───────┘
                  │
         ┌────────▼──────────────┐
         │ Close open H2 section │
         │ Add closing </ul>     │
         │ Return RenderedPage   │
         └──────────────────────┘
```

---

## Important Patterns

### Pattern 1: Extracting Heading Text

```rust
let mut header_text = String::new();
let mut j = i + 1;
while j < events.len() {
    match &events[j] {
        Event::Text(t) | Event::Code(t) => header_text.push_str(t),
        Event::End(Tag::Heading(_, _, _)) => break,
        _ => {}
    }
    j += 1;
}
```

This looks ahead to collect all text nodes within a heading.

### Pattern 2: Event Buffering

```rust
// When we see a heading, flush previous content
if !pending.is_empty() {
    pulldown_cmark::html::push_html(&mut content_html, pending.into_iter());
    pending = Vec::new();
}

// For non-heading events, just buffer them
other => {
    pending.push(other.clone());
    i += 1;
}
```

This lets us handle headings specially while delegating paragraph/list rendering to pulldown_cmark.

### Pattern 3: Advancing Event Position

```rust
// Find the matching End tag
let mut k = j;
while k < events.len() {
    if let Event::End(Tag::Heading(_, _, _)) = &events[k] {
        break;
    }
    k += 1;
}
// Skip past the heading's End event
i = k + 1;
```

We skip the heading's `End` tag since we render it manually.

---

## Rendering Notes

### Why Manual Heading Rendering?

Instead of using pulldown_cmark's default heading rendering:
```html
<h2>My Title</h2>
```

We render custom:
```html
<h2>My Title<a href='#my-title' class='permalink'></a></h2>
```

And add section wrapping for H2:
```html
<section id='my-title'>
  <h2>My Title<a href='#my-title' class='permalink'></a></h2>
  <!-- Content -->
</section>
```

This provides:
1. **Semantic structure** - Proper document outline
2. **Accessibility** - ID anchors and permalinks
3. **JS Integration** - Sections for scroll-spy observation
4. **Styling flexibility** - CSS can target nested sections

### Quote Escaping

The parser uses single quotes in HTML attributes:
```html
<section id='my-title'>
```

Instead of:
```html
<section id="my-title">
```

This simplifies Rust string handling and avoids escaping issues.

---

## Edge Cases

### Empty Headings

```markdown
##
```

→ Empty string → slugify produces empty ID → Could cause issues

**Mitigation:** In real use, markdown editors don't allow empty headings.

### Special Characters in Headings

```markdown
## Hello "World"!
```

→ Slugifies to: `hello-world`
→ Quotes and punctuation are stripped

This ensures IDs are always valid HTML identifiers.

### Consecutive H2s with No H3

```markdown
## Chapter 1
Text...

## Chapter 2
Text...
```

→ First H2 section closes when second H2 opens
→ Proper nesting maintained

---

## Performance Considerations

### Complexity

- **Time:** O(n) where n = number of events
- **Space:** O(m) where m = total markdown character count

### Optimizations Applied

1. **Single pass** through events (after collection)
2. **Vec buffering** instead of recursive calls
3. **String::push_str** instead of format! for HTML
4. **write! macro** for safe string building

### Potential Improvements

1. **Stream processing** - Don't collect all events upfront
2. **StringBuilder** - Use more efficient string accumulation
3. **Lazy TOC** - Generate TOC only if needed

---

## Testing

### Test Cases to Consider

```rust
#[test]
fn test_simple_heading() {
    let md = "# Hello";
    let result = parse_markdown(md);
    assert!(result.content_html.contains("<h1 id='hello'"));
}

#[test]
fn test_h2_creates_section() {
    let md = "## Section";
    let result = parse_markdown(md);
    assert!(result.content_html.contains("<section id='section'>"));
}

#[test]
fn test_h3_with_parent() {
    let md = "## Parent\n\n### Child";
    let result = parse_markdown(md);
    assert!(result.content_html.contains("data-parent-heading='Parent'"));
}

#[test]
fn test_toc_generation() {
    let md = "# H1\n\n## H2\n\n### H3";
    let result = parse_markdown(md);
    assert!(result.toc_html.contains("H1"));
    assert!(result.toc_html.contains("H2"));
    assert!(result.toc_html.contains("H3"));
}
```

---

## Integration with JavaScript

The HTML structure is designed for JavaScript interaction:

### Scroll Spy (IntersectionObserver)

```javascript
// Watch sections as user scrolls
document.querySelectorAll("section[id]").forEach((section) => {
    observer.observe(section);
});

// When section enters viewport
entries.forEach((entry) => {
    const id = entry.target.getAttribute("id");
    const link = document.querySelector(`nav a[href="#${id}"]`);
    if (link) {
        link.parentElement.classList.toggle("visible", entry.isIntersecting);
    }
});
```

The IDs we generate make this possible.

### Permalink Clicks

```javascript
// User clicks permalink anchor
document.querySelector('a[href="#my-title"]').addEventListener('click', () => {
    // Browser handles smooth scroll to section
    // TOC updates via Intersection Observer
});
```

---

## Summary

The markdown parser:

1. ✅ Converts markdown to semantic HTML
2. ✅ Generates accessible heading IDs
3. ✅ Creates proper section nesting
4. ✅ Builds a table of contents
5. ✅ Integrates with scroll-spy JS
6. ✅ Maintains good performance
7. ✅ Handles special characters safely

The design balances **correctness**, **performance**, and **usability**.
