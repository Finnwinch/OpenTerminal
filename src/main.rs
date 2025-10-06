#![windows_subsystem = "windows"]
use winapi::um::winuser::{
    RegisterHotKey, GetMessageW, TranslateMessage, DispatchMessageW,
    MOD_ALT, MOD_CONTROL, WM_HOTKEY, MSG,
};
use std::ptr;
use std::mem;
use std::process::Command;

const VK_T: u32 = 0x54;

fn main() {
    unsafe {
        let hotkey_id: i32 = 0xBFFF;
        let modifiers = MOD_CONTROL | MOD_ALT;

        if RegisterHotKey(ptr::null_mut(), hotkey_id, modifiers.try_into().unwrap(), VK_T) != 0 {

            let mut msg: MSG = mem::zeroed();

            loop {
                if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
                    if msg.message == WM_HOTKEY && msg.wParam == hotkey_id as usize {
                        println!("Raccourci détecté, ouverture du terminal...");

                        Command::new("wt.exe")
                            .spawn()
                            .expect("Échec du lancement du programme");
                    }

                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
}