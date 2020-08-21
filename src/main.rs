mod breakout;
mod data;
mod graph;
mod parse;

extern crate clap;

use clap::{App, AppSettings, Arg};
use std::str::FromStr;

use data::*;

use graph::*;
use crate::breakout::println_breakdown;

// todo configurable output file
// todo switch institution over to using an enum

fn main() {
    let matches = App::new("benford")
        .author("Trent Johnson <trent@trent.io>")
        .about("Learned about Benford's law thought it was neat")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
        App::new("graph")
            .about("Make a graph based on some data")
            .arg(
                Arg::with_name("type")
                    .short('t')
                    .long("type")
                    .default_value("random")
                    .required(true)
                    .possible_values(&["random", "buildings", "transactions"]))
            .arg(
                Arg::with_name("size")
                    .about("the number of random values if random is chosen")
                    .short('s')
                    .long("size")
                    .default_value("1000")
            )
            .arg(
                Arg::with_name("institution")
                    .short('i')
                    .long("long")
                    .possible_values(&["veridian", "silverlake"])
                    .default_value("veridian")
            )
        )
        .get_matches();

    if let Some(graph) = matches.subcommand_matches("graph") {
        let graph_type: UseCase= graph.value_of_t("type").unwrap_or_else(|e| e.exit());

        match graph_type {
            UseCase::RandomNumbers => {
                let size: isize = graph.value_of_t("size").unwrap_or(1000);
                let data = gen_random_num_vector(size).into_iter().map(|x| first_digit(x)).collect();
                println_breakdown(&data);
                println!("Random Number distribution graph created {:?}", create_graph(data))
            },
            UseCase::TallestBuilding => {
                let data: Vec<usize> = tallest_buildings().into_iter().map(|x| first_digit(x)).collect();
                println_breakdown(&data);
                println!("Tallest buildings!!! {:?}", create_graph(data));
            },
            UseCase::Transactions => {
                let inst = graph.value_of_t("institution").unwrap_or("veridian".to_string());
                let data: Vec<usize> = transaction_amounts(inst).into_iter().map(|x| first_digit(x)).collect();
                println_breakdown(&data);
                println!("Transactions Graph Created: {:?}", create_graph(data));
            }
        }
    }
}



enum UseCase {
    RandomNumbers,
    TallestBuilding,
    Transactions,
}

impl FromStr for UseCase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Buildings" | "buildings" => Ok(UseCase::TallestBuilding),
            "Random" | "random" => Ok(UseCase::RandomNumbers),
            "Transactions" | "transactions" => Ok(UseCase::Transactions),
            _ => Err("no match"),
        }
    }
}









