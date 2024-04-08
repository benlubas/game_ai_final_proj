use clap::Parser;
use rlbot_lib::{
    self,
    rlbot::{
        ControllerState, GameTickPacket, PlayerInput, QuickChat, QuickChatSelection, ReadyMessage,
    },
    Packet, RLBotConnection,
};
use std::{env, process::exit};

use crate::{bot::bot::Agent, solo_strategy::strategy::SoloStrategy};

mod actions;
mod bot;
mod cli;
mod solo_strategy;
mod utils;

const DEFAULT_CAR_ID: usize = 0;

fn main() {
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
            wantsBallPredictions: false,
            wantsQuickChat: false,
            wantsGameMessages: true,
        }))
        .unwrap();

    let car_id = env::var("CAR_ID")
        .map(|x| x.parse().unwrap())
        .unwrap_or(DEFAULT_CAR_ID);

    println!("Creating Agent");
    let mut agent = Agent::new(true, car_id, SoloStrategy {});

    loop {
        match rlbot_connection.recv_packet() {
            Ok(received_packet) => {
                match received_packet {
                    Packet::GameTickPacket(packet) => {
                        let input = agent.handle_game_tick(packet);
                        rlbot_connection
                            .send_packet(Packet::PlayerInput(input))
                            .unwrap();
                    }
                    Packet::PlayerInput(_) => continue, // TODO: takeover mode?
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
                    // Packet::MessagePacket(_) => todo!(),
                    // Packet::FieldInfo(_) => continue,
                    // Packet::MatchSettings(_) => continue,
                    // Packet::DesiredGameState(_) => continue,
                    // Packet::RenderGroup(_) => todo!(),
                    _ => continue,
                };
            }
            Err(e) => println!("packet error:\n{e:?}"),
        };
    }
}
