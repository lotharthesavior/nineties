
# Nineties

This is a traditional starter for web with rust.

This is an early stage of this project, and it currently creates an MVC structure with the following structure:

```
my-project/
├── controllers/
│   └── home_controller.rs
├── resources/
│   ├── views/
│   │   └── home.html
│   ├── css/
│   │   └── styles.css
│   └── js/
│       └── scripts.js
├── src/
│   ├── helpers.rs
│   ├── routes.rs
│   └── main.rs
└── package.json
```

## Quick Start

**Step 1**: Download the release binary.

**Step 2**: Create a project with cargo:

```bash
cargo new my_project
```

**Step 3**: Run nineties within that project's directory:

```bash
cd my_project
nineties
```

Step 4: run your project with 2 terminals running the following:

Server:

```bash
cargo run
```

> Note 1: The server will run on `http://localhost:8000`

> Note 2: The server will not automatically reload on changes. If you want that, you need to
>         install `cargo-watch` (`cargo install cargo-watch`) and run `cargo watch -x run`.

Tailwind css assets:

```bash
npm run build:css
```

---

That's it, now you can develop.