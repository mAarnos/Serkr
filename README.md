# Serkr

[![Build Status](https://travis-ci.org/mAarnos/Serkr.svg?branch=master)](https://travis-ci.org/mAarnos/Serkr)

Serkr is an experimental automated theorem prover for first order logic with equality. Internally, the proof search uses pure equational logic and performs inferences with a version of the superposition calculus. It also contains a number of fairly standard simplification techniques, e.g. rewriting, subsumption, tautology deletion and deletion of useless literals.

Note that the development of Serkr is still in its early stages. As such, it has a lot of rough edges, the documentation and code quality are a bit wonky, and it is missing some rather important stuff which a mature ATP system would have. Hopefully this will change in the future.

## Usage

Serkr analyzes problems files in the TPTP format. Some (mostly easy and CNF) example problems can be found, unsurprisingly, in the folder 'examples'. More information on TPTP can be found at http://www.cs.miami.edu/~tptp/.

Typical usage might be something like this:

    cargo build --release
    target\release\serkr --help
    target\release\serkr examples\RNG009-5.p
    target\release\serkr examples\CAT002-3.p
    target\release\serkr --time_limit=30 examples\ANA002-2.p
    
## License

The license is GPLv3. See LICENSE for more details.
