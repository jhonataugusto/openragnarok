#[cfg(feature = "debug")]
use korangar_debug::logging::print_debug;
use ragnarok_packets::ClientTick;

use super::action::ActionEvent;
use super::entity::EntityType;
use super::weapon_fallback::weapon_fallback;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TargetDamageFeedback {
    Hurt { duration: u32 },
    Miss,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AttackSoundFeedback {
    WeaponFallback,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HitSoundFeedback {
    SoundEffect { path: &'static str },
}

pub fn target_damage_feedback(damage_amount: Option<usize>, damage_duration: u32, target_dead: bool) -> Option<TargetDamageFeedback> {
    if target_dead {
        return None;
    }

    match damage_amount {
        Some(_) => Some(TargetDamageFeedback::Hurt {
            duration: damage_duration.max(1),
        }),
        None => Some(TargetDamageFeedback::Miss),
    }
}

pub fn combat_feedback_due_tick(current_tick: ClientTick, damage_delay: u32) -> ClientTick {
    ClientTick(current_tick.0.wrapping_add(damage_delay.max(1)))
}

pub fn combat_feedback_is_due(current_tick: ClientTick, due_tick: ClientTick) -> bool {
    current_tick.0.wrapping_sub(due_tick.0) < u32::MAX / 2
}

pub fn timed_combat_feedback_is_due(current_tick: ClientTick, fallback_tick: ClientTick, source_still_attacking: bool) -> bool {
    combat_feedback_is_due(current_tick, fallback_tick) && !source_still_attacking
}

pub fn weapon_hit_sound_feedback(
    source_entity_type: EntityType,
    weapon: u32,
    weapon_sprite_name: Option<&str>,
    damage_amount: Option<usize>,
) -> Option<HitSoundFeedback> {
    if source_entity_type != EntityType::Player || damage_amount.is_none() {
        return None;
    }

    Some(HitSoundFeedback::SoundEffect {
        path: weapon_hit_sound_path(weapon, weapon_sprite_name),
    })
}

pub fn attack_sound_feedback(event: ActionEvent, entity_type: EntityType) -> Option<AttackSoundFeedback> {
    match (event, entity_type) {
        (ActionEvent::Attack, EntityType::Player) => Some(AttackSoundFeedback::WeaponFallback),
        (ActionEvent::Sound { .. }, _) | (ActionEvent::Unknown, _) => None,
        (ActionEvent::Attack, _) => None,
    }
}

fn weapon_hit_sound_path(weapon: u32, weapon_sprite_name: Option<&str>) -> &'static str {
    if weapon == 0 {
        return "_hit_fist1.wav";
    }

    let fallback_sound_path = weapon_fallback(weapon).map(|fallback| fallback.hit_sound_path);

    let Some(weapon_sprite_name) = weapon_sprite_name else {
        if let Some(path) = fallback_sound_path {
            return path;
        }

        log_unknown_weapon_hit_sound(weapon, None, "_hit_fist1.wav");
        return "_hit_fist1.wav";
    };

    let lower_case_name = weapon_sprite_name.to_ascii_lowercase();

    if weapon_sprite_name.contains("단검")
        || weapon_sprite_name.contains("카타르")
        || lower_case_name.contains("dagger")
        || lower_case_name.contains("katar")
    {
        return "_hit_dagger.wav";
    }

    if weapon_sprite_name.contains("창") || lower_case_name.contains("spear") {
        return "_hit_spear.wav";
    }

    if weapon_sprite_name.contains("활") || lower_case_name.contains("bow") {
        return "_hit_arrow.wav";
    }

    if weapon_sprite_name.contains("도끼") || lower_case_name.contains("axe") {
        return "_hit_axe.wav";
    }

    if weapon_sprite_name.contains("둔기") || weapon_sprite_name.contains("메이스") || lower_case_name.contains("mace") {
        return "_hit_mace.wav";
    }

    if weapon_sprite_name.contains("롯드")
        || weapon_sprite_name.contains("로드")
        || weapon_sprite_name.contains("지팡이")
        || lower_case_name.contains("rod")
        || lower_case_name.contains("staff")
    {
        return "_hit_rod.wav";
    }

    if weapon_sprite_name.contains("너클") || lower_case_name.contains("knuckle") || lower_case_name.contains("fist") {
        return "_hit_fist1.wav";
    }

    if weapon_sprite_name.contains("검") || lower_case_name.contains("sword") {
        return "_hit_sword.wav";
    }

    if let Some(path) = fallback_sound_path {
        return path;
    }

    log_unknown_weapon_hit_sound(weapon, Some(weapon_sprite_name), "_hit_fist1.wav");
    "_hit_fist1.wav"
}

fn log_unknown_weapon_hit_sound(_weapon: u32, _weapon_sprite_name: Option<&str>, _fallback: &'static str) {
    #[cfg(feature = "debug")]
    print_debug!(
        "[combat-audio] unknown weapon hit sound mapping: weapon_view_id={} weapon_sprite_name={:?} fallback={}",
        _weapon,
        _weapon_sprite_name,
        _fallback
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_damage_requests_hurt_feedback() {
        assert_eq!(
            target_damage_feedback(Some(12), 180, false),
            Some(TargetDamageFeedback::Hurt { duration: 180 })
        );
    }

    #[test]
    fn hit_damage_duration_is_never_zero() {
        assert_eq!(
            target_damage_feedback(Some(12), 0, false),
            Some(TargetDamageFeedback::Hurt { duration: 1 })
        );
    }

    #[test]
    fn miss_damage_requests_miss_feedback_without_hurt() {
        assert_eq!(target_damage_feedback(None, 180, false), Some(TargetDamageFeedback::Miss));
    }

    #[test]
    fn dead_target_ignores_damage_feedback() {
        assert_eq!(target_damage_feedback(Some(12), 180, true), None);
    }

    #[test]
    fn combat_feedback_due_tick_uses_damage_delay() {
        assert_eq!(combat_feedback_due_tick(ClientTick(1000), 240).0, 1240);
    }

    #[test]
    fn combat_feedback_due_tick_is_never_scheduled_in_the_past() {
        assert_eq!(combat_feedback_due_tick(ClientTick(1000), 0).0, 1001);
    }

    #[test]
    fn timed_feedback_waits_for_attack_animation_to_finish() {
        assert!(!timed_combat_feedback_is_due(ClientTick(1240), ClientTick(1240), true));
    }

    #[test]
    fn timed_feedback_falls_back_when_source_is_not_attacking() {
        assert!(timed_combat_feedback_is_due(ClientTick(1240), ClientTick(1240), false));
    }

    #[test]
    fn attack_event_requests_weapon_fallback_sound() {
        assert_eq!(
            attack_sound_feedback(ActionEvent::Attack, EntityType::Player),
            Some(AttackSoundFeedback::WeaponFallback)
        );
    }

    #[test]
    fn monster_attack_event_keeps_using_animation_sound_events() {
        assert_eq!(attack_sound_feedback(ActionEvent::Attack, EntityType::Monster), None);
    }

    #[test]
    fn player_sword_hit_requests_sword_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1, Some("검"), Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_sword.wav" })
        );
    }

    #[test]
    fn player_spear_hit_requests_spear_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1, Some("창"), Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_spear.wav" })
        );
    }

    #[test]
    fn player_dagger_hit_requests_dagger_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1, Some("단검"), Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_dagger.wav" })
        );
    }

    #[test]
    fn player_bow_hit_requests_arrow_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1, Some("활"), Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_arrow.wav" })
        );
    }

    #[test]
    fn player_zero_weapon_hit_requests_fist_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 0, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_fist1.wav" })
        );
    }

    #[test]
    fn known_sword_item_id_without_weapon_table_uses_sword_hit_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1101, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_sword.wav" })
        );
    }

    #[test]
    fn known_dagger_item_id_without_weapon_table_uses_dagger_hit_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1201, Some("1201"), Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_dagger.wav" })
        );
    }

    #[test]
    fn known_weapon_ids_without_weapon_table_use_category_hit_sounds() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1301, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_axe.wav" })
        );
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1401, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_spear.wav" })
        );
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1501, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_mace.wav" })
        );
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1601, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_rod.wav" })
        );
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 1701, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_arrow.wav" })
        );
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 13100, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect {
                path: "_hit_\u{ad8c}\u{cd1d}.wav"
            })
        );
    }

    #[test]
    fn unknown_nonzero_player_weapon_without_name_falls_back_to_fist_impact_sound() {
        assert_eq!(
            weapon_hit_sound_feedback(EntityType::Player, 999999, None, Some(12)),
            Some(HitSoundFeedback::SoundEffect { path: "_hit_fist1.wav" })
        );
    }

    #[test]
    fn miss_does_not_request_weapon_impact_sound() {
        assert_eq!(weapon_hit_sound_feedback(EntityType::Player, 1, Some("검"), None), None);
    }

    #[test]
    fn monster_hit_does_not_request_player_weapon_impact_sound() {
        assert_eq!(weapon_hit_sound_feedback(EntityType::Monster, 1, Some("검"), Some(12)), None);
    }
}
