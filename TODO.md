- Handle 'other' option
- Syncing state between arduino and changes made by media keys or win system settings
- App icons in UI
- UI animation
- Should volume up auto disable mute like it does with win? Need to back propagate value to arduino to make work if so.
- Tauri 2.0
- Make work on lock screen?
- Create tray menu
  - Debug console
  - Edit config UI
  - Exit
  - List com ports, switch
  - Open file location / config, etc.

Bugs

- Element focus outlines
  - Need to drop / reset window focus on close hook
- First run slider broken / no volume set
- Crashes when try click UI, used to work.
