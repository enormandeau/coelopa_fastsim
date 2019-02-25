#![allow(warnings)]
//// Modules
//extern crate clap;
//use clap::{Arg, App, SubCommand};

extern crate rand;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand::Rng;

use std::collections::HashMap;
use std::io::prelude::*;
use std::vec::Vec;
//use std::io;
//use std::env;

//// Enums
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Sex {
    female,
    male,
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Sex::female => "female",
            Sex::male => "male",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Genotype {
    AA,
    AB,
    BB,
}

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

//// Structs
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Fly {
    sex: Sex,
    genotype: Genotype,
}

impl std::fmt::Display for Fly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}_{}", self.sex, self.genotype)
    }
}

#[derive(Debug, Copy, Clone)]
struct ProportionSexe {
    sex: Sex,
    proportion: f64,
}

#[derive(Debug, Copy, Clone)]
struct ProportionGenotype {
    genotype: Genotype,
    proportion: f64,
}

//// Functions
fn create_first_generation(
    n: &u32,
    psexes: &Vec<ProportionSexe>,
    pgenotypes: &Vec<ProportionGenotype>,
) -> Vec<Fly> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();

    // Create adults with random sex and genotype using proportions
    for _ in 0..*n {
        let sex = psexes
            .choose_weighted(&mut rng, |item| item.proportion)
            .unwrap()
            .sex;
        let genotype = pgenotypes
            .choose_weighted(&mut rng, |item| item.proportion)
            .unwrap()
            .genotype;

        v.push(Fly {
            sex: sex,
            genotype: genotype,
        });
    }

    v
}

fn allele_from_parent(p: &Fly) -> char {
    match p.genotype {
        Genotype::AA => 'A',
        Genotype::AB => {
            let mut rng = rand::thread_rng();
            let random_number: f64 = rng.gen();
            if random_number < 0.5 {
                'A'
            } else {
                'B'
            }
        }
        Genotype::BB => 'B',
    }
}

fn genotype_from_alleles(a1: char, a2: char) -> Genotype {
    //TODO debug
    if a1 == 'A' && a2 == 'A' {
        Genotype::AA
    } else if a1 == 'B' && a2 == 'B' {
        Genotype::BB
    } else if a1 == 'A' && a2 == 'B' {
        Genotype::AB
    } else {
        Genotype::AB
    }
}

fn get_genotype_proportions(samples: &Vec<Fly>) -> [f64; 3] {
    let mut genotype_counts: [i32; 3] = [0, 0, 0];
    let mut genotype_proportions: [f64; 3] = [0.0, 0.0, 0.0];

    for s in samples.iter() {
        let genotype = s.genotype;

        match genotype {
            Genotype::AA => genotype_counts[0] += 1,
            Genotype::AB => genotype_counts[1] += 1,
            Genotype::BB => genotype_counts[2] += 1,
        };
    }

    if samples.len() == 0 {
        genotype_proportions
    } else {
        for i in 0..3 {
            genotype_proportions[i] = genotype_counts[i] as f64 / samples.len() as f64;
        }

        genotype_proportions
    }
}

fn report_genotypes(samples: &Vec<Fly>, generation: &i32, lifestage: &str) {
    // TODO Report results to file
    let genotypes = get_genotype_proportions(&samples);
    println!(
        "{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}",
        generation,
        lifestage,
        samples.len(),
        genotypes[0],
        genotypes[1],
        genotypes[2]
    );
}

