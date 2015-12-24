# awm - Another Window Manager

Implementing a basic Window Manager in Rust. This is just a fun project for experimenting purposes. There is no guarantee that this won't fuck up your system in any bad way imaginable, as I don't know anything about Window Managers and Rust at all. Have fun!

## Prerequisites

Install **Rust** and the **Cargo** build system:

`sh
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
`

Install the **Xephyr** server as a test XServer for the Window Manager (assuming Arch with **pacman**):

`sh
$ sudo pacman -Su xorg-server-xephyr
`

## How to build

Clone this repository and use cargo to build:

```sh
$ git clone git@github.com:suspectpart/awm 
$ cd awm 
$ cargo build 
```

## How to run: 

- First, we start the Xephyr server with a DISPLAY of **:42**, running as a background job:

```sh
$ Xephyr :42 &  
```

- Then, we run our app, connecting to that display by setting the $DISPLAY environment variable:
- 
```sh
$ DISPLAY=:42 ./target/debug/awm &
```

Right now, all you should see is just some white box somewhere on that screen. Closing the Xephyr server should kill all related jobs.
