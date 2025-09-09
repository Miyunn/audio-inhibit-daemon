# Audio Inhibit Daemon (Rust)

Minimal Rust daemon that **prevents system sleep while audio is playing**.

## WHY

My system would go to sleep whenever there’s no keyboard or mouse activity, which makes sense, of course. However, Discord and Spotify (spotify-launcher) don’t prevent the system from sleeping while in use. I also don’t want to disable sleep entirely, so this daemon keeps the system awake **when audio is playing**.

## How

1. Polls `pactl list short sink-inputs` every second.

   * PipeWire provides `pactl` compatibility via `pipewire-pulse`.
2. Checks each sink input for the line:

   * Only considers audio **actively playing** if `Corked: no` is present.
3. When **any active audio is detected**:

   * Spawns:

     ```bash
     systemd-inhibit --what=sleep --why="Audio playing" --mode=block sleep infinity
     ```
   * Keeps the inhibitor process running to prevent sleep.
4. When **no active audio is detected** (all corked or none exist):

   * Kills the inhibitor child to release the sleep block.

## Install

Run the provided `install.sh` script:

```bash
./install.sh
```

This script will:

* Check if Rust is installed.
* Build the release binary.
* Copy the binary to `~/.local/bin`.
* Install and enable the systemd user service (`audio-inhibit-daemon.service`).

## Uninstall

Run the provided `uninstall.sh` script:

```bash
./uninstall.sh
```

This script will:

* Stop and disable the systemd user service.
* Remove the binary from `~/.local/bin`.

## Please Note

* This daemon is made for **my use**. I haven't tested it elsewhere, so use at your own risk.
* This is one of those projects where I wish I knew more to do it differently, definitely open to changes.
