use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

//use smashline::Priority::*;


pub fn install() {
    let costume = &mut Vec::new();
    unsafe {
        for i in 0..crate::MARKED_COLORS.len() {
            if crate::MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

    Agent::new("ken")
        .set_costume(costume.to_vec())
        
        
        
        .install();
}
