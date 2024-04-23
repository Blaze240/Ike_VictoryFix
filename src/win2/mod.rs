use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
    },
    smash_script::*,
    smashline::*
};

unsafe extern "C" fn ike_sound_win2(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win02"));
}
}
    else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win02"));
        }
                      }
                     }
pub fn install() {
    Agent::new("ike")
    .sound_acmd("sound_win2",ike_sound_win2)
    .install();
}

