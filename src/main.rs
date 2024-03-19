use clap::Parser;
use rlbot_lib::{
    self,
    rlbot::{
        ControllerState, GameTickPacket, PlayerInput, QuickChatSelection, ReadyMessage, QuickChat,
    },
    Packet, RLBotConnection,
};
use std::{env, process::exit};

mod utils;
mod cli;
mod bot;
mod actions;
mod solo_strategy;

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
        exit(0);
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

    loop {
        match rlbot_connection.recv_packet().unwrap() {
            Packet::GameTickPacket(packet) => {
                rlbot_connection = handle_game_tick_packet(packet, rlbot_connection, car_id);
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
            },
            Packet::BallPrediction(_packet) => {
                // println!("Ball Prediction Packet:\n{packet:?}");
            },
            // Packet::ReadyMessage(_) => todo!(),
            // Packet::MessagePacket(_) => todo!(),
            // Packet::FieldInfo(_) => continue,
            // Packet::MatchSettings(_) => continue,
            // Packet::DesiredGameState(_) => continue,
            // Packet::RenderGroup(_) => todo!(),
            _ => (),
        };
        // let Packet::GameTickPacket(game_tick_packet) = rlbot_connection.recv_packet().unwrap()
        // else {
        //     continue;
        // };
    }
}
