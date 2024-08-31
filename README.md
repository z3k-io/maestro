# Mix Monkey

Welcome to Mix Monkey, the definitive replacement for Windows audio controls!

## Features

- Beautiful and ergonomic volume controls for Windows applications.
- Replacement for the dry and dated Windows UI.
- Individually control application volumes, i.e. Chrome / Firefox, Discord, Games, etc.
- First class support for hardware controllers including custom keyboards and Arduinos.

## Setup

Build yourself an audio box with rotary encoders [TK]  
Download and flash the [Arduino Sketch](/arduino/sketch/sketch.ino) to your microcontroller.  
Download and install the latest release  
Download config file [config.yaml](/src-tauri/config.yaml) and modify for your usecase.  
Run the app and turn the dials.

## Contribution

Contributions are welcome, create an Issue or open a Merge Request.

## Acknoledgements

This App took a lot of inspiration for the excellent project [Deej](https://github.com/omriharel) by Omri Harel. For 3D printable designs, and more insights, check out their repo and related searches on 3D model sites.

## License

TK

### Implementation

Mix Monkey is built with [Tauri](https://tauri.app/). It runs a Rust application as a background process which listens for volume change events over a COM port. It processes serial data and interfaces with the Windows API to change program volume status according to user supplied configs. It also controls an overlay window that serves to replace the basic volume change UI Microsoft provides. This window is actually a webpage built and styled with [React](https://react.dev/) and [Tailwind](https://tailwindcss.com/) that is then run in a headless window using the OS provided [WebView2 Runtime](https://learn.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH).

### Developer Environment

    Requirements
      - Rust (1.8+)
      - NodeJS (22.5+)


    Recommendations
      - VSCode
      - RustFMT
      - Bun

    To run in dev mode, simply run `bun start` from the project root.
