
<h1 align="center">Nineties
<img src="docs/imgs/nineties-logo.png" alt="Nineties - Web App" style="width: 44px; height: 44px;" width="44" height="44" /></h1>

[![Build and Test](https://github.com/lotharthesavior/nineties/actions/workflows/tests.yml/badge.svg)](https://github.com/lotharthesavior/nineties/actions/workflows/tests.yml)

This is a starter for web with rust on top of [Actix](https://actix.rs).

Spend time with **your ideas** on top of a solid foundation.

## Dependencies

At this point this is tested in Ubuntu, and it requires the following:

- build-essential (available in apt)
- libssl-dev (available in apt)
- libsqlite3-dev (available in apt)
- Run `cargo install cargo-watch`

## Quick Start

Just clone this repo start coding your web app:

```bash
git clone https://github.com/lotharthesavior/nineties.git my_project
cd my_project
cargo run migrate
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

### JWT Bearer Authentication (API Alternative)

JWT Bearer tokens provide stateless auth for separate frontend/API clients alongside session cookies for HTML.

**Setup:** Copy `.env.example` to `.env`, set `JWT_SECRET` (min 32 chars) and `JWT_EXPIRY_HOURS` (default 24).

**Endpoints:**
- `POST /api/login` body: `{"email": "jekyll@example.com", "password": "password"}` → `{"token": "..."}`
- `GET /api/protected/profile` header: `Authorization: Bearer <token>` → user JSON (password omitted)

**Curl Login & Profile:**
```bash
# Login
curl -X POST http://localhost:8080/api/login \
-H "Content-Type: application/json" \
-d '{"email":"jekyll@example.com","password":"password"}'

# Profile (manual token)
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/api/protected/profile
```

**Bash Validation Script** (`test-jwt.sh`):
```bash
#!/bin/bash
BASE_URL="http://localhost:8080"
echo "=== JWT Auth Test ==="

# Login
RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/api/login" \
-H "Content-Type: application/json" \
-d '{"email":"jekyll@example.com","password":"password"}')
CODE=$(echo "$RESP" | tail -1)
BODY=$(echo "$RESP" | sed '$d')
echo "$BODY (Status: $CODE)"

[ "$CODE" = "200" ] || { echo "Login failed"; exit 1; }
TOKEN=$(echo "$BODY" | jq -r '.token')
echo "Token: $TOKEN"

# Profile
RESP=$(curl -s -w "\n%{http_code}" -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/protected/profile")
echo "$RESP"
```

Run: `chmod +x test-jwt.sh; make db-setup; cargo run serve &; sleep 5; ./test-jwt.sh`

**Add New Endpoint:**
1. Add in `src/http/controllers/api_controller.rs`: `#[get("/new")] pub async fn new_endpoint(req: HttpRequest) -> ... { ... }`
2. In `src/routes.rs` `config()`: add `.service(new_endpoint)` under `/api` (public) or `/protected` (JWT).
3. `cargo check && cargo clippy`
4. Test: `curl ... /api/new` or with Bearer for protected.

---

## Nineties binary

To build nineties binary, the following steps are necessary:

**Step 1**:

```bash
source ./prepare-environment.sh
```

**Step 2**:

```bash
# to create a new project from here
cargo run my_project
# to prepare the binary
cargon build --release
```

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

- [ ] Add registration page
- [ ] WebSockets for realtime interactions
- [ ] Wrap diesel rollback command, and add that to our `main.rs` entrypoint available commands

## Contributing

Feel free to contribute to this project. You can open issues, create pull requests, or just fork it and make your own version.

## License

This project is licensed under the MIT License.