//// Main
fn main() {
    //// Parameters
    // TODO Parse arguments with `clap`
    let output_file = "output_file.txt";
    let number_generations = 10;
    let proportion_females = 0.5;
    let number_eggs_per_generation = 1000;
    let number_eggs_per_female = 50 as f64;

    let proportion_aa = 0.07;
    let proportion_bb = 0.44;

    let survival_global = 0.3;
    let survival_females_aa = 0.71;
    let survival_females_ab = 0.9;
    let survival_females_bb = 1.0;
    let survival_males_aa = 0.81;
    let survival_males_ab = 1.0;
    let survival_males_bb = 0.88;

    let female_eggs_aa = 1.0;
    let female_eggs_ab = 0.97;
    let female_eggs_bb = 0.87;

    let male_success_aa = 1.0;
    let male_success_ab = 0.55;
    let male_success_bb = 0.1;
    let male_freq_dep_coef = 0.1;

    let female_maturation_days = 8.8;
    let male_maturation_days_aa = 12.8;
    let male_maturation_days_ab = 10.3;
    let male_maturation_days_bb = 8.7;
    let maturation_cv = 0.5;

    let environment_time = 100.0;
    let environment_time_variation = 1.0;

    // Initialize random number generation
    let mut rng = rand::thread_rng();

    // Compute derived parameters
    let proportion_ab = 1.0 - proportion_aa - proportion_bb;
    let proportion_males = 1.0 - proportion_females;

    //// Survival and reproduction parameters
    // Survival from egg to adult
    let mut egg_survival: HashMap<&Fly, f64> = HashMap::new();
    egg_survival.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AA,
        },
        survival_females_aa,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AB,
        },
        survival_females_ab,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::BB,
        },
        survival_females_bb,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::AA,
        },
        survival_males_aa,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::AB,
        },
        survival_males_ab,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::BB,
        },
        survival_males_bb,
    );

    // Number of eggs per female genotype
    let mut female_eggs: HashMap<&Fly, f64> = HashMap::new();
    female_eggs.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AA,
        },
        number_eggs_per_female * female_eggs_aa,
    );
    female_eggs.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AB,
        },
        number_eggs_per_female * female_eggs_ab,
    );
    female_eggs.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::BB,
        },
        number_eggs_per_female * female_eggs_bb,
    );

    // Male reproductive sucess per genotype
    let mut male_success: HashMap<&Genotype, f64> = HashMap::new();
    male_success.insert( &Genotype::AA, male_success_aa,);
    male_success.insert( &Genotype::AB, male_success_ab,);
    male_success.insert( &Genotype::BB, male_success_bb,);

    // Maturation time
    let mut maturation_time: HashMap<&Fly, f64> = HashMap::new();
    maturation_time.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AA,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::AB,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::female,
            genotype: Genotype::BB,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::AA,
        },
        male_maturation_days_aa,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::AB,
        },
        male_maturation_days_ab,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::male,
            genotype: Genotype::BB,
        },
        male_maturation_days_bb,
    );

    // Proportions for weighted sampling with `choose_weighted`
    let proportion_sexes = vec![
        ProportionSexe {
            sex: Sex::female,
            proportion: proportion_females,
        },
        ProportionSexe {
            sex: Sex::male,
            proportion: proportion_males,
        },
    ];

    let proportion_genotypes = vec![
        ProportionGenotype {
            genotype: Genotype::AA,
            proportion: proportion_aa,
        },
        ProportionGenotype {
            genotype: Genotype::AB,
            proportion: proportion_ab,
        },
        ProportionGenotype {
            genotype: Genotype::BB,
            proportion: proportion_bb,
        },
    ];

    //// Generate first generation of eggs
    // Create initial fly and eggs vectors
    let mut individual_eggs: Vec<Fly> = Vec::new();
    let mut individual_eggs_previous: Vec<Fly> = Vec::new();
    let mut mature_adults: Vec<Fly> = Vec::new();
    let mut mature_females: Vec<Fly> = Vec::new();
    let mut mature_males: Vec<Fly> = Vec::new();

    let number_adults = number_eggs_per_generation as f64 * survival_global;
    let number_adults = number_adults as u32;

    let mut individual_adults =
        create_first_generation(&number_adults, &proportion_sexes, &proportion_genotypes);

    //// Iterate over generations
    println!("Gen\tStage\tNum\tAA\tAB\tBB");
    for gen in 1..=number_generations {
        // Egg survival to adulthood (except generation 1)
        //println!("\n\n-Eggs");
        report_genotypes(&individual_eggs, &gen, &"eggs");

        if gen != 1 {
            // Egg survival by sex and genotype
            individual_adults.clear();

            for egg in individual_eggs.iter() {
                //println!("Survival: {}", *egg_survival.get(&egg).unwrap() * survival_global);
                let random_number: f64 = rng.gen();

                if random_number < *egg_survival.get(&egg).unwrap() * survival_global {
                    individual_adults.push(*egg);
                }
            }
        }

        individual_eggs_previous = individual_eggs.to_vec();
        individual_eggs.clear();
        //report_genotypes(&individual_adults, &gen, &"adults");

        //// Survival to reproduction
        // Environment duration
        //println!("-Environment");
        let environment_duration_min: f64 = environment_time - environment_time_variation;
        let environment_duration_max: f64 = environment_time + environment_time_variation;
        let environment_range = Uniform::from(environment_duration_min..environment_duration_max);
        mature_adults.clear();
        mature_females.clear();
        mature_males.clear();

        for adult in individual_adults.iter() {
            // Environment duration
            let environment_duration = environment_range.sample(&mut rng);

            // Sample development time
            let adult_maturation = *maturation_time.get(&adult).unwrap();
            let adult_maturation_cv = adult_maturation * maturation_cv;
            let adult_maturation_lower = adult_maturation - adult_maturation_cv;
            let adult_maturation_upper = adult_maturation + adult_maturation_cv;
            let adult_maturation_range =
                Uniform::from(adult_maturation_lower..adult_maturation_upper);
            let m1 = adult_maturation_range.sample(&mut rng);
            let m2 = adult_maturation_range.sample(&mut rng);
            let m3 = adult_maturation_range.sample(&mut rng);
            let m = (m1 * m2 * m3).powf(1.0 / 3.0);

            // Decide survival
            if environment_duration >= m {
                mature_adults.push(Fly {
                    sex: adult.sex,
                    genotype: adult.genotype,
                });

                if adult.sex == Sex::female {
                    mature_females.push(Fly {
                        sex: adult.sex,
                        genotype: adult.genotype,
                    });

                } else {
                    mature_males.push(Fly {
                        sex: adult.sex,
                        genotype: adult.genotype,
                    });
                }
            }
        }
        report_genotypes(&mature_adults, &gen, &"adults");
        //report_genotypes(&mature_females, &gen, &"females");
        //report_genotypes(&mature_males, &gen, &"males");

        //// Reproduction
        //println!("-Reproduction");
        // Count male genotypes
        let number_mature_males = mature_males.len();
        let mut male_genotype_counts: HashMap<&Genotype, f64> = HashMap::new();
        male_genotype_counts.insert(&Genotype::AA, 0.0);
        male_genotype_counts.insert(&Genotype::AB, 0.0);
        male_genotype_counts.insert(&Genotype::BB, 0.0);

        for male in mature_males.iter() {
            *male_genotype_counts.entry(&male.genotype).or_insert(0.0) += 1.0;
        }

        // Make them into proportions
        let mut male_genotype_proportions: HashMap<&Genotype, f64> = HashMap::new();

        for (genotype, count) in male_genotype_counts.iter() {
            let proportion: f64 = count / (number_mature_males as f64);
            male_genotype_proportions.insert(genotype, proportion);
        }

        // Compute frequency dependent selection coefficient
        let mut male_freq_dep: HashMap<&Genotype, f64> = HashMap::new();
        let proportion_male_aa = male_genotype_proportions.get(&Genotype::AA).unwrap_or(&0.0);

        male_freq_dep.insert(&Genotype::AA, 1.0);
        male_freq_dep.insert(
            &Genotype::AB,
            1.0 - male_freq_dep_coef * (1.0 - proportion_male_aa) / 2.0,
        );
        male_freq_dep.insert(
            &Genotype::BB,
            1.0 - male_freq_dep_coef * (1.0 - proportion_male_aa),
        );

        // Compute male genotype probabilities for mating
        //TODO Bug: does not behave has python simulation script
        let mut male_genotype_probabilities: HashMap<&Genotype, f64> = HashMap::new();
        male_genotype_probabilities.insert(&Genotype::AA,
                                           male_genotype_proportions.get(&Genotype::AA).unwrap() *
                                           male_success.get(&Genotype::AA).unwrap() *
                                           male_genotype_proportions.get(&Genotype::AA).unwrap());

        male_genotype_probabilities.insert(&Genotype::AB,
                                           male_genotype_proportions.get(&Genotype::AB).unwrap() *
                                           male_success.get(&Genotype::AB).unwrap() *
                                           male_genotype_proportions.get(&Genotype::AB).unwrap());

        male_genotype_probabilities.insert(&Genotype::BB,
                                           male_genotype_proportions.get(&Genotype::BB).unwrap() *
                                           male_success.get(&Genotype::BB).unwrap() *
                                           male_genotype_proportions.get(&Genotype::BB).unwrap());
        // Normalize probabilities to 1.0
        let total_coefficient: f64 = male_genotype_probabilities.values().sum();

        let proportion_genotypes = vec![
            ProportionGenotype {
                genotype: Genotype::AA,
                proportion: male_genotype_probabilities.get(&Genotype::AA).unwrap() / total_coefficient,
            },
            ProportionGenotype {
                genotype: Genotype::AB,
                proportion: male_genotype_probabilities.get(&Genotype::AB).unwrap() / total_coefficient,
            },
            ProportionGenotype {
                genotype: Genotype::BB,
                proportion: male_genotype_probabilities.get(&Genotype::BB).unwrap() / total_coefficient,
            },
        ];

        for female in mature_females.iter() {
            // Pick weighted random mate genotype
            let random_male_genotype = proportion_genotypes
                .choose_weighted(&mut rng, |item| item.proportion)
                .unwrap()
                .genotype;

            let num_eggs = *female_eggs.get(female).unwrap() as u32;

            for egg in 1..=num_eggs {
                // Get female allele
                let female_allele = allele_from_parent(&female);

                // Get male allele
                let male_allele = allele_from_parent(&Fly {
                    sex: Sex::male,
                    genotype: random_male_genotype,
                });

                // Create egg
                let genotype = genotype_from_alleles(female_allele, male_allele);
                let random_number: f64 = rng.gen();
                let sex = if random_number < proportion_females {
                    Sex::female
                } else {
                    Sex::male
                };

                individual_eggs.push(Fly {
                    sex: sex,
                    genotype: genotype,
                });
            }
        }
        //report_genotypes(&individual_eggs, &gen, &"eggs");

        // Shuffle and keep number_eggs_per_generation eggs
        rng.shuffle(&mut individual_eggs);
        individual_eggs = individual_eggs[..number_eggs_per_generation].to_vec();
        //report_genotypes(&individual_eggs, &gen, &"eggs");

        // if either AA or BB alleles get fixated, end simulation
        let mut count_AA = 0;
        let mut count_AB = 0;
        let mut count_BB = 0;
        for egg in individual_eggs.iter() {
            match egg.genotype {
                Genotype::AA => count_AA += 1,
                Genotype::AB => count_AB += 1,
                Genotype::BB => count_BB += 1,
            }
        }

        //println!("AA: {}, AB: {}, BB: {}", count_AA, count_AB, count_BB);

        let num_individual_eggs = individual_eggs.len();
        if (count_AA == 0 && count_AB == 0) || (count_BB == 0 && count_AB == 0) {
            println!("Alleles fixated on generation {}!", gen);
            break;
        }
        println!("")
    }
}
