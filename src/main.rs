#![allow(warnings)]
/// Modules
extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate rand;
use rand::prelude::*;
use rand::Rng;
use rand::distributions::Uniform;

use std::collections::HashMap;
use counter::Counter;
use std::vec::Vec;
use std::io::prelude::*;
use std::io;
use std::env;

/// Enums
#[derive(Copy,Clone)]
#[derive(Hash, Eq, PartialEq, Debug)]
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

#[derive(Copy,Clone)]
#[derive(Hash, Eq, PartialEq, Debug)]
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

/// Structs
#[derive(Copy,Clone)]
#[derive(Hash, Eq, PartialEq, Debug)]
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

/// Functions
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

/// Main
fn main() {
    /// Parameters
    // TODO Parse arguments with `clap`
    let output_file = "output_file.txt";
    let number_generations = 5;
    let number_eggs_per_generation = 10000;
    let number_eggs_per_female = 50 as f64;
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

    /// Survival and reproduction parameters
    // Survival from egg to adult
    let mut egg_survival: HashMap<&Fly, f64> = HashMap::new();
    egg_survival.insert(&Fly { sex: Sex::female, genotype: Genotype::AA }, survival_females_aa);
    egg_survival.insert(&Fly { sex: Sex::female, genotype: Genotype::AB }, survival_females_ab);
    egg_survival.insert(&Fly { sex: Sex::female, genotype: Genotype::BB }, survival_females_bb);
    egg_survival.insert(&Fly { sex: Sex::male, genotype: Genotype::AA }, survival_males_aa);
    egg_survival.insert(&Fly { sex: Sex::male, genotype: Genotype::AB }, survival_males_ab);
    egg_survival.insert(&Fly { sex: Sex::male, genotype: Genotype::BB }, survival_males_bb);

    // Number of eggs per female genotype
    let mut female_eggs: HashMap<&Fly, f64> = HashMap::new();
    female_eggs.insert(&Fly { sex: Sex::female, genotype: Genotype::AA }, number_eggs_per_female * female_eggs_aa);
    female_eggs.insert(&Fly { sex: Sex::female, genotype: Genotype::AB }, number_eggs_per_female * female_eggs_ab);
    female_eggs.insert(&Fly { sex: Sex::female, genotype: Genotype::BB }, number_eggs_per_female * female_eggs_bb);

    // Male reproductive sucess per genotype
    let mut male_success: HashMap<&Fly, f64> = HashMap::new();
    male_success.insert(&Fly { sex: Sex::male, genotype: Genotype::AA }, male_success_aa);
    male_success.insert(&Fly { sex: Sex::male, genotype: Genotype::AB }, male_success_ab);
    male_success.insert(&Fly { sex: Sex::male, genotype: Genotype::BB }, male_success_bb);

    // Maturation time
    let mut maturation_time: HashMap<&Fly, f64> = HashMap::new();
    maturation_time.insert(&Fly { sex: Sex::female, genotype: Genotype::AA }, female_maturation_days);
    maturation_time.insert(&Fly { sex: Sex::female, genotype: Genotype::AB }, female_maturation_days);
    maturation_time.insert(&Fly { sex: Sex::female, genotype: Genotype::BB }, female_maturation_days);
    maturation_time.insert(&Fly { sex: Sex::male, genotype: Genotype::AA }, male_maturation_days_aa);
    maturation_time.insert(&Fly { sex: Sex::male, genotype: Genotype::AB }, male_maturation_days_ab);
    maturation_time.insert(&Fly { sex: Sex::male, genotype: Genotype::BB }, male_maturation_days_bb);

    // Proportions for weighted sampling with `choose_weighted`
    let proportion_sexes = vec![
        ProportionSexe{ sex: Sex::female, proportion: proportion_females },
        ProportionSexe{ sex: Sex::male, proportion: proportion_males },
    ];

    let proportion_genotypes = vec![
        ProportionGenotype{ genotype: Genotype::AA, proportion: proportion_aa },
        ProportionGenotype{ genotype: Genotype::AB, proportion: proportion_ab },
        ProportionGenotype{ genotype: Genotype::BB, proportion: proportion_bb },
    ];

    /// Generate first generation of eggs
    // Create initial fly and eggs vectors
    let mut individual_eggs: Vec<Fly> = Vec::new();
    let mut individual_eggs_previous: Vec<Fly> = Vec::new();
    let mut mature_adults: Vec<Fly> = Vec::new();
    let mut mature_females: Vec<Fly> = Vec::new();
    let mut mature_males: Vec<Fly> = Vec::new();

    let number_adults = number_eggs_per_generation as f64 * survival_global;
    let number_adults = number_adults as u32;

    let mut individual_adults = create_first_generation(
        &number_adults,
        &proportion_sexes,
        &proportion_genotypes
        );

    /// Iterate over generations
    for gen in 1..=number_generations {

        println!("\n= ( Generation: {:5} ) ===========", gen);

        // Egg survival to adulthood (except generation 1)
        println!("- Eggs");
        println!("  Number of eggs: {}", individual_eggs.len());
        println!("  Number of adults before: {}", individual_adults.len());

        if gen != 1 {
            // Egg survival by sex and genotype
            for egg in individual_eggs.iter() {

                let random_number: f64 = rng.gen();

                if random_number < *egg_survival.get(&egg).unwrap() {
                    individual_adults.push(*egg);
                }
            }
        }

        individual_eggs_previous = individual_eggs.to_vec();
        individual_eggs.clear();
        println!("  Number of adults after: {}", individual_adults.len());

        /// Survival to reproduction
        // Environment duration
        println!("- Environment");
        let environment_duration_min: f64 = environment_time - environment_time_variation;
        let environment_duration_max: f64 = environment_time + environment_time_variation;
        let environment_range = Uniform::from(environment_duration_min..environment_duration_max);

        for adult in individual_adults.iter() {
            // Environment duration
            let environment_duration = environment_range.sample(&mut rng);

            // Sample development time
            let adult_maturation = *maturation_time.get(&adult).unwrap();
            let adult_maturation_cv = adult_maturation * maturation_cv;
            let adult_maturation_lower = adult_maturation - adult_maturation_cv;
            let adult_maturation_upper = adult_maturation + adult_maturation_cv;
            let adult_maturation_range = Uniform::from(adult_maturation_lower..adult_maturation_upper);
            let m1 = adult_maturation_range.sample(&mut rng);
            let m2 = adult_maturation_range.sample(&mut rng);
            let m3 = adult_maturation_range.sample(&mut rng);
            let m = (m1 * m2 * m3).powf(1.0/3.0);

            // Decide survival
            if environment_duration >= m {
                mature_adults.push(Fly { sex: adult.sex, genotype: adult.genotype });

                if adult.sex == Sex::female {
                    mature_females.push(Fly { sex: adult.sex, genotype: adult.genotype });
                } else {
                    mature_males.push(Fly { sex: adult.sex, genotype: adult.genotype });
                }
            }
        }
        println!("  Number of adults after environment: {}", mature_adults.len());
        println!("  Number of females: {}", mature_females.len());
        println!("  Number of males: {}", mature_males.len());

        individual_adults.clear();

        /// TODO Reproduction
        println!("- Reproduction");
        // Count male genotypes
        let male_genotypes = mature_males.iter().collect::<Counter<_>>();
        let mut male_freq_dep: HashMap<&Genotype, f64> = HashMap::new();
        //male_freq_dep.insert(&Genotype::AA, f64::from(male_genotypes.get(&Fly {sex: Sex::male, genotype: Genotype::AA }).unwrap()) / (mature_males.len() as f64));




        let mut sum = 0;
        for (k, v) in male_genotypes.iter() {
            sum += v;
        }
        println!("Counter: {:?}", male_genotypes);
        println!("Sum: {}", sum);

        // for each female, pick a male randomly (weighted)
        // for each egg, pick sex (weighted) and genotype (from available males) randomly
        println!("- Frequency");
        // Frequency dependence

        /// TODO end simulation
        // if either AA or BB alleles get fixated, end simulation
        // report results
        let simulation_finished = false;
        if simulation_finished {
            break;
        }
    }

    println!("");
}
