# About this fork
Unfortunately the original author of xidlehook passed away (R.I.P.). There was a small issue bothering me, and there was a PR for that, but it couldn't get merged, so I just created a fork with the changes in that PR. I have no experience with Rust, but since no one is taking over this awesome project I decided to do it anyway. Perhaps I will spend some of my free time looking into this project, perhaps not. With that being said, please feel free to open issues or PRs here and I will try my best to investigate them if I had time.

xidlehook is a very useful piece of software and I have been using it for a long time. Thanks jD91mZM2 for making it. May God be with you in Heaven.

-   [xidlehook](#xidlehook)
    -   [Example](#example)
    -   [Installation](#installation)
        -   [Other installation methods](#other-installation-methods)
    -   [Socket API](#socket-api)
    -   [Caffeinate](#caffeinate)
    -   [Configuring via `systemd`](#configuring-via-systemd)
    -   [Troubleshooting](#troubleshooting)

# xidlehook

*Because xautolock is annoying to work with.*

xidlehook is a general-purpose replacement for
[xautolock](https://linux.die.net/man/1/xautolock). It executes a
command when the computer has been idle for a specified amount of time.

**Improvements over xautolock:**

-   Allows "cancellers" which can undo a timer action when new user
    activity is detected.
-   Unlimited amount of timers (provided necessary resources).
-   Not specific to locking.
-   Multiple instances can run at the same time.
-   Optionally only run through chain once.
-   Optionally prevent locking when an application is fullscreen.
-   Optionally prevent locking when any application plays audio.

**Missing features:**

-   Magic corners
-   All the instance related stuff (you should use unix sockets with
    –socket).

## Example

Here's a lock using i3lock, with screen dim support:

``` bash
#!/usr/bin/env bash

# Only exported variables can be used within the timer's command.
export PRIMARY_DISPLAY="$(xrandr | awk '/ primary/{print $1}')"

# Run xidlehook
xidlehook \
  `# Don't lock when there's a fullscreen application` \
  --not-when-fullscreen \
  `# Don't lock when there's audio playing` \
  --not-when-audio \
  `# Dim the screen after 60 seconds, undim if user becomes active` \
  --timer 60 \
    'xrandr --output "$PRIMARY_DISPLAY" --brightness .1' \
    'xrandr --output "$PRIMARY_DISPLAY" --brightness 1' \
  `# Undim & lock after 10 more seconds` \
  --timer 10 \
    'xrandr --output "$PRIMARY_DISPLAY" --brightness 1; i3lock' \
    '' \
  `# Finally, suspend an hour after it locks` \
  --timer 3600 \
    'systemctl suspend' \
    ''
```

*Note: Every command is passed through `sh -c`, so you should be able to
mostly use normal syntax.*

## Installation

*As of currently, you will need to use the Rust 1.39.0 higher when
building xidlehook.*

Recommended installation is through the [Nix package
manager](https://nixos.org/nix/), which will get you a sane default
configuration of xidlehook as well as all required libraries.

``` bash
nix-env -iA nixpkgs.xidlehook
```

will install xidlehook regardless of whether you have rust installed,
regardless of whether you have libxcb and friends installed, whatever.
Nix just works.

If you instead would like to use the latest master, you can install it
using the following.

``` bash
nix-env -if https://gitlab.com/jD91mZM2/xidlehook/-/archive/master.tar.gz
```

### Other installation methods

While I do definitely encourage you to try Nix if you haven't already,
there are other ways to install xidlehook, of course. But it will
involve some system-specific trouble I can't really help you with.

Arch Linux users can avoid that, however, thanks to an [unofficial AUR
package](https://aur.archlinux.org/packages/xidlehook/)!

Xidlehook with the default settings requires **libxcb**,
**libXScrnSaver** (or libxss) and **libpulseaudio**. On debian/ubuntu,
don't forget to install the `-dev` versions of all the mentioned
dependencies, also.

| Which feature flag?                  | Native dependency                  |
|--------------------------------------|------------------------------------|
| Always                               | libxcb, libXScrnSaver (aka libxss) |
| When using –features pulse (default) | libpulseaudio                      |

After getting these native libraries, one way of installing is with
cargo, the official rust package manager that works almost everywhere
with rust installed.

``` bash
cargo install xidlehook --bins
```

Or if you want to clone it manually:

``` bash
git clone https://gitlab.com/jD91mZM2/xidlehook
cd xidlehook
cargo build --release --bins
```

## Socket API

The socket API can be communicated with over JSON. The full data and
types for these structures can be seen in all the struct definitions of
`xidlehook/src/socket/models.rs`.

For convenience, there is now an xidlehook-client (see
[\#18](https://github.com/jD91mZM2/xidlehook/pull/18)), which will
communicate with this API for you. See

``` bash
xidlehook-client --help
```

for details.

A common use case of `xidlehook` is using it to run a lockscreen. To
then manually lock the screen, you could first decide what ID the timer
has, either by counting the indexes yourself of the timers you inform
xidlehook of (starting from 0), or by querying timer information after
starting it:

``` bash
# Check what timer(s) you want to trigger...
xidlehook-client --socket /path/to/xidlehook.sock query
```

And then bind a hotkey or bash alias to lock it:

``` bash
# Trigger it/them
xidlehook-client --socket /path/to/xidlehook.sock control --action trigger --timer <my timer id>
```

## Caffeinate

If you're looking for a more elaborate client to temporarily disable
`xidlehook`, take a look at
[caffeinate](https://github.com/rschmukler/caffeinate) which has timers
and PID based monitoring.

## Configuring via `systemd`

If you use a distribution that uses `systemd` for its init system, you may wish
to use it to ensure that `xidlehook` is always started when you log in. You can
do this at the level of an individual user by placing the following service
file at `$XDG_CONFIG_HOME/systemd/user/xidlehook.service` (typically
`$HOME/.config/systemd/user/xidlehook.service`):

``` systemd
[Unit]
Description=Automatic Screen Locker

[Service]
Type=simple
Environment=DISPLAY=:0
Environment=XIDLEHOOK_SOCK=%t/xidlehook.socket
ExecStart=/usr/bin/xidlehook --not-when-audio --not-when-fullscreen --socket $XIDLEHOOK_SOCK --timer 900 '/usr/bin/slock' ''

[Install]
WantedBy=multi-user.target
```

The above service file example locks the screen using the `slock` program after
15 minutes of inactivity, but you can edit it  the `xidlehook` command to do
anything you wish. We need to thread the `DISPLAY` environment variable down so
that `xidlehook` knows how to open up a connection to the X server.

## Troubleshooting

If you have `redshift` running, the brightness of your screen will be
quickly overriden by `redshift`. You can specify the brightness of the
screen via `redshift` instead of `xrandr` to fix this issue.
