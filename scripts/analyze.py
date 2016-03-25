from argparse import ArgumentParser
from subprocess import check_output
from sys import argv
from os.path import join, getsize
from os import walk
from multiprocessing import Pool
from re import match

def parse_args():
    parser = ArgumentParser(description='Analyze TPTP problems en masse')
    parser.add_argument('-d', dest='dir', type=str, required=True,
                        help='Analyze all files in this directory and its subdirectories')
    parser.add_argument('-t', dest='time', type=int, default=10,
                        help='The amount of time to analyze each file for, in seconds')
    parser.add_argument('-p', dest='processes', type=int, default=1,
                        help='The amount of processes to use for analyzing')
    parser.add_argument('--cnf', dest='analyze_cnf', action='store_true',
                        help='Analyze CNF problems')
    parser.add_argument('--fof', dest='analyze_fof', action='store_true',
                        help='Analyze FOF problems. Currently not functional.')
    args = parser.parse_args()
    if not (args.analyze_cnf or args.analyze_fof):
        parser.error('One of --cnf or --fof must be given')
    return args

def is_valid_problem_status_cnf(s):
    return s == "Unknown" or s == "Unsatisfiable" or s == "Satisfiable" or s == "Open"

def is_valid_problem_status_fof(s):
    return is_valid_problem_status_cnf(s) or s == "CounterSatisfiable" or s == "Theorem"

def is_valid_problem_status(s, cnf):
    if cnf:
        return is_valid_problem_status_cnf(s)
    else:
        return is_valid_problem_status_fof(s)

def find_status_of_problem(path, cnf):
    with open(path) as f:
        for line in f:
            line = ''.join(line.split())
            if line.startswith("%Status:"):
                status = line[8:]           
                if not is_valid_problem_status(status, cnf):
                    raise AssertionError("Status not known")
                return status
    raise AssertionError("Status should always be possible to find")

def is_valid_prover_status_cnf(s):
    return s == "Timeout" or s == "Unsatisfiable" or s == "Satisfiable"

def is_valid_prover_status_fof(s):
    return is_valid_prover_status_cnf(s) or s == "Theorem" or s == "CounterSatisfiable"

def is_valid_prover_status(s, cnf):
    if cnf:
        return is_valid_prover_status_cnf(s)
    else:
        return is_valid_prover_status_fof(s)

def find_status_of_prover(s, cnf):
    for line in s.split('\n'):
        m = match("% SZS status ([a-zA-Z]+) .*", line)
        if m and is_valid_prover_status(m.group(1), cnf):
            return m.group(1)
    raise AssertionError("Prover status should always be possible to find")

args = parse_args()

def analyze_file(data):
    s = data[0]
    cnf = data[1]
    print(s)
    problem_status = find_status_of_problem(s, cnf)
    output = check_output(["serkr.exe", "-t=%d" % args.time, s], universal_newlines=True)
    prover_status = find_status_of_prover(output, cnf)

    if not prover_status == "Timeout":
        if not prover_status == problem_status:
            raise AssertionError("Prover disagrees with TPTP problem status, take note!")
        return 1

    return 0

def main():
    file_names = []
    for root, dirs, files in walk(args.dir):
        for file in files:
            if file.endswith("p"):
                if args.analyze_cnf and file[6] in {'-'}:
                    file_names.append((join(root, file), True))
                elif args.analyze_fof and file[6] in {'+'}:
                    file_names.append((join(root, file), False))

    pool = Pool(args.processes)
    results = pool.map(analyze_file, file_names)
    print("Proved %d out of %d" % (sum(results), len(file_names)))

if __name__ == "__main__":
    main()