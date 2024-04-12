Garnet DeGelder's ScoreKeeper 2
===============================

Requirements
------------

### Runtime

#### Client
- A recent web browser (Should work in Chrome, Edge, Firefox, Safari, and any other modern browser released in the past year or 2)
	- Primarily developed with Firefox

#### Server
- Should run on pretty much anything it compiles on.
- Uses a SQLite3 database (currently hardcoded to `./database.sqlite3`)

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
- Copy all the client static files to a directory `./client/dist/` relative to the working directory of the server (not yet configurable)


Running
-------

- Change to the project root (where the client files are accessible in `./client/dist/`)
- Run the executable
	- This will make a SQLite3 database called `database.sqlite3` in the current working directory (not yet configurable)
- The server and client are accessible at port `8080` (not yet configurable)
