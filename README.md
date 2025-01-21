
<h1 align="center">Nineties
<img src="assets/nineties-logo.png" alt="Nineties - Web App" style="width: 44px; height: 44px;" width="44" height="44" /></h1>

[![Build and Release](https://github.com/lotharthesavior/nineties/actions/workflows/release.yml/badge.svg)](https://github.com/lotharthesavior/nineties/actions/workflows/release.yml) [![Build and Test](https://github.com/lotharthesavior/nineties/actions/workflows/tests.yml/badge.svg)](https://github.com/lotharthesavior/nineties/actions/workflows/tests.yml)

This is a starter for web with rust on top of [Actix](https://actix.rs).

Spend time with **your ideas** on top of a solid foundation.

It creates an MVC structure with the following structure:

```
my-project/
├── Cargo.lock
├── Cargo.toml
├── database
├── diesel.toml
├── dist
├── migrations
│   └── 2024-12-16-134059_create_users
│       ├── down.sql
│       └── up.sql
├── package.json
├── package-lock.json
├── postcss.config.js
├── src
│   ├── console
│   │   └── development.rs
│   ├── database
│   │   └── seeders
│   │       ├── create_users.rs
│   │       ├── mod.rs
│   │       └── traits
│   │           ├── mod.rs
│   │           └── seeder.rs
│   ├── helpers
│   │   ├── database.rs
│   │   ├── form.rs
│   │   ├── general.rs
│   │   ├── session.rs
│   │   ├── template.rs
│   │   └── test.rs
│   ├── http
│   │   ├── controllers
│   │   │   ├── admin_controller.rs
│   │   │   ├── auth_controller.rs
│   │   │   └── home_controller.rs
│   │   └── middlewares
│   │       └── auth_middleware.rs
│   ├── main.rs
│   ├── models
│   │   └── user.rs
│   ├── resources
│   │   ├── css
│   │   │   └── styles.css
│   │   ├── imgs
│   │   │   └── nineties-logo.png
│   │   ├── js
│   │   │   └── script.js
│   │   └── views
│   │       ├── admin
│   │       │   ├── index.html
│   │       │   ├── pages
│   │       │   │   ├── dashboard.html
│   │       │   │   ├── profile.html
│   │       │   │   └── settings.html
│   │       │   ├── parts
│   │       │   │   ├── side-menu.html
│   │       │   │   └── top-menu.html
│   │       │   └── signin-form.html
│   │       ├── home.html
│   │       ├── parts
│   │       │   ├── components.html
│   │       │   ├── footer.html
│   │       │   ├── header.html
│   │       │   ├── hero.html
│   │       │   ├── html-head.html
│   │       │   ├── notification.html
│   │       │   └── open-graph.html
│   │       └── signin.html
│   ├── routes.rs
│   ├── schema.rs
│   └── services
│       └── user_service.rs
├── tailwind.config.js
└── vite.config.js
```

## Quick Start

**Step 1**: Install the create:

```bash
cargo install cargo-watch nineties
```

**Step 2**: Create a project with nineties:

```bash
nineties my_project
```

**Step 3**: Go inside your project, seed and and start develop

```bash
cd my_project
cargo run seed
cargo run develop
```

> Note 1: The server will run on `http://localhost:8080`
>
> Note 2: This "develop" command will run server with hot-reload and tailwind bundling.
>
> Note 3: To just run the server, use `cargo run serve`

After seeding, you can login using the credentials:

```
username: jekyll@example.com
password: password
```

That's it, now you can develop.

---

## Frontend

The UI is based in on [Tera](https://keats.github.io/tera/) templating engine, [Tailwind CSS](https://tailwindcss.com/) and [AlpineJS](https://alpinejs.dev/) + [HTMX](https://htmx.org).

### Assets

The UI assets sit in the `resources` folder:

```
├── css
    └── styles.css
├── imgs
    └── nineties-logo.png
├── js
    └── script.js 
└── views
    ├── ...
```

### The public path

The endpoints starting with `/public` are served from the `dist` folder at the root of your project.

### Bundling

When you run `cargon run develop`, you have 2 processes running:

1. The web server
2. The Vite bundling

The vite bundling will watch for changes in the following files and bundle them into `dist/` directory:

- `resources/css/styles.css`
- `resources/js/script.js` <- you can also include your css here
- `resources/imgs/**/*` <- these will be just copied

That is specified in the command `dev` (`npm run dev`) in the `package.json` file.

### Views

The base views folder has the following structure:

```
├── admin <- Behind login wall
    ├── ...
└── ... <- Public views
```

### Components

The components are located in the `parts/components` folder. They are included in the views using the `{% include "parts/components.html" %}` syntax. These components are Tera macros, and one example is the following:

```html
{% macro my_div(label) %}
    <div id="{{ label }}" ...></div>
{% endmacro my_div %}
```

This component can be called like following

```html
{{ my_div("my-div") }}
```

### Notifications

The notifications are session based and are rendered in the `parts/notification.html` file. It is a very basic alpinejs solution where it shows for a short amount of time and then fades out.

## Backend

### Entrypoint

At the `main.rs` file you'll find the main entry point of the application. There we define a few commands that are avalable:

- `serve`: Start the server
- `develop`: Start the server with hot-reload and tailwind bundling
- `seed`: Seed the database with the seeders
- `migrate`: Run the migrations

### Routing

The server routing is defined in the `routes.rs` file. Actix routing points to services. In nineties, each file carrying these services is considered a controllers, and is located in the `http/controllers` folder.

### Database

For database management we use Diesel. Their documentation can be found here: https://diesel.rs/guides/getting-started.html.

The schema is defined in the `schema.rs` file. The migrations are located in the `migrations` folder. The seeders are located in the `database/seeders` folder.

#### Migrations

This command will create migration db table:

```bash
diesel migration generate create_users
```

To run the migrations, you can use the following command:

```bash
cargo run migrate
```

This command will run the migrations and create the database tables you have specified at the `/migrations` folder.

#### Seeders

To run the seeders, you run:

```bash
cargon run seed
```

This command will run the seeders and populate the database with some sample data.

Seeders don't have a command at this moment, but you can create them by creating a new file in the `database/seeders` folder.

Your new seeders must implement the `Seeder` trait.

---

## Features

- [x] MVC structure
- [x] Diesel ORM
- [x] Tera based templates
- [x] Tailwind CSS
- [x] Hot-reload for development
- [x] Seeders
- [x] Auth middleware
- [x] Basic login wall
- [x] Basic admin dashboard
- [x] Basic settings page
- [x] Basic session based notifications
- [x] Basic hero section, footer, header
- [x] Tests
- [x] JS Bundling
- [x] AlpineJS + HTMX
- [x] Basic form validation
- [x] UI Components
- [x] Profile CRUD

## Roadmap

- [ ] WebSockets for realtime interactions
- [ ] Wrap diesel rollback command, and add that to our `main.rs` entrypoint available commands

## Contributing

Feel free to contribute to this project. You can open issues, create pull requests, or just fork it and make your own version.

## License

This project is licensed under the MIT License.

