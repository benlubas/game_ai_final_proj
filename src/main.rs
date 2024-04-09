use clap::Parser;
use rlbot_lib::{
    self,
    rlbot::{QuickChat, QuickChatSelection, ReadyMessage, RenderGroup},
    Packet, RLBotConnection,
};
use std::env;

use crate::{
    bot::bot::Agent,
    strategies::{solo_strategy::SoloStrategy, test_strategy::TestStrategy},
};

mod actions;
mod bot;
mod cli;
mod strategies;
mod utils;

const DEFAULT_CAR_ID: usize = 0;

fn main() -> ! {
    println!("Connecting");

    let args = cli::cli::Args::parse();
    let mut rlbot_connection = RLBotConnection::new("127.0.0.1:23234").expect("connection");

    if args.start {
        println!("Starting Match");
        let match_settings = utils::rl_match::start_match();
        rlbot_connection
            .send_packet(Packet::MatchSettings(match_settings))
            .expect("Failed to Start Match");
    }

    println!("Running!");

    rlbot_connection
        .send_packet(Packet::ReadyMessage(ReadyMessage {
            wantsBallPredictions: true,
            wantsQuickChat: true,
            wantsGameMessages: true,
        }))
        .unwrap();

    let car_id = env::var("CAR_ID")
        .map(|x| x.parse().unwrap())
        .unwrap_or(DEFAULT_CAR_ID);

    println!("Creating Agent");
    let mut agent: Agent;
    // if args.test {
    //     agent = Agent::new(true, car_id, TestStrategy {});
    // } else {
        agent = Agent::new(true, car_id, SoloStrategy {});
    // }

    // // setup the game according to the strategy
    // if let Some(state) = agent.strategy.set_game_state() {
    //     rlbot_connection
    //         .send_packet(Packet::DesiredGameState(state))
    //         .unwrap();
    // }
    loop {
        match rlbot_connection.recv_packet() {
            Ok(received_packet) => {
                match received_packet {
                    Packet::GameTickPacket(packet) => {
                        let res = agent.handle_game_tick(packet);
                        println!("{:?}", res.input);
                        println!("{:?}", res.render);
                        rlbot_connection
                            .send_packet(Packet::PlayerInput(res.input))
                            .unwrap();
                        rlbot_connection
                            .send_packet(Packet::RenderGroup(RenderGroup {
                                renderMessages: Some(res.render),
                                id: 456, // NOTE: ~~I might need to make these unique.~~ I don't
                            }))
                            .unwrap();
                    }
                    Packet::QuickChat(packet) => {
                        if packet.quickChatSelection == QuickChatSelection::Compliments_WhatASave {
                            // WARN: ignoring a result here
                            let _ = rlbot_connection.send_packet(Packet::QuickChat(QuickChat {
                                quickChatSelection: QuickChatSelection::Compliments_Thanks,
                                ..QuickChat::default()
                            }));
                        }
                    }
                    Packet::BallPrediction(_packet) => {
                        // println!("Ball Prediction Packet:\n{packet:?}");
                    }
                    // Packet::ReadyMessage(_) => todo!(),
                    // Packet::MessagePacket(packet) => {
                    //     println!("{packet:?}");
                    // }
                    // Packet::FieldInfo(_) => continue,
                    // Packet::MatchSettings(_) => continue,
                    // Packet::DesiredGameState(_) => continue,
                    // Packet::RenderGroup(_) => todo!(),
                    _ => continue,
                };
            }
            Err(_e) => {} // println!("packet error:\n{e:?}"),
        };
    }
}
