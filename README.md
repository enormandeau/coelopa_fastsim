# coelopa_fastsim

Coelopa inversion selection simulation in Rust

This code is released with the following paper :  
[Balancing selection via life-history trade-offs maintains an inversion polymorphism in a seaweed fly](https://www.biorxiv.org/content/10.1101/648584v1)

## Pre-requisites

In order to use `coelopa_fastsim`, you will need the [rust compiler](https://www.rust-lang.org/learn/get-started)

## Running

1. Prepare a parameter file. See examples used in the paper in the `02_info` folder.

2. Compile the code:
```
cargo build --release
```

3. Launch simulation with the `simulate` wrapper script. Pass it the name of the parameter file and an integer for the number of replicates to run per simulation:
```
./simulate 02_info/parameters_to_test_61_rust_freq_env_2019-05-06.csv 30
```

Use `./simulate_stop_when_fixated` if you want the run to end once only one allele remains.

## License

CC share-alike

<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons Licence" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br /><span xmlns:dct="http://purl.org/dc/terms/" property="dct:title">Coelopa Fastsim</span> by <span xmlns:cc="http://creativecommons.org/ns#" property="cc:attributionName">Eric Normandeau</span> is licensed under a  
<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.
