const moves = [
  {
    name: "Mindflare",
    type: "Psyche",
    category: "Special",
    power: 90,
    accuracy: 1.0,
    pp: 10,
    effect: {
      chance: 0.3,
      stat: "special",
      stages: -1
    }
  },
  {
    name: "Revitalize",
    type: "Neutral",
    category: "Status",
    power: null,
    accuracy: null,
    pp: 20,
    effect: {
      heals: 0.5
    }
  },
  {
    name: "Spark Shackle",
    type: "Spark",
    category: "Status",
    power: null,
    accuracy: 1.0,
    pp: 20,
    effect: {
      statusCondition: "paralysis"
    }
  },
  {
    name: "Groundbreaker",
    type: "Brawler",
    category: "Physical",
    power: null,
    accuracy: 1.0,
    pp: 20,
    effect: {
      damage: "user_level"
    }
  },
  {
    name: "Dream Dust",
    type: "Flora",
    category: "Status",
    power: null,
    accuracy: 0.75,
    pp: 15,
    effect: {
      statusCondition: "sleep"
    }
  },
  {
    name: "Nature's Hold",
    type: "Flora",
    category: "Status",
    power: null,
    accuracy: 0.75,
    pp: 30,
    effect: {
      statusCondition: "paralysis"
    }
  },
  {
    name: "Verdant Slash",
    type: "Flora",
    category: "Physical",
    power: 55,
    accuracy: 0.95,
    pp: 25,
    effect: {
      highCriticalHitRatio: true
    }
  },
  {
    name: "Binding Vines",
    type: "Neutral",
    category: "Physical",
    power: 15,
    accuracy: 0.9,
    pp: 20,
    effect: {
      trap: {
        duration: "4"
      }
    }
  },
  {
    name: "Mind Shield",
    type: "Psyche",
    category: "Status",
    power: null,
    accuracy: null,
    pp: 20,
    effect: {
      stat: "special",
      stages: 2
    }
  },
  {
    name: "Tidal Wave",
    type: "Aqua",
    category: "Special",
    power: 95,
    accuracy: 1.0,
    pp: 15,
    effect: null
  },
  {
    name: "Tranquil Slumber",
    type: "Psyche",
    category: "Status",
    power: null,
    accuracy: null,
    pp: 10,
    effect: {
      sleepTurns: 2,
      heal: "full"
    }
  },
  {
    name: "Aqua Bind",
    type: "Aqua",
    category: "Physical",
    power: 35,
    accuracy: 0.85,
    pp: 10,
    effect: {
      trap: {
        duration: "4"
      }
    }
  },
  {
    name: "Frost Vortex",
    type: "Frost",
    category: "Special",
    power: 120,
    accuracy: 0.9,
    pp: 5,
    effect: {
      chance: 0.1,
      statusCondition: "freeze"
    }
  },
  {
    name: "Celestial Ray",
    type: "Neutral",
    category: "Special",
    power: 150,
    accuracy: 0.9,
    pp: 5,
    effect: {
      recharge: true
    }
  },
  {
    name: "Volatile Burst",
    type: "Neutral",
    category: "Physical",
    power: 170,
    accuracy: 1.0,
    pp: 5,
    effect: {
      selfDestruct: true
    }
  },
  {
    name: "Mind Mesmerize",
    type: "Psyche",
    category: "Status",
    power: null,
    accuracy: 0.6,
    pp: 20,
    effect: {
      statusCondition: "sleep"
    }
  },
  {
    name: "Storm Surge",
    type: "Spark",
    category: "Special",
    power: 95,
    accuracy: 1.0,
    pp: 15,
    effect: {
      chance: 0.1,
      statusCondition: "paralysis"
    }
  },
  {
    name: "Terra Tremor",
    type: "Terra",
    category: "Physical",
    power: 100,
    accuracy: 1.0,
    pp: 10,
    effect: null
  },
  {
    name: "Stonefall",
    type: "Boulder",
    category: "Physical",
    power: 75,
    accuracy: 0.9,
    pp: 10,
    effect: {
      chance: 0.3,
      statusCondition: "flinch"
    }
  },
  {
    name: "Crushing Force",
    type: "Neutral",
    category: "Physical",
    power: 85,
    accuracy: 1.0,
    pp: 15,
    effect: {
      chance: 0.3,
      statusCondition: "paralysis"
    }
  },
  {
    name: "Glacial Ray",
    type: "Frost",
    category: "Special",
    power: 95,
    accuracy: 1.0,
    pp: 10,
    effect: {
      chance: 0.1,
      statusCondition: "freeze"
    }
  },
  {
    name: "Nourishing Light",
    type: "Neutral",
    category: "Status",
    power: null,
    accuracy: null,
    pp: 10,
    effect: {
      heals: 0.5
    }
  },
  {
    name: "Sky Lance",
    type: "Zephyr",
    category: "Physical",
    power: 80,
    accuracy: 1.0,
    pp: 20,
    effect: null
  },
  {
    name: "Swift Breeze",
    type: "Psyche",
    category: "Status",
    power: null,
    accuracy: null,
    pp: 30,
    effect: {
      stat: "speed",
      stages: 2
    }
  },
  {
    name: "Inferno Burst",
    type: "Inferno",
    category: "Special",
    power: 120,
    accuracy: 0.85,
    pp: 5,
    effect: {
      chance: 0.1,
      statusCondition: "burn"
    }
  },
  {
    name: "Flame Spiral",
    type: "Inferno",
    category: "Special",
    power: 35,
    accuracy: 0.85,
    pp: 15,
    effect: {
      trap: {
        duration: "4"
      }
    }
  },
  {
    name: "Phantom Glare",
    type: "Specter",
    category: "Status",
    power: null,
    accuracy: 1.0,
    pp: 10,
    effect: {
      statusCondition: "confusion"
    }
  }
];