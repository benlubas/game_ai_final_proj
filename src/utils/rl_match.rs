use rlbot_lib::rlbot::{
    BoostOption, ExistingMatchBehavior, GameMap, GameMode, MatchLength, MatchSettings,
    MutatorSettings, PlayerClass, PlayerConfiguration, PlayerLoadout, PsyonixBotPlayer,
    RLBotPlayer,
};

/// Starting a match
pub fn start_match() -> MatchSettings {
    MatchSettings {
        playerConfigurations: Some(vec![
            PlayerConfiguration {
                variety: PlayerClass::RLBotPlayer(Box::new(RLBotPlayer {})),
                name: Some("BOT1".to_owned()),
                team: 0,
                loadout: Some(Box::new(PlayerLoadout::default())),
                spawnId: 0,
            },
            PlayerConfiguration {
                // variety: PlayerClass::RLBotPlayer(Box::new(RLBotPlayer {})),

                // I'm not sure how to get it to automatically spawn me in
                // variety: PlayerClass::HumanPlayer(Box::new(HumanPlayer { _tab: 0 })),
                variety: PlayerClass::PsyonixBotPlayer(Box::new(PsyonixBotPlayer { botSkill: 0. })),
                name: Some("BOT2".to_owned()),
                team: 1,
                loadout: Some(Box::new(PlayerLoadout::default())),
                spawnId: 1,
            },
        ]),
        gameMode: GameMode::Soccer,
        // There's not much freedom with the maps. Seems like only a few of them work
        gameMap: GameMap::DFHStadium,
        // mutatorSettings CANNOT be None, otherwise RLBot will crash
        mutatorSettings: Some(Box::new(MutatorSettings {
            matchLength: MatchLength::Unlimited,
            boostOption: BoostOption::Unlimited_Boost,
            ..Default::default()
        })),
        existingMatchBehavior: ExistingMatchBehavior::Restart,
        enableRendering: true,
        instantStart: true,
        enableStateSetting: true,
        // skipReplays: true,
        ..Default::default()
    }
}
