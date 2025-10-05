use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc, RwLock,
};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyNameTextW, MapVirtualKeyW, MAPVK_VK_TO_VSC};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExA, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL,
    WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use super::key::KeyChord;

type Callback = Arc<dyn Fn() + Send + Sync + 'static>;

// TODO: Focus loss when window open. Only for the tauri windows though. Other windows are fine.
struct HookData {
    chord: KeyChord,
    callback: Callback,
    should_block: bool,
}

impl HookData {
    fn new(chord: KeyChord, should_block: bool, callback: Callback) -> Self {
        Self {
            chord,
            should_block,
            callback,
        }
    }
}

struct SendableHHOOK(AtomicPtr<std::ffi::c_void>);

unsafe impl Send for SendableHHOOK {}
unsafe impl Sync for SendableHHOOK {}

impl SendableHHOOK {
    fn new(hook: HHOOK) -> Self {
        Self(AtomicPtr::new(hook.0))
    }

    fn get(&self) -> HHOOK {
        HHOOK(self.0.load(Ordering::Relaxed))
    }

    fn set(&self, hook: HHOOK) {
        self.0.store(hook.0, Ordering::Relaxed)
    }
}

lazy_static! {
    static ref HOOKS: Arc<RwLock<Vec<HookData>>> = Arc::new(RwLock::new(Vec::new()));
    static ref PRESSED_KEYS: Arc<RwLock<HashMap<u32, bool>>> = Arc::new(RwLock::new(HashMap::new()));
}

fn key_code_to_string(vk_code: u32) -> String {
    unsafe {
        let scan_code = MapVirtualKeyW(vk_code, MAPVK_VK_TO_VSC);
        let mut buffer = [0u16; 32];
        let len = GetKeyNameTextW((scan_code << 16) as i32, &mut buffer);
        String::from_utf16_lossy(&buffer[..len as usize])
    }
}

// Global hook, fires for any key press event
unsafe extern "system" fn keyboard_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    log::trace!("Entering hook, {} hooks", HOOKS.read().unwrap().len());

    if code == HC_ACTION as i32 {
        let kb_struct = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;
        let key_string = key_code_to_string(vk_code);

        // Key down event
        if w_param.0 as u32 == WM_KEYDOWN || w_param.0 as u32 == WM_SYSKEYDOWN {
            log::trace!("KEYDOWN: {} ({})", key_string, vk_code);

            PRESSED_KEYS.write().unwrap().insert(vk_code, true);

            log::trace!("Pressed keys: {:?}", PRESSED_KEYS.read().unwrap().keys());
        }

        // Key up event
        if w_param.0 as u32 == WM_KEYUP || w_param.0 as u32 == WM_SYSKEYUP {
            log::trace!("KEYUP: {} ({})", key_string, vk_code);

            PRESSED_KEYS.write().unwrap().remove(&vk_code);

            log::trace!("Pressed keys: {:?}", PRESSED_KEYS.read().unwrap().keys());
        }

        let mut should_block_propagation = false;

        let hooks = HOOKS.read().unwrap();
        for hook in hooks.iter() {
            let pressed_keys = PRESSED_KEYS.read().unwrap().keys().cloned().collect::<Vec<u32>>();
            if hook.chord.is_pressed(&pressed_keys) {
                log::trace!("Triggering callback for chord: {:?}", hook.chord.to_string());
                (hook.callback)();

                if hook.should_block {
                    should_block_propagation = true;
                }
            }
        }

        if should_block_propagation {
            log::trace!("Blocking event propagation");
            return LRESULT(1);
        }
    }

    // Always call the next hook in the chain, regardless of whether we've handled the event
    CallNextHookEx(None, code, w_param, l_param)
}

pub struct KeyListener {
    hook: SendableHHOOK,
}

impl KeyListener {
    pub fn new() -> Self {
        let hook = unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0) }.expect("Failed to set hook");

        KeyListener {
            hook: SendableHHOOK::new(hook),
        }
    }

    pub fn init(&self) {
        let h_instance = unsafe { GetModuleHandleW(None) }.expect("Failed to get module handle");
        let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0) }.expect("Failed to set hook");
        self.hook.set(hook);
    }

    pub fn register(&self, key_chord: KeyChord, should_block: bool, callback: Callback) {
        let hook_data = HookData::new(key_chord, should_block, callback);
        HOOKS.write().unwrap().push(hook_data);
    }

    pub fn unregister_all(&self) {
        HOOKS.write().unwrap().clear();
    }

    pub fn dispose(&self) {
        unsafe {
            self.unregister_all();
            UnhookWindowsHookEx(self.hook.get()).expect("Failed to unregister hook.");
        }
    }
}
