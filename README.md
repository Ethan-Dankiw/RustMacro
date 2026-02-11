## THIS IS A PROOF OF CONCEPT PROJECT TO LEARN RUST

The code is probably shocking, and non-standard for a production grade codebase.
I do not intend for this to work on a machine other than mine, so I cannot provide any guarantee on the safety,
security, reliability or compatability that the code works on your machine as expected and does not break anything
if downloaded and executed.

# System Requirements

APP may require Arch based OS and KDE Plasma desktop environment, however this is unverified.

# Design Goal

I wanted an AHK like application that I could use to program macros. However, on Linux and more-so on Wayland, 
there was no existing software that I could find. So I decided to create my own.

The macro's that I currently use were originally designed for Stardew Valley, primarily animation cancelling and were 
expended over time. The programmed macros live in `src/macro/scripts` with examples of how to interact with the
Mouse and Keyboard.

Because of the limitations of Wayland. Sending mouse movements and keyboard inputs directly to existing hardware was 
not possible. Thus, a virtual keyboard and mouse that Linux treats as physical hardware is used to execute instructions.

# Mouse and Keyboard Setup

The application requires the manual selection of inputs for the mouse and keyboard. 
These values will be prompted on first launch and saved to a `config.toml` file in the current working directory.

Where this config file is located depends on if being run inside your IDE or in the terminal.

### Getting the input event value for the mouse

One of the following two methods can be executed in your terminal to find the event values of your mouse.

1. `make list_mouse`
2. `ls -l /dev/input/by-id/ | grep "event-mouse`

**Sample Output**

```
❯ make list_mouse
ls -l /dev/input/by-id/ | grep "event-mouse"
lrwxrwxrwx 1 root root  9 Feb  9 13:16 usb-ROYUAN_EPOMAKER_TH80_Pro-XXXX-event-mouse -> ../event6
lrwxrwxrwx 1 root root  9 Feb  9 13:16 usb-Swiftpoint_Limited_Z_2_XXXXXXXXX-event-mouse -> ../event9
```

IMPORTANT NOTE: If you are using your mouse to detected key presses, e.g. F1 to F24 to activate a macro. 
Then you MUST use the event value for your mouse, but the keyboard entry. 
See the section on getting the input event value for a keyboard.


Otherwise, since I know that `usb-Swiftpoint_Limited_Z_2_XXXXXXXXX-event-mouse` is my mouse, and I want to detect mouse clicks e.g. Left Click or mouse movement.
I know that when prompted, I would enter `event9` as the value for mouse input.

### Getting the input event value for the keyboard

One of the following two methods can be executed in your terminal to find the event values of your keyboard.

1. `make list_keyboard`
2. `ls -l /dev/input/by-id/ | grep "event-kbd`

**Sample Output**

```
❯ make list_keyboard
ls -l /dev/input/by-id/ | grep "event-kbd"
lrwxrwxrwx 1 root root  9 Feb  9 13:16 usb-ROYUAN_EPOMAKER_TH80_Pro-event-kbd -> ../event2
lrwxrwxrwx 1 root root 10 Feb  9 13:16 usb-Swiftpoint_Limited_Z_2_XXXXXXXXX-XXXX-event-kbd -> ../event10
```

Since I know that `usb-ROYUAN_EPOMAKER_TH80_Pro-event-kbd` is my keyboard, I know that when prompted,
I would enter `event2` as the value for keyboard input.

Since I know that `usb-Swiftpoint_Limited_Z_2_XXXXXXXXX-XXXX-event-kbd` is my mouse, If I want to detect key presses on my mouse.
I know that when prompted, I would enter `event10` as the value for MOUSE input.

## What creating a new config file should look like

When running the executable, the first block of entries in the terminal should have the following format

```
Attempting to load config.toml
Looking for config.toml at the path: /YOUR/CURRENT/WORKING/DIRECTORY/config.toml
No config.toml file detected, creating one...
Mouse Input (e.g. event9): event9
Keyboard Input (e.g. event2): event2
Config file created: /YOUR/CURRENT/WORKING/DIRECTORY/config.toml
Detected Config Entries:
 - Mouse Input: event9
 - Keyboard Input: event2
```

## What loading an existing config file should look like

When running the executable, the first block of entries in the terminal should have the following format

NOTE: This terminal output was run AFTER the above step of creating a new config.

```
Attempting to load config.toml
Looking for config.toml at the path: /YOUR/CURRENT/WORKING/DIRECTORY/config.toml
Detected config.toml file, loading from disk...
Detected Config Entries:
 - Mouse Input: event9
 - Keyboard Input: event2
```
