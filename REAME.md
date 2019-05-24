# Dashbutton

A simple Rust project to detect button pushes from an Amazon Dash button. Work
in progress.

This project is mostly just to practice using Rust. If you want a serious
project to use with your Dash button, you should probably check out something like:

- https://github.com/Nekmo/amazon-dash
- https://github.com/mikeflynn/go-dash-button
- https://github.com/chrisgilbert/godash

## Installation

### If you have Rust / cargo:

```console
$ git clone https://github.com/n8henrie/dashbutton
$ cd dashbutton
$ make
$ # Optionally, if on Linux with libcap installed:
$ make rootless
```

## Usage

Edit the included `config-sample.toml` to include your Dash button's MAC
address and optionally the network device that will be detecting the pushes
(e.g. `wlan0`).

## Finding mac addresses

After you've connected your Dash button to your network (but made sure *not* to
select a product, so it's not purchasing anything), you should probably be able
to find its mac address by searching the output of `sudo tcpdump 'arp'` after a
button press; the Dash button may show up as `(oui Unknown)`.
