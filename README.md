# Mark Henry

## A standalone and robust markdown parser to satisfy its own ego

src/
├── lib.rs           <-- The Root (Registers all other files)
├── config.rs        <-- Constants (Repo, User, Branch)
├── types.rs         <-- Structs (MenuItem, RenderedPage)
├── parser.rs        <-- Markdown parsing logic
├── utils.rs         <-- DOM helpers (Observer, Highlighting)
└── app.rs           <-- The Main Yew Component

### some really good notes

for web-sys, setter-style methods mutate self and return &mut Self.
