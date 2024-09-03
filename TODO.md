# Mix Monkey: In the Pipeline

### New Features

- [ ] Propagate state back to arduino
  - [ ] Volume
  - [ ] Mute
  - [ ] Initialize at program start, use winAPI as source of truth
- [ ] Config
  - [x] Allow user to set in configs
  - [x] Map to all other programs
  - [x] How to handle mute?
  - [ ] Support multiple inputs for a single encoder
  - [ ] What to show in UI
  - [ ] System Sounds ?
- [ ] App Icons
  - [ ] Figure out how resolve app specific icons
  - [ ] Add to UI
- [ ] User config / settings
  - [ ] Option in tray menu
  - [ ] Options in config.yaml
  - [ ] Read write / update config.yaml based on tray menu choices - just open an editor? reload on close?
  - [ ] Feature flag support
    - [ ] Theme/styling
    - [ ] App icons on / off
    - [ ] Timeout for UI visible
- [ ] Tray Menu
  - [ ] List com ports
  - [ ] switch com
  - [ ] restart
  - [ ] view raw arduino logs
- [ ] Volume up should unmute
- [ ] UI Improvements
  - [ ] Animations?
- [ ] Upgrade to Tauri 2.0
- [ ] Lock screen support
- [ ] Crash recovery / user alerts
- [ ] GitHub
  - [ ] License
  - [ ] Workflow to build release
  - [ ] Versioning
  - [ ] Documentations
  - [ ] Auto update - ship new versions / prompt to users when release is published

### Bugs

- Element focus outlines
- App taking focus blocks media key event listeners
  - Preventing / reseting focus on close should avoid
- First run slider broken / no volume set
- Crashes when try click UI, used to work.
- Don't unmute when lowering volume

### Completed ✓

- [x] A lot of stuff!
