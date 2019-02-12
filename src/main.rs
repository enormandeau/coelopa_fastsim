#![allow(warnings)]
// Modules
extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate rand;
use rand::prelude::*;
use rand::Rng;

use std::vec::Vec;
use std::io::prelude::*;
use std::io;
use std::env;

// Enums
#[derive(Debug)]
#[derive(Copy,Clone)]
enum Sex { female, male }

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Sex::female => "female",
            Sex::male => "male",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
#[derive(Copy,Clone)]
enum Genotype { AA, AB, BB }

impl std::fmt::Display for Genotype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Genotype::AA => "AA",
            Genotype::AB => "AB",
            Genotype::BB => "BB",
        };
        write!(f, "{}", printable)
    }
}

// Structs
#[derive(Debug)]
#[derive(Copy,Clone)]
struct Fly {
    sex: Sex,
    genotype: Genotype
}

impl std::fmt::Display for Fly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}_{}", self.sex, self.genotype)
    }
}

#[derive(Debug)]
#[derive(Copy,Clone)]
struct ProportionSexe {
    sex: Sex,
    proportion: f64
}

#[derive(Debug)]
#[derive(Copy,Clone)]
struct ProportionGenotype {
    genotype: Genotype,
    proportion: f64
}

// Functions
fn create_first_generation(n: &u32, psexes: &Vec<ProportionSexe>,
                           pgenotypes: &Vec<ProportionGenotype>) -> Vec<Fly> {

    let mut rng = rand::thread_rng();
    let mut v = Vec::new();

    // Create adults with random sex and genotype using proportions
    for _ in 0..*n {
        let sex = psexes.choose_weighted(&mut rng, |item| item.proportion).unwrap().sex;
        let genotype = pgenotypes.choose_weighted(&mut rng, |item| item.proportion).unwrap().genotype;

        v.push(Fly{ sex: sex, genotype: genotype });
    }

    v
}

// Main
fn main() {
    // Parameters
    let output_file = "output_file.txt";
    let number_generations = 5;
    let number_eggs_per_generation = 1000;
    let number_eggs_per_female = 50;
    let proportion_females = 0.5;
    let proportion_aa = 0.07;
    let proportion_bb = 0.44;
    let survival_global = 0.3;
    let survival_females_aa = 0.71;
    let survival_females_ab = 0.9;
    let survival_females_bb = 1.0;
    let survival_males_aa = 0.81;
    let survival_males_ab = 1.0;
    let survival_males_bb = 1.0;
    let male_success_aa = 1.0;
    let male_success_ab = 0.55;
    let male_success_bb = 0.1;
    let male_freq_dep_coef = 0.0;
    let female_eggs_aa = 1.0;
    let female_eggs_ab = 0.97;
    let female_eggs_bb = 0.87;
    let female_maturation_days = 8.8;
    let male_maturation_days_aa = 12.8;
    let male_maturation_days_ab = 10.3;
    let male_maturation_days_bb = 8.7;
    let maturation_cv = 0.5;
    let environment_time = 10.0;
    let environment_time_variation = 1.0;

    // Initialize random number generation
    let mut rng = rand::thread_rng();

    // Compute derived parameters
    let proportion_ab = 1.0 - proportion_aa - proportion_bb;
    let proportion_males = 1.0 - proportion_females;

    // Create global variables
    let proportion_sexes = vec![
        ProportionSexe{ sex: Sex::female, proportion: proportion_females },
        ProportionSexe{ sex: Sex::male, proportion: proportion_males },
    ];

    let proportion_genotypes = vec![
        ProportionGenotype{ genotype: Genotype::AA, proportion: proportion_aa },
        ProportionGenotype{ genotype: Genotype::AB, proportion: proportion_ab },
        ProportionGenotype{ genotype: Genotype::BB, proportion: proportion_bb },
    ];

    // Create initial fly and eggs vectors
    let mut individual_adults: Vec<Fly> = Vec::new();
    let mut individual_eggs: Vec<Fly> = Vec::new();

    // Generate first generation of eggs
    let number_adults = number_eggs_per_generation as f64 * survival_global;
    let number_adults = number_adults as u32;

    let individual_eggs_bogus = create_first_generation(
        &number_adults,
        &proportion_sexes,
        &proportion_genotypes
        );

//    for i in individual_eggs_bogus {
//        println!("{}", i);
//    }

    for gen in 1..=number_generations {

        // Skip egg part of the cycle for generation 0
        if gen != 1 {
            let a = 0;
        }

        println!("Generation: {:5}", gen);
    }
}
