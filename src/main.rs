//#![allow(warnings)]
//// Modules
extern crate clap;
use clap::{App, Arg};

extern crate rand;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process;
use std::vec::Vec;

//// Enums
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Sex {
    Female,
    Male,
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Sex::Female => "female",
            Sex::Male => "male",
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Lifestage {
    Egg,
    Adult,
}

//// Structs
impl std::fmt::Display for Lifestage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Lifestage::Egg => "egg",
            Lifestage::Adult => "adult",
        };
        write!(f, "{}", printable)
    }
}

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
    let mut samples = Vec::new();

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

        samples.push(Fly {
            sex: sex,
            genotype: genotype,
        });
    }

    samples
}

fn allele_from_parent(p: &Fly) -> char {
    // Return a random allele from a parent
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
    // Create a Genotype from two alleles passed as chars
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
    // Return array of 3 values containing the proportion of
    // AA, AB, and BB genotypes
    let mut genotype_counts = [0, 0, 0];
    let mut genotype_proportions = [0.0, 0.0, 0.0];

    for s in samples.iter() {
        match s.genotype {
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

fn report_genotypes(
    // Print genotype proportions on screen and write them to file
    samples: &Vec<Fly>,
    generation: &u32,
    lifestage: &Lifestage,
    outfile: &mut File,
    quiet: &bool,
) {
    let genotypes = get_genotype_proportions(&samples);
    if !quiet {
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

    // Report to file
    // Eggs come first on each line (no \n) and then adults
    match lifestage {
        Lifestage::Egg => outfile.write(
            format!(
                "{},{},{},{},",
                generation, genotypes[0], genotypes[1], genotypes[2]
            )
            .as_bytes(),
        ),

        Lifestage::Adult => outfile
            .write(format!("{},{},{}\n", genotypes[0], genotypes[1], genotypes[2]).as_bytes()),
    }
    .unwrap();
}

//// Main
fn main() {
    // Get parameters with Clap
    let matches = App::new("Coelopa FastSim")
        .version("v0.1")
        .author("Eric Normandeau")
        .about("Coelopa Genomic Inversion Simulator")
        // Parameter names have underscores but 'long' have
        // dashes for compatibility with Python arguments and
        // so the automatic simulation script can launch both
        // without too many modifications
        // Can't remember what this "slop" option does...
        //.arg(Arg::with_name("slop").multiple(true).last(true))
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .short("o")
                .value_name("STRING")
                .help("Name of output file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("experiment_name")
                .long("experiment-name")
                .value_name("STRING")
                .help("Name of output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number_generations")
                .long("number-generations")
                .value_name("INT")
                .help("Duration of simulation in number of generations [>= 0] (default=5)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("proportion_females")
                .long("proportion-females")
                .value_name("FLOAT")
                .help("Proportion of females in eggs [0, 1] (default=0.5)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number_eggs_per_generation")
                .long("number-eggs-per-generation")
                .value_name("INT")
                .help("Number of eggs to keep per generation [>= 0] (default=1000)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number_eggs_per_female")
                .long("number-eggs-per-female")
                .value_name("INT")
                .help("Number of eggs per female per generation [>= 0] (default=50)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("proportion_aa")
                .long("proportion-aa")
                .value_name("FLOAT")
                .help("Starting proportion of AA individuals [0, 1] (default=0.07)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("proportion_bb")
                .long("proportion-bb")
                .value_name("FLOAT")
                .help("Starting proportion of BB individuals [0, 1] (default=0.44)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_global")
                .long("survival-global")
                .value_name("FLOAT")
                .help("Basic proportion of surviving individuals [0, 1] (default=0.3)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_females_aa")
                .long("survival-females-aa")
                .value_name("FLOAT")
                .help("Relative survival rate of AA female eggs [0, 1] (default=0.71)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_females_ab")
                .long("survival-females-ab")
                .value_name("FLOAT")
                .help("Relative survival rate of AB female eggs [0, 1] (default=0.9)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_females_bb")
                .long("survival-females-bb")
                .value_name("FLOAT")
                .help("Relative survival rate of BB female eggs [0, 1] (default=1.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_males_aa")
                .long("survival-males-aa")
                .value_name("FLOAT")
                .help("Relative survival rate of AA male eggs [0, 1] (default=0.81)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_males_ab")
                .long("survival-males-ab")
                .value_name("FLOAT")
                .help("Relative survival rate of AB male eggs [0, 1] (default=1.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("survival_males_bb")
                .long("survival-males-bb")
                .value_name("FLOAT")
                .help("Relative survival rate of BB male eggs [0, 1] (default=0.88)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("female_eggs_aa")
                .long("female-eggs-aa")
                .value_name("FLOAT")
                .help("Relative number of eggs for AA females [0, 1] (default=1.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("female_eggs_ab")
                .long("female-eggs-ab")
                .value_name("FLOAT")
                .help("Relative number of eggs for AA females [0, 1] (default=0.97)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("female_eggs_bb")
                .long("female-eggs-bb")
                .value_name("FLOAT")
                .help("Relative number of eggs for AA females [0, 1] (default=0.87)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_success_aa")
                .long("male-success-aa")
                .value_name("FLOAT")
                .help("Relative reproductive success for AA males [0, 1] (default=1.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_success_ab")
                .long("male-success-ab")
                .value_name("FLOAT")
                .help("Relative reproductive success for AB males [0, 1] (default=0.55)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_success_bb")
                .long("male-success-bb")
                .value_name("FLOAT")
                .help("Relative reproductive success for BB males [0, 1] (default=0.1)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_freq_dep_coef")
                .long("male-freq-dep-coef")
                .value_name("FLOAT")
                .help("Intensity of frequency dependence on males [0, 1] (default=0.1)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("female_maturation_days")
                .long("female-maturation-days")
                .value_name("FLOAT")
                .help("Number of days before females are mature [0, 1] (default=8.8)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_maturation_days_aa")
                .long("male-maturation-days-aa")
                .value_name("FLOAT")
                .help("Number of days before AA males are mature [0, 1] (default=12.8)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_maturation_days_ab")
                .long("male-maturation-days-ab")
                .value_name("FLOAT")
                .help("Number of days before AB males are mature [0, 1] (default=10.3)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("male_maturation_days_bb")
                .long("male-maturation-days-bb")
                .value_name("FLOAT")
                .help("Number of days before BB males are mature [0, 1] (default=8.7)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("maturation_cv")
                .long("maturation-cv")
                .value_name("FLOAT")
                .help("Variation coefficient for maturation time [0, 1] (default=0.5)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("environment_time")
                .long("environment-time")
                .value_name("FLOAT")
                .help("Duration of breeding environment in days [0, 1] (default=10.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("environment_time_variation")
                .long("environment-time-variation")
                .value_name("FLOAT")
                .help("Deviation on breeding environment duration [0, 1] (default=1.0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop_when_fixated")
                .long("stop-when-fixated")
                .value_name("BOOL")
                .help("Stop simulation if only one allele remains (default=false)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .value_name("BOOL")
                .help("Do no report progress on screen (default=false)")
                .takes_value(false),
        )
        .get_matches();

    // Convert parameters to wanted types
    let output_file = matches
        .value_of("output_file")
        .expect("Cannot create output file");

    let experiment_name = matches
        .value_of("experiment_name")
        .unwrap_or("unnamed_experiment");

    let number_generations = matches
        .value_of("number_generations")
        .unwrap_or("5")
        .parse::<u32>()
        .unwrap();

    let proportion_females = matches
        .value_of("proportion_females")
        .unwrap_or("0.5")
        .parse::<f64>()
        .unwrap();

    let number_eggs_per_generation = matches
        .value_of("number_eggs_per_generation")
        .unwrap_or("1000")
        .parse::<usize>()
        .unwrap();

    let number_eggs_per_female = matches
        .value_of("number_eggs_per_female")
        .unwrap_or("50")
        .parse::<f64>()
        .unwrap();

    let proportion_aa = matches
        .value_of("proportion_aa")
        .unwrap_or("0.07")
        .parse::<f64>()
        .unwrap();

    let proportion_bb = matches
        .value_of("proportion_bb")
        .unwrap_or("0.44")
        .parse::<f64>()
        .unwrap();

    let survival_global = matches
        .value_of("survival_global")
        .unwrap_or("0.3")
        .parse::<f64>()
        .unwrap();

    let survival_females_aa = matches
        .value_of("survival_females_aa")
        .unwrap_or("0.71")
        .parse::<f64>()
        .unwrap();

    let survival_females_ab = matches
        .value_of("survival_females_ab")
        .unwrap_or("0.9")
        .parse::<f64>()
        .unwrap();

    let survival_females_bb = matches
        .value_of("survival_females_bb")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();

    let survival_males_aa = matches
        .value_of("survival_males_aa")
        .unwrap_or("0.81")
        .parse::<f64>()
        .unwrap();

    let survival_males_ab = matches
        .value_of("survival_males_ab")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();

    let survival_males_bb = matches
        .value_of("survival_males_bb")
        .unwrap_or("0.88")
        .parse::<f64>()
        .unwrap();

    let female_eggs_aa = matches
        .value_of("female_eggs_aa")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();

    let female_eggs_ab = matches
        .value_of("female_eggs_ab")
        .unwrap_or("0.97")
        .parse::<f64>()
        .unwrap();

    let female_eggs_bb = matches
        .value_of("female_eggs_bb")
        .unwrap_or("0.87")
        .parse::<f64>()
        .unwrap();

    let male_success_aa = matches
        .value_of("male_success_aa")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();

    let male_success_ab = matches
        .value_of("male_success_ab")
        .unwrap_or("0.55")
        .parse::<f64>()
        .unwrap();

    let male_success_bb = matches
        .value_of("male_success_bb")
        .unwrap_or("0.1")
        .parse::<f64>()
        .unwrap();

    let male_freq_dep_coef = matches
        .value_of("male_freq_dep_coef")
        .unwrap_or("0.1")
        .parse::<f64>()
        .unwrap();

    let female_maturation_days = matches
        .value_of("female_maturation_days")
        .unwrap_or("8.8")
        .parse::<f64>()
        .unwrap();

    let male_maturation_days_aa = matches
        .value_of("male_maturation_days_aa")
        .unwrap_or("12.8")
        .parse::<f64>()
        .unwrap();

    let male_maturation_days_ab = matches
        .value_of("male_maturation_days_ab")
        .unwrap_or("10.3")
        .parse::<f64>()
        .unwrap();

    let male_maturation_days_bb = matches
        .value_of("male_maturation_days_bb")
        .unwrap_or("8.7")
        .parse::<f64>()
        .unwrap();

    let maturation_cv = matches
        .value_of("maturation_cv")
        .unwrap_or("0.5")
        .parse::<f64>()
        .unwrap();

    let environment_time = matches
        .value_of("environment_time")
        .unwrap_or("10.0")
        .parse::<f64>()
        .unwrap();

    let environment_time_variation = matches
        .value_of("environment_time_variation")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();

    let stop_when_fixated = match matches.occurrences_of("stop_when_fixated") {
        0 => false,
        1 => true,
        _ => false,
    };

    let quiet = match matches.occurrences_of("quiet") {
        0 => false,
        1 => true,
        _ => false,
    };

    // Compute derived parameters
    let proportion_ab = 1.0 - proportion_aa - proportion_bb;
    let proportion_males = 1.0 - proportion_females;

    // Initialize random number generation
    let mut rng = rand::thread_rng();

    //// Survival and reproduction parameters
    // Survival from egg to adult
    let mut egg_survival: HashMap<&Fly, f64> = HashMap::new();
    egg_survival.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AA,
        },
        survival_females_aa,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AB,
        },
        survival_females_ab,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::BB,
        },
        survival_females_bb,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::AA,
        },
        survival_males_aa,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::AB,
        },
        survival_males_ab,
    );
    egg_survival.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::BB,
        },
        survival_males_bb,
    );

    // Number of eggs per female genotype
    let mut female_eggs: HashMap<&Fly, f64> = HashMap::new();
    female_eggs.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AA,
        },
        number_eggs_per_female * female_eggs_aa,
    );
    female_eggs.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AB,
        },
        number_eggs_per_female * female_eggs_ab,
    );
    female_eggs.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::BB,
        },
        number_eggs_per_female * female_eggs_bb,
    );

    // Male reproductive sucess per genotype
    let mut male_success: HashMap<&Genotype, f64> = HashMap::new();
    male_success.insert(&Genotype::AA, male_success_aa);
    male_success.insert(&Genotype::AB, male_success_ab);
    male_success.insert(&Genotype::BB, male_success_bb);

    // Maturation time
    let mut maturation_time: HashMap<&Fly, f64> = HashMap::new();
    maturation_time.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AA,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::AB,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::Female,
            genotype: Genotype::BB,
        },
        female_maturation_days,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::AA,
        },
        male_maturation_days_aa,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::AB,
        },
        male_maturation_days_ab,
    );
    maturation_time.insert(
        &Fly {
            sex: Sex::Male,
            genotype: Genotype::BB,
        },
        male_maturation_days_bb,
    );

    // Proportions for weighted sampling with `choose_weighted`
    let proportion_sexes = vec![
        ProportionSexe {
            sex: Sex::Female,
            proportion: proportion_females,
        },
        ProportionSexe {
            sex: Sex::Male,
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
    //let mut individual_eggs_previous: Vec<Fly> = Vec::new();
    let mut mature_adults: Vec<Fly> = Vec::new();
    let mut mature_females: Vec<Fly> = Vec::new();
    let mut mature_males: Vec<Fly> = Vec::new();

    let number_adults = number_eggs_per_generation as f64 * survival_global;
    let number_adults = number_adults as u32;

    let mut individual_adults =
        create_first_generation(&number_adults, &proportion_sexes, &proportion_genotypes);

    // Create output file and write header
    let mut outfile = File::create(output_file).expect("Cannot creat file");
    outfile
        .write(b"Generation,eggAA,eggAB,eggBB,adultAA,adultAB,adultBB\n")
        .expect("Cannot write to file");

    //// Iterate over generations
    if !quiet {
        println!("#Gen\tStage\tNum\tAA\tAB\tBB");
    }

    for gen in 1..=number_generations {
        // Egg survival to adulthood (except generation 1)
        if gen != 1 {
            // Egg survival by sex and genotype
            individual_adults.clear();

            for egg in individual_eggs.iter() {
                let random_number: f64 = rng.gen();

                if random_number < *egg_survival.get(&egg).unwrap() * survival_global {
                    individual_adults.push(*egg);
                }
            }
        }

        // Report egg genotypes and cleanup
        report_genotypes(
            &individual_eggs,
            &gen,
            &Lifestage::Egg,
            &mut outfile,
            &quiet,
        );
        //individual_eggs_previous = individual_eggs.to_vec();
        individual_eggs.clear();

        //// Survival to reproduction
        // Environment duration
        mature_adults.clear();
        mature_females.clear();
        mature_males.clear();
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

                if adult.sex == Sex::Female {
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

        // Report adult genotypes
        report_genotypes(
            &mature_adults,
            &gen,
            &Lifestage::Adult,
            &mut outfile,
            &quiet,
        );

        //// Reproduction
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

        // Compute male genotype probabilities for mating as function of
        // genotype proportions, reproduction success of each genotype, and
        // frequency dependent selection
        let mut male_genotype_probabilities: HashMap<&Genotype, f64> = HashMap::new();
        male_genotype_probabilities.insert(
            &Genotype::AA,
            male_genotype_proportions.get(&Genotype::AA).unwrap()
                * male_success.get(&Genotype::AA).unwrap()
                * male_freq_dep.get(&Genotype::AA).unwrap(),
        );

        male_genotype_probabilities.insert(
            &Genotype::AB,
            male_genotype_proportions.get(&Genotype::AB).unwrap()
                * male_success.get(&Genotype::AB).unwrap()
                * male_freq_dep.get(&Genotype::AB).unwrap(),
        );

        male_genotype_probabilities.insert(
            &Genotype::BB,
            male_genotype_proportions.get(&Genotype::BB).unwrap()
                * male_success.get(&Genotype::BB).unwrap()
                * male_freq_dep.get(&Genotype::BB).unwrap(),
        );

        // Normalize probabilities to 1.0
        let total_coefficient: f64 = male_genotype_probabilities.values().sum();

        let proportion_genotypes = vec![
            ProportionGenotype {
                genotype: Genotype::AA,
                proportion: male_genotype_probabilities.get(&Genotype::AA).unwrap()
                    / total_coefficient,
            },
            ProportionGenotype {
                genotype: Genotype::AB,
                proportion: male_genotype_probabilities.get(&Genotype::AB).unwrap()
                    / total_coefficient,
            },
            ProportionGenotype {
                genotype: Genotype::BB,
                proportion: male_genotype_probabilities.get(&Genotype::BB).unwrap()
                    / total_coefficient,
            },
        ];

        // Stop simulation if one of proportion_genotypes is NaN
        for p in proportion_genotypes.iter() {
            if p.proportion.is_nan() {
                print!("{}\t", experiment_name);
                report_genotypes(
                    &mature_adults,
                    &gen,
                    &Lifestage::Adult,
                    &mut outfile,
                    &false,
                );
                process::exit(0);
            }
        }

        // Each female reproduces with one male
        for female in mature_females.iter() {
            // Pick weighted random mate genotype
            let random_male_genotype = proportion_genotypes
                .choose_weighted(&mut rng, |item| item.proportion)
                .unwrap()
                .genotype;

            // Determine number of eggs to lay
            let num_eggs = *female_eggs.get(female).unwrap() as u32;

            for _ in 1..=num_eggs {
                // Get one female allele
                let female_allele = allele_from_parent(&female);

                // Get one male allele
                let male_allele = allele_from_parent(&Fly {
                    sex: Sex::Male,
                    genotype: random_male_genotype,
                });

                // Create egg from parent genotypes
                let genotype = genotype_from_alleles(female_allele, male_allele);
                let random_number: f64 = rng.gen();

                let sex = if random_number < proportion_females {
                    Sex::Female
                } else {
                    Sex::Male
                };

                individual_eggs.push(Fly {
                    sex: sex,
                    genotype: genotype,
                });
            }
        }

        // Shuffle and keep number_eggs_per_generation eggs
        individual_eggs.shuffle(&mut rng);
        let number_eggs = individual_eggs.len();
        let mut keep_n_eggs = number_eggs_per_generation;

        if number_eggs < number_eggs_per_generation {
            keep_n_eggs = number_eggs;
        }

        individual_eggs = individual_eggs[..keep_n_eggs].to_vec();

        // Count genotypes to decide if we end the simulation
        // because alleles are fixated
        if stop_when_fixated {
            let mut count_aa = 0;
            let mut count_ab = 0;
            let mut count_bb = 0;

            for egg in individual_eggs.iter() {
                match egg.genotype {
                    Genotype::AA => count_aa += 1,
                    Genotype::AB => count_ab += 1,
                    Genotype::BB => count_bb += 1,
                }
            }

            // End simulation if either AA or BB alleles get fixated
            if (count_aa == 0 && count_ab == 0) || (count_bb == 0 && count_ab == 0) {
                //println!("Alleles fixated on generation {}!", gen);
                print!("{}\t", experiment_name);
                report_genotypes(
                    &individual_eggs,
                    &gen,
                    &Lifestage::Egg,
                    &mut outfile,
                    &false,
                );
                break;
            } else if gen == number_generations {
                print!("{}\t", experiment_name);
                report_genotypes(
                    &mature_adults,
                    &gen,
                    &Lifestage::Adult,
                    &mut outfile,
                    &false,
                );
            }
        }
    }
}
