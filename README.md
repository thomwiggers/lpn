# Attacks on Learning Parity with Noise

This software package allows to construct attacks on LPN.
The `examples/` directory shows many examples of solving algorithms that solve various LPN instances using various (combinations of) reductions and solution-finding algorithms.
It also contains some example programs that try to find covering codes (mainly StGen codes) that have desirable properties.

## Usage

1. Get Rust and Cargo set up. You may need the nightly version (as of September 2018).
2. `cargo run --release --example bkw`

## Source material

Thom Wiggers. Solving LPN using Large Covering Codes. *Master's Thesis* Radboud University, 2018.

See also https://thomwiggers.nl/publication/msc-thesis/

Thom Wiggers and Simona Samardjiska.  Practically Solving LPN. _IEEE ISIT 2021_, 2021.

See also https://thomwiggers.nl/publication/lpn/

## References

* Blum, Kalai and Wasserman. Noise-tolerant learning, the parity problem, and the statistical query model. *Computing, 2000*, ACM, 2013.
* Levieil and Fouque. An improved LPN algorithm. *SCN 2006*, Springer 2018.
* Bogos, Tramer and Vaudenay. On solving LPN using BKW and variants – implementation and analysis. *Cryptography and Communications*, 2016.
* Bogos and Vaudenay. Optimization of LPN solving algorithms. *ASIACRYPT 2015*, Springer, 2016.
* Esser, Kübler and May. LPN Decoded. *CRYPTO 2017*, Springer, 2017.
* Samardjiska and Gligoroski. Approaching Maximum Embedding Efficiency on Small Covers Using Staricase-Generator Codes. *2015 IEEE International Symposium on Information Theory*, 2015.
