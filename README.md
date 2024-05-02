Garnet DeGelder's ScoreKeeper 2
===============================

Requirements
------------

### Runtime

#### Client
- A recent web browser (Should work in Chrome, Edge, Firefox, Safari, and any other modern browser released in the past year or two)
	- Primarily developed with Firefox

#### Server
- Should run on pretty much anything it compiles on.
- Uses a SQLite3 database (by default `./database.sqlite3`)

### Compilation

#### Server
- Rust (developed with Rust 1.77.2, latest specified MSRV for dependencies is currently 1.73 but may change)

#### Client
- NodeJS (developed with NodeJS 21, other recent versions will probably work)


Compilation
-----------

### Clone the repo

- `git clone 'https://github.com/g-dg/scorekeeper2.git'`
- `cd scorekeeper2`

#### Server

- `cargo build --release`
Server executable is `target/release/scorekeeper` (`scorekeeper.exe` if on Windows)

#### Client

- `cd client`
- `npm install`
- `npm run build`
- `cd ..`
Client static files are in `client/dist/`


Installation
------------

- Copy the server executable
- Copy all the client static files to a directory `./client/dist/` relative to the working directory of the server (configurable with `static_file_root` key in `config.json`)


Running
-------

- Change to the project root (where the client files are accessible in `./client/dist/`)
- The config file `config.json` in the current working directory (if not present, a default configuration will be used).
- Run the executable
	- This will make a SQLite3 database called `database.sqlite3` in the current working directory (configurable with `database_file` key in `config.json`)
- The server and client are accessible at address `0.0.0.0`, port `8080` by default (configurable with `host`, and `port` in `config.json`)
- Default admin credentials are username: `admin`, password: `admin` (should be changed after setup, can be configured with `default_admin_username` and `default_admin_password` config options)
