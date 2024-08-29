# Mix Monkey

Welcome to Mix Monkey, the definitive replacement for Windows audio controls!

## Features

- Beautiful and ergonomic volume controls for Windows applications.
- Replacement for the dry and dated Windows UI.
- Individually control application volumes, i.e. Chrome / Firefox, Discord, Games, etc.
- First class support for hardware controllers including custom keyboards and Arduinos.

## Contribution

Contributions are welcome, create an Issue or open a Merge Request.

### Implementation

Mix Monkey is built with Tauri. It runs a Rust application as a background process which listens for volume change events over a COM port. It processes serial data and interfaces with the Windows API to change program volume status according to user supplied configs. It also controls an overlay window that serves to replace the basic volume change UI Microsoft provides. This window is actually a webpage built and styled with React and Tailwind that is then run in a headless window using the OS provided WebView2 Runtime.

### Developer Environment

    Requirements
      - Rust (1.8+)
      - NodeJS (22.5+)


    Recommendations
      - VSCode
      - RustFMT
