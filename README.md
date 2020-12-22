# Battilde

[Battilde client](https://github.com/jmdejong/battilde-client)

A multiplayer top-down terminal shooter.

## About Battilde

Battilde is a multiplayer co-op shooter game that is played in the terminal.

The intended use is to play this servers with a shared login (through ssh) but it can be played in other contexts too.

Players fight of progressively stronger waves of monsters.

You start in the sanctuary, an area that heals you.
If your health is full you can leave the sanctuary.
You can not come back through the gates after leaving.
If you die you respawn in the sanctuary again.

There are 4 several pillars around the sanctuary.
Some monsters will attack these.
The game ends when all pillars are destroyed.


## Installation/Running

Installation has been tested and confirmed to work on Linux.
If anyone is able to run this on other operating systems, please tell me about the results.

Install Rust and Cargo: https://www.rust-lang.org/tools/install

Run the command `cargo run` to compile and run battilde with all the default options.

Run the command `cargo build --release` to compile battilde and to create a binary which can then be run without cargo.
The binary will appear in the `target/release` directory, with the name `battilde`.

This repo only contains the server.
To play the game you need [the client too](https://github.com/jmdejong/battilde-client)

## Command line arguments

To see all command line arguments, pass the argument `--help`:

	$ ./battilde --help
	Battilde 0.2.0
	Multiplayer terminal shooter (server)

	USAGE:
		battilde [OPTIONS] --admins <admins>

	FLAGS:
		-h, --help       Prints help information
		-V, --version    Prints version information

	OPTIONS:
		-a, --address <address>...             A server type and address. Allowed server types: 'inet', 'unix', 'abstract'.
											Example: "inet:127.0.0.1:1234" or "abstract:battilde" or "unix:/tmp/battilde"
											or "inet:[::1]:1234"
			--admins <admins>                  The name(s) of the server admin(s) [env: USER=troido]
			--custom-map <custom-map>          File path for a custom map to play
			--game-mode <game-mode>            The gamemode of the server. Options: coop, pvp [default: coop]
			--map <map>                        The built-in map to play. Ignored if --custom-map is used. [default: square]
			--step-duration <step-duration>    The time (in milliseconds) between two steps [default: 100]




