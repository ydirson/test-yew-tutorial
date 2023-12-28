// https://army-forge.onepagerules.com/api/tts?id=ybjR2-7kHUNY
pub const JSON_DATA: &str = r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gff",
   "id" : "ybjR2-7kHUNY",
   "isCloud" : false,
   "modified" : "2023-11-13T23:04:45.116Z",
   "name" : "Prime Brothers 200pts",
   "points" : 200,
   "pointsLimit" : 200,
   "selectedUnitId" : null,
   "specialRules" : [
      {
         "aliasedRuleId" : 48,
         "description" : "this model and all friendly units that are within 12” at the beginning of the round get AP(+1) when shooting. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Precision Shots"
      },
      {
         "aliasedRuleId" : 45,
         "description" : "this model and all friendly units that are within 12” at the beginning of the round get +1 to hit when shooting. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 45,
         "name" : "Battle Rites"
      },
      {
         "aliasedRuleId" : 857,
         "description" : "Attacks targeting units where all models have this rule count as having AP(-1), to a min. of AP(0).",
         "hasRating" : false,
         "id" : 857,
         "name" : "Heavy Shield"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Gets +1 to hit in melee and shooting.",
         "hasRating" : false,
         "id" : 128,
         "name" : "Veteran Infantry"
      },
      {
         "aliasedRuleId" : 561,
         "description" : "this model and all friendly units that are within 12” at the beginning of the round get Regeneration. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 561,
         "name" : "Medical Training"
      },
      {
         "aliasedRuleId" : 55,
         "description" : "this model and all friendly units that are within 12” at the beginning of the round get Furious. If they already had Furious, they get extra hits on unmodified rolls of 5-6 instead. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 55,
         "name" : "War Chant"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "Xz6Ig",
         "size" : 1,
         "sortId" : 4,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : false,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "9mH34",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "-38vE",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 40,
         "defense" : 2,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "id" : "NeuN0Gx",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "name" : "Infernal",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "wSM6O",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 65,
               "key" : "fearless",
               "name" : "Fearless"
            },
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 60,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "id" : "qGGJpGt",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "name" : "Elite Raider",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "ui7dg",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "furious",
               "name" : "Furious",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "C1",
            "qIWMF",
            "A1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 40,
         "defense" : 2,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "id" : "NeuN0Gx",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "name" : "Infernal",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "4AxnW",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 65,
               "key" : "fearless",
               "name" : "Fearless"
            },
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#;
