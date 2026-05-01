#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct WeaponFallback {
    pub(crate) sprite_name: &'static str,
    pub(crate) hit_sound_path: &'static str,
    pub(crate) attack_action: WeaponAttackAction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum WeaponAttackAction {
    Attack2,
    Attack3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum WeaponFallbackKind {
    Axe,
    Book,
    Bow,
    Dagger,
    Gatling,
    Grenade,
    Huuma,
    Katar,
    Knuckle,
    Mace,
    Musical,
    Revolver,
    Rifle,
    Rod,
    Shotgun,
    Spear,
    Sword,
    Whip,
}

pub(crate) fn weapon_fallback(weapon: u32) -> Option<WeaponFallback> {
    Some(weapon_fallback_kind(weapon)?.fallback())
}

pub(crate) fn weapon_projectile_effect_path(weapon: u32) -> Option<&'static str> {
    match weapon_fallback_kind(weapon)? {
        WeaponFallbackKind::Bow => Some("arrowshot.str"),
        _ => None,
    }
}

fn weapon_fallback_kind(weapon: u32) -> Option<WeaponFallbackKind> {
    use WeaponFallbackKind::*;

    match weapon {
        1201..=1249 | 13000..=13099 | 28700..=28799 | 500073 | 510000..=510199 => Some(Dagger),
        1100..=1199 | 13400..=13499 | 21000..=21099 | 32350..=32399 | 500000..=500199 | 600000..=600099 => Some(Sword),
        1250..=1299 | 28000..=28099 | 610000..=610199 => Some(Katar),
        1339 | 13300..=13399 | 650000..=650099 => Some(Huuma),
        1340 | 1501..=1549 | 1599 | 16000..=16099 | 32400..=32499 | 590000..=590199 => Some(Mace),
        1300..=1399 | 28100..=28199 | 520000..=520199 | 620000..=620199 => Some(Axe),
        1472..=1473 | 1600..=1699 | 2000..=2064 | 26100..=26199 | 550000..=550199 | 640000..=640099 => Some(Rod),
        1400..=1499 | 26000..=26099 | 32000..=32099 | 530000..=530199 | 630000..=630199 => Some(Spear),
        1550..=1598 | 28600..=28699 | 540000..=540199 => Some(Book),
        1701..=1749 | 18100..=18299 | 700000..=700199 => Some(Bow),
        1800..=1848 | 1857..=1859 | 1861..=1862 | 1864..=1867 | 1870..=1873 | 560000..=560199 => Some(Knuckle),
        1900..=1949 | 32101..=32105 | 32107..=32117 | 570000..=570199 => Some(Musical),
        1950..=1999 | 26200..=26299 | 32106 | 580000..=580199 => Some(Whip),
        13100..=13149 | 32300..=32349 | 800000..=800099 => Some(Revolver),
        13150..=13153
        | 13163..=13166
        | 13170..=13171
        | 13175..=13176
        | 13180
        | 13184
        | 13189..=13190
        | 13195
        | 28203
        | 28214..=28215
        | 28223
        | 28227..=28228
        | 28235..=28236
        | 28240
        | 28249
        | 28253
        | 28255
        | 810000..=810099 => Some(Rifle),
        13154..=13156
        | 13167..=13169
        | 13173
        | 13178
        | 13181
        | 13186
        | 13192..=13194
        | 13196
        | 28204
        | 28218
        | 28224
        | 28233..=28234
        | 28238
        | 28242
        | 28244
        | 28251
        | 28256
        | 28261
        | 820000..=820099 => Some(Shotgun),
        13157..=13159
        | 13172
        | 13177
        | 13182
        | 13185
        | 13197..=13199
        | 28216
        | 28225
        | 28229..=28230
        | 28237
        | 28241
        | 28250
        | 28254
        | 28258
        | 830000..=830099 => Some(Gatling),
        13160..=13162
        | 13174
        | 13179
        | 13183
        | 13187
        | 28200..=28202
        | 28217
        | 28226
        | 28231..=28232
        | 28239
        | 28243
        | 28252
        | 28257
        | 840000..=840099 => Some(Grenade),
        _ => None,
    }
}

impl WeaponFallbackKind {
    fn fallback(self) -> WeaponFallback {
        match self {
            Self::Axe => WeaponFallback {
                sprite_name: "\u{b3c4}\u{b07c}",
                hit_sound_path: "_hit_axe.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Book => WeaponFallback {
                sprite_name: "\u{cc45}",
                hit_sound_path: "_hit_mace.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Bow => WeaponFallback {
                sprite_name: "\u{d65c}",
                hit_sound_path: "_hit_arrow.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Dagger => WeaponFallback {
                sprite_name: "\u{b2e8}\u{ac80}",
                hit_sound_path: "_hit_dagger.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Gatling => WeaponFallback {
                sprite_name: "\u{ae30}\u{ad00}\u{cd1d}",
                hit_sound_path: "_hit_\u{ac1c}\u{d2c0}\u{b9c1}\u{d55c}\u{bc1c}.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Grenade => WeaponFallback {
                sprite_name: "\u{adf8}\u{b808}\u{b124}\u{c774}\u{b4dc}\u{b7f0}\u{ccd0}",
                hit_sound_path: "_hit_\u{adf8}\u{b808}\u{b124}\u{c774}\u{b4dc}\u{b7f0}\u{ccd0}.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Huuma => WeaponFallback {
                sprite_name: "\u{c218}\u{b9ac}\u{ac80}",
                hit_sound_path: "_hit_dagger.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Katar => WeaponFallback {
                sprite_name: "\u{ce74}\u{d0c0}\u{b974}",
                hit_sound_path: "_hit_dagger.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Knuckle => WeaponFallback {
                sprite_name: "\u{b108}\u{d074}",
                hit_sound_path: "_hit_fist1.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Mace => WeaponFallback {
                sprite_name: "\u{b454}\u{ae30}",
                hit_sound_path: "_hit_mace.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Musical => WeaponFallback {
                sprite_name: "\u{c545}\u{ae30}",
                hit_sound_path: "_hit_mace.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Revolver => WeaponFallback {
                sprite_name: "\u{ad8c}\u{cd1d}",
                hit_sound_path: "_hit_\u{ad8c}\u{cd1d}.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Rifle => WeaponFallback {
                sprite_name: "\u{b77c}\u{c774}\u{d50c}",
                hit_sound_path: "_hit_\u{b77c}\u{c774}\u{d50c}.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Rod => WeaponFallback {
                sprite_name: "\u{b86f}\u{b4dc}",
                hit_sound_path: "_hit_rod.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Shotgun => WeaponFallback {
                sprite_name: "\u{c0f7}\u{ac74}",
                hit_sound_path: "_hit_\u{c0f7}\u{ac74}.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Spear => WeaponFallback {
                sprite_name: "\u{cc3d}",
                hit_sound_path: "_hit_spear.wav",
                attack_action: WeaponAttackAction::Attack3,
            },
            Self::Sword => WeaponFallback {
                sprite_name: "\u{ac80}",
                hit_sound_path: "_hit_sword.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
            Self::Whip => WeaponFallback {
                sprite_name: "\u{cc44}\u{cc0d}",
                hit_sound_path: "_hit_sword.wav",
                attack_action: WeaponAttackAction::Attack2,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bow_weapon_uses_arrowshot_projectile_effect() {
        assert_eq!(weapon_projectile_effect_path(1701), Some("arrowshot.str"));
    }

    #[test]
    fn non_bow_weapon_has_no_projectile_effect() {
        assert_eq!(weapon_projectile_effect_path(1101), None);
    }
}
