
# Nineties

This is a traditional starter for web with rust.

The started app has a login wall and everything you need to start: a basic MVC structure.

This is an early stage of this project, and it currently creates an MVC structure with the following structure:

```
my-project/
├── Cargo.lock
├── Cargo.toml
├── database
├── diesel.toml
├── dist
│ └── imgs
│     └── nineties-logo.png
├── migrations
│ └── 2024-12-16-134059_create_users
│     ├── down.sql
│     └── up.sql
├── package.json
├── package-lock.json
├── src
│ ├── console
│ │ └── development.rs
│ ├── database
│ │ └── seeders
│ │     └── create_users.rs
│ ├── helpers.rs
│ ├── http
│ │ ├── controllers
│ │ │ ├── admin_controller.rs
│ │ │ ├── auth_controller.rs
│ │ │ └── home_controller.rs
│ │ └── middlewares
│ │     └── auth_middleware.rs
│ ├── main.rs
│ ├── models
│ │ └── user.rs
│ ├── resources
│ │ ├── css
│ │ │ └── styles.css
│ │ ├── imgs
│ │ │ └── nineties-logo.png
│ │ ├── js
│ │ │ └── script.js
│ │ └── views
│ │     ├── admin
│ │     │ ├── dashboard.html
│ │     │ ├── parts
│ │     │ ├── settings.html
│ │     │ └── signin-form.html
│ │     ├── home.html
│ │     ├── parts
│ │     │ ├── footer.html
│ │     │ ├── header.html
│ │     │ ├── hero.html
│ │     │ ├── html-head.html
│ │     │ └── notification.html
│ │     └── signin.html
│ ├── routes.rs
│ └── schema.rs
└── tailwind.config.js
```

## Quick Start

**Step 1**: Download the release binary.

**Step 2**: Create a project with nineties:

```bash
./nineties my_project
```

**Step 3**: Go inside your project, seed and and start develop

```bash
cd my_project
cargo run seed
cargo run develop
```

> Note 1: The server will run on `http://localhost:8080`
> Note 2: This "develop" command will run server with hot-reload and tailwind bundling.
> Note 3: To just run the server, use `cargo run serve`

---

That's it, now you can develop.
