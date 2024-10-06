use std::u8;

use anchor_lang::prelude::*;
use strum_macros::EnumString;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct MovementInfo {
    pub movement: Movement,
    pub m_type: MovementType,
    pub category: MovementCategory,
    pub power: Option<u8>,
    pub accuracy: Option<u8>,
    pub pp: u8,
    pub effect: Option<Effect>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, EnumString)]
pub enum MovementType {
    Flora,
    Aqua,
    Inferno,
    Zephyr,
    Terra,
    Spark,
    Boulder,
    Specter,
    Metal,
    Psyche,
    Venom,
    Brawler,
    Shadow,
    Frost,
    Mystic,
    Bug,
    Neutral,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum MovementCategory {
    Status,
    Physical,
    Special,
    Change,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Effect {
    Damage { amount: u8 },
    Heals { amount: u8 },
    StatusCondition { condition: Status },
    SelfStatModifier { stat: Stats, stages: i8 },
    OpponentStatModifier { stat: Stats, stages: i8 },
    HighCriticalHitRatio,
    HealAndStatusCondition { amount: u8, condition: Status },
    Recharge,
    SelfDestruct,
    ChangeElemental { elemental: u8 },
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Stats {
    Special,
    Speed,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Status {
    Normal,
    Paralysis(u8),
    Confusion,
    Burn(u8),
    Freeze(u8),
    Flinch(u8),
    Sleep,
    Trap(u8),
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, EnumString)]
pub enum Movement {
    #[strum(serialize = "Mindflare")]
    Mindflare,
    #[strum(serialize = "Revitalize")]
    Revitalize,
    #[strum(serialize = "Spark Shackle")]
    SparkShackle,
    #[strum(serialize = "Groundbreaker")]
    Groundbreaker,
    #[strum(serialize = "Dream Dust")]
    DreamDust,
    #[strum(serialize = "Nature's Hold")]
    NaturesHold,
    #[strum(serialize = "Verdant Slash")]
    VerdantSlash,
    #[strum(serialize = "Binding Vines")]
    BindingVines,
    #[strum(serialize = "Mind Shield")]
    MindShield,
    #[strum(serialize = "Tidal Wave")]
    TidalWave,
    #[strum(serialize = "Tranquil Slumber")]
    TranquilSlumber,
    #[strum(serialize = "Aqua Bind")]
    AquaBind,
    #[strum(serialize = "Frost Vortex")]
    FrostVortex,
    #[strum(serialize = "Celestial Ray")]
    CelestialRay,
    #[strum(serialize = "Volatile Burst")]
    VolatileBurst,
    #[strum(serialize = "Mind Mesmerize")]
    MindMesmerize,
    #[strum(serialize = "Storm Surge")]
    StormSurge,
    #[strum(serialize = "Terra Tremor")]
    TerraTremor,
    #[strum(serialize = "Stonefall")]
    Stonefall,
    #[strum(serialize = "Crushing Force")]
    CrushingForce,
    #[strum(serialize = "Glacial Ray")]
    GlacialRay,
    #[strum(serialize = "Nourishing Light")]
    NourishingLight,
    #[strum(serialize = "Sky Lance")]
    SkyLance,
    #[strum(serialize = "Swift Breeze")]
    SwiftBreeze,
    #[strum(serialize = "Inferno Burst")]
    InfernoBurst,
    #[strum(serialize = "Flame Spiral")]
    FlameSpiral,
    #[strum(serialize = "Phantom Glare")]
    PhantomGlare,
    #[strum(serialize = "Switch Elemental")]
    SwitchElemental(u8),
}

impl Movement {
    pub fn get_info(&self) -> MovementInfo {
        use crate::movements::Status::*;
        use Effect::*;
        use Movement::*;
        use MovementCategory::*;
        use MovementType::*;

        match self {
            Mindflare => MovementInfo {
                movement: Mindflare,
                m_type: Psyche,
                category: Special,
                power: Some(90),
                accuracy: Some(100),
                pp: 10,
                effect: Some(OpponentStatModifier {
                    stat: Stats::Special,
                    stages: -1,
                }),
            },
            Revitalize => MovementInfo {
                movement: Revitalize,
                m_type: Neutral,
                category: Status,
                power: None,
                accuracy: None,
                pp: 20,
                effect: Some(Heals { amount: 50 }),
            },
            SparkShackle => MovementInfo {
                movement: SparkShackle,
                m_type: Spark,
                category: Status,
                power: None,
                accuracy: Some(100),
                pp: 20,
                effect: Some(StatusCondition {
                    condition: Paralysis(100),
                }),
            },
            Groundbreaker => MovementInfo {
                movement: Groundbreaker,
                m_type: Brawler,
                category: Physical,
                power: None,
                accuracy: Some(100),
                pp: 20,
                effect: Some(Damage { amount: 50 }),
            },
            DreamDust => MovementInfo {
                movement: DreamDust,
                m_type: Flora,
                category: Status,
                power: None,
                accuracy: Some(75),
                pp: 15,
                effect: Some(StatusCondition { condition: Sleep }),
            },
            NaturesHold => MovementInfo {
                movement: NaturesHold,
                m_type: Flora,
                category: Status,
                power: None,
                accuracy: Some(75),
                pp: 30,
                effect: Some(StatusCondition {
                    condition: Paralysis(100),
                }),
            },
            VerdantSlash => MovementInfo {
                movement: VerdantSlash,
                m_type: Flora,
                category: Physical,
                power: Some(55),
                accuracy: Some(95),
                pp: 25,
                effect: Some(HighCriticalHitRatio),
            },
            BindingVines => MovementInfo {
                movement: BindingVines,
                m_type: Neutral,
                category: Physical,
                power: Some(15),
                accuracy: Some(90),
                pp: 20,
                effect: Some(StatusCondition { condition: Trap(4) }),
            },
            MindShield => MovementInfo {
                movement: MindShield,
                m_type: Psyche,
                category: Status,
                power: None,
                accuracy: None,
                pp: 20,
                effect: Some(SelfStatModifier {
                    stat: Stats::Special,
                    stages: 2,
                }),
            },
            TidalWave => MovementInfo {
                movement: TidalWave,
                m_type: Aqua,
                category: Special,
                power: Some(95),
                accuracy: Some(100),
                pp: 15,
                effect: None,
            },
            TranquilSlumber => MovementInfo {
                movement: TranquilSlumber,
                m_type: Psyche,
                category: Status,
                power: None,
                accuracy: None,
                pp: 10,
                effect: Some(HealAndStatusCondition {
                    amount: 100,
                    condition: Sleep,
                }),
            },
            AquaBind => MovementInfo {
                movement: AquaBind,
                m_type: Aqua,
                category: Physical,
                power: Some(35),
                accuracy: Some(85),
                pp: 10,
                effect: Some(StatusCondition { condition: Trap(4) }),
            },
            FrostVortex => MovementInfo {
                movement: FrostVortex,
                m_type: Frost,
                category: Special,
                power: Some(120),
                accuracy: Some(90),
                pp: 5,
                effect: Some(StatusCondition {
                    condition: Freeze(10),
                }),
            },
            CelestialRay => MovementInfo {
                movement: CelestialRay,
                m_type: Neutral,
                category: Special,
                power: Some(150),
                accuracy: Some(90),
                pp: 5,
                effect: Some(Recharge),
            },
            VolatileBurst => MovementInfo {
                movement: VolatileBurst,
                m_type: Neutral,
                category: Physical,
                power: Some(170),
                accuracy: Some(100),
                pp: 5,
                effect: Some(SelfDestruct),
            },
            MindMesmerize => MovementInfo {
                movement: MindMesmerize,
                m_type: Psyche,
                category: Status,
                power: None,
                accuracy: Some(60),
                pp: 20,
                effect: Some(StatusCondition { condition: Sleep }),
            },
            StormSurge => MovementInfo {
                movement: StormSurge,
                m_type: Spark,
                category: Special,
                power: Some(95),
                accuracy: Some(100),
                pp: 15,
                effect: Some(StatusCondition {
                    condition: Paralysis(10),
                }),
            },
            TerraTremor => MovementInfo {
                movement: TerraTremor,
                m_type: Terra,
                category: Physical,
                power: Some(100),
                accuracy: Some(100),
                pp: 10,
                effect: None,
            },
            Stonefall => MovementInfo {
                movement: Stonefall,
                m_type: Boulder,
                category: Physical,
                power: Some(75),
                accuracy: Some(90),
                pp: 10,
                effect: Some(StatusCondition {
                    condition: Flinch(30),
                }),
            },
            CrushingForce => MovementInfo {
                movement: CrushingForce,
                m_type: Neutral,
                category: Physical,
                power: Some(85),
                accuracy: Some(100),
                pp: 15,
                effect: Some(StatusCondition {
                    condition: Paralysis(30),
                }),
            },
            GlacialRay => MovementInfo {
                movement: GlacialRay,
                m_type: Frost,
                category: Special,
                power: Some(95),
                accuracy: Some(100),
                pp: 10,
                effect: Some(StatusCondition {
                    condition: Freeze(10),
                }),
            },
            NourishingLight => MovementInfo {
                movement: NourishingLight,
                m_type: Neutral,
                category: Status,
                power: None,
                accuracy: None,
                pp: 10,
                effect: Some(Heals { amount: 50 }),
            },
            SkyLance => MovementInfo {
                movement: SkyLance,
                m_type: Zephyr,
                category: Physical,
                power: Some(80),
                accuracy: Some(100),
                pp: 20,
                effect: None,
            },
            SwiftBreeze => MovementInfo {
                movement: SwiftBreeze,
                m_type: Psyche,
                category: Status,
                power: None,
                accuracy: None,
                pp: 30,
                effect: Some(SelfStatModifier {
                    stat: Stats::Speed,
                    stages: 2,
                }),
            },
            InfernoBurst => MovementInfo {
                movement: InfernoBurst,
                m_type: Inferno,
                category: Special,
                power: Some(120),
                accuracy: Some(85),
                pp: 5,
                effect: Some(StatusCondition {
                    condition: Burn(10),
                }),
            },
            FlameSpiral => MovementInfo {
                movement: FlameSpiral,
                m_type: Inferno,
                category: Special,
                power: Some(35),
                accuracy: Some(85),
                pp: 15,
                effect: Some(StatusCondition { condition: Trap(4) }),
            },
            PhantomGlare => MovementInfo {
                movement: PhantomGlare,
                m_type: Specter,
                category: Status,
                power: None,
                accuracy: Some(100),
                pp: 10,
                effect: Some(StatusCondition {
                    condition: Confusion,
                }),
            },
            SwitchElemental(i) => MovementInfo {
                movement: SwitchElemental(*i),
                m_type: Neutral,
                category: Change,
                power: None,
                accuracy: None,
                pp: 0,
                effect: Some(ChangeElemental { elemental: *i }),
            },
        }
    }
}

impl MovementType {
    pub fn strong_against(&self) -> Vec<MovementType> {
        use MovementType::*;
        match self {
            Flora => [Aqua, Terra, Boulder].to_vec(),
            Aqua => [Inferno, Terra, Boulder].to_vec(),
            Inferno => [Flora, Bug, Frost, Boulder].to_vec(),
            Zephyr => [Flora, Brawler, Bug].to_vec(),
            Terra => [Spark, Metal, Inferno, Boulder].to_vec(),
            Spark => [Aqua, Zephyr].to_vec(),
            Boulder => [Zephyr, Bug, Frost].to_vec(),
            Specter => [Specter, Psyche].to_vec(),
            Metal => [Frost, Boulder].to_vec(),
            Psyche => [Brawler, Venom].to_vec(),
            Venom => [Flora, Brawler].to_vec(),
            Brawler => [Boulder, Metal, Shadow].to_vec(),
            Shadow => [Psyche, Specter].to_vec(),
            Frost => [Flora, Zephyr, Mystic].to_vec(),
            Mystic => [Mystic].to_vec(),
            Bug => [Flora, Psyche, Venom].to_vec(),
            Neutral => [].to_vec(),
        }
    }

    pub fn weak_against(&self) -> Vec<MovementType> {
        use MovementType::*;
        match self {
            Flora => [Inferno, Frost, Bug, Venom].to_vec(),
            Aqua => [Flora, Spark].to_vec(),
            Inferno => [Aqua, Terra, Inferno].to_vec(),
            Zephyr => [Spark, Boulder].to_vec(),
            Terra => [Aqua, Flora, Zephyr].to_vec(),
            Spark => [Terra, Boulder].to_vec(),
            Boulder => [Aqua, Flora, Brawler, Terra].to_vec(),
            Specter => [Shadow].to_vec(),
            Metal => [Inferno, Brawler, Venom].to_vec(),
            Psyche => [Shadow, Bug].to_vec(),
            Venom => [Psyche, Boulder].to_vec(),
            Brawler => [Psyche, Zephyr].to_vec(),
            Shadow => [Brawler].to_vec(),
            Frost => [Inferno, Metal, Boulder].to_vec(),
            Mystic => [Frost].to_vec(),
            Bug => [Inferno, Zephyr, Boulder].to_vec(),
            Neutral => [Brawler].to_vec(),
        }
    }
}
