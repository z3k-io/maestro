# Mix Monkey: In the Pipeline

### New Features

- [ ] Propagate state back to arduino
  - [ ] Volume
  - [ ] Mute
  - [ ] Initialize at program start, use winAPI as source of truth
- [ ] Config
  - [ ] Support multiple inputs for a single encoder
  - [ ] What to show in UI
- [ ] App Icons
  - [ ] Figure out how resolve app specific icons
  - [ ] Add to UI
- [ ] User config / settings
  - [ ] Option in tray menu
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
- [ ] Handle COM port changes / disconnects / reconnects
- [ ] Auto update - ship new versions / prompt to users when release is published

### Bugs

- Element focus outlines
- App taking focus blocks media key event listeners
  - Preventing / reseting focus on close should avoid
- First run slider broken / no volume set
- Crashes when try click UI, used to work.
- config.yaml is being bundle with / not overwritten or honored
  - AppData\Local\mix-monkey

### Completed ✓

- [x] A lot of stuff!
