use std::mem;
use std::panic;
use windows::Win32::{
    Foundation::*,
    System::{
        Console::AllocConsole,
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::*,
        Threading::{CreateThread, THREAD_CREATION_FLAGS},
    },
};

fn compose_entity_flags(god_mode: bool, demigod_mode: bool, no_target: bool) -> u8
{
    let mut result = 0u8;
    if god_mode
    {
        result |= 1;
    }
    if demigod_mode
    {
        result |= 2;
    }
    if no_target
    {
        result |= 4;
    }
    result
}

unsafe extern "system" fn thread_function(_: *mut std::ffi::c_void) -> u32
{
    println!("In the thread.");
    let module_base: *const u8 = 0x400000 as *const u8;

    loop
    {
        let result = panic::catch_unwind(|| unsafe {
            let specialops_dvar = *(module_base.offset(0x1A06E84) as *const u32); // 01C4F0C8
            let so_survival_dvar = *(module_base.offset(0x1A06E94) as *const u32);

            if specialops_dvar != 0
            {
                let value = *(specialops_dvar.wrapping_add(0xC) as *const u8);

                if value == 1
                {
                    // println!("We are in Spec Ops!");
                }
            }
            if so_survival_dvar != 0
            {
                let value = *(so_survival_dvar.wrapping_add(0xC) as *const u8);

                if value == 1
                {
                    // println!("We are in Spec Ops Survival!");

                    // Make the player's riot-shield unbreakable (just like IW4 SP)
                    // +F8CE00, default = 20000
                    if *(module_base.offset(0xF8CE00) as *mut u32) == 20_000
                    {
                        *(module_base.offset(0xF8CE00) as *mut u32) = mem::transmute(20_000u32);
                    }

                    // 0x74 bytes per each entity, loop 32 or 48 times, check their tags,
                    // Entity 1: F563E4
                    // Entity 2: F56458
                    // Entity 3: F564CC
                    // Entity 4: F56540
                    let entity_base = module_base.offset(0xF563E4) as *const u8;

                    for i in 0..32
                    {
                        let current_entity_addr = entity_base.wrapping_add(i * 0x74usize) as *const u8;
                        let current_entity = *(current_entity_addr as *const u32);

                        if current_entity != 0 && current_entity > 0x100_0000 && current_entity < 0x300_0000
                        {
                            println!("valid {i} {:x}", current_entity - 0x40_0000);
                            match *(current_entity.wrapping_add(0x70) as *const u32)
                            {
                                0x201 =>
                                {
                                    // Delta team
                                    println!("Delta team {i} => {current_entity:x}!");

                                    *(current_entity.wrapping_add(0x13C) as *mut u8) =
                                        mem::transmute(compose_entity_flags(true, false, false));

                                    // laser?
                                    *(current_entity.wrapping_add(0xF) as *mut u8) = 12;

                                    // 2B48 for spawn protection (maybe it's part of script supplied data?)
                                    //*(current_entity.wrapping_add(0x2B48) as *mut u8) = 1;

                                    // 54: Chance of getting hit by bullets (f32)
                                    // 2BD4: Grenade count
                                    //
                                }
                                0x202 =>
                                {
                                    // GIGN team
                                    // println!("GIGN team!");

                                    *(current_entity.wrapping_add(0x13C) as *mut u8) =
                                        mem::transmute(compose_entity_flags(true, false, true));
                                }
                                0x100_0000 =>
                                {
                                    // Turret
                                    println!("Turret!");

                                    // TODO(iFarbod): Add TOML options
                                    *(current_entity.wrapping_add(0x13C) as *mut u8) =
                                        mem::transmute(compose_entity_flags(true, false, true));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        });

        match result
        {
            Ok(()) =>
            {}
            Err(_) =>
            {
                println!("An error occurred while reading the value");
            }
        }
    }

    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(dll_module: windows::Win32::Foundation::HMODULE, call_reason: u32, _: *mut ()) -> BOOL
{
    unsafe {
        DisableThreadLibraryCalls(dll_module);
    }
    match call_reason
    {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true.into()
}

fn attach()
{
    unsafe { AllocConsole() };
    println!("Attached.");

    let thread = unsafe {
        CreateThread(
            None,
            0 as usize,
            Some(thread_function),
            Some(std::ptr::null_mut()),
            THREAD_CREATION_FLAGS(0),
            None,
        )
    };

    match thread
    {
        Ok(_handle) =>
        {
            println!("Created thread");
        }
        Err(e) =>
        {
            panic!("Unable to create thread {e:?}");
        }
    }
}

#[no_mangle]
pub extern "C" fn lib_test()
{
    println!("Hello from the library!");
}
