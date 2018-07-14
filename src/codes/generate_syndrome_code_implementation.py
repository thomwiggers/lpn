#!/usr/bin/env sage
"""Script to generate small, lookup-table hamming code implementations"""
from __future__ import print_function
import itertools
from jinja2 import Environment
from sage.all import codes, GF, vector, ZZ, random_vector, channels, matrix


def boollist(lst):
    """Convert (1, 0) into '[true, false]'"""
    return (', '.join(map(lambda x: str(bool(x)), lst))).lower()


ENVIRONMENT = Environment()
ENVIRONMENT.filters['boollist'] = boollist


def vectors_up_to(weight, n):
    assert weight <= n, "Weight should be less than n"

    poses = list(range(n))
    for w in range(weight+1):
        for combination in itertools.combinations(poses, w):
            v = vector(GF(2), n)
            for i in combination:
                v[i] = 1
            yield (v, w)




def generate_code_implementation(name, code):
    """Generate a code implementation"""
    k = code.dimension()
    cs, _p = code.standard_form()
    info = {
        'name': name,
        'n': code.length(),
        'k': k,
        'generator': cs.systematic_generator_matrix(),
        'parity_matrix': cs.parity_check_matrix(),
    }

    max_error = code.decoder().maximum_error_weight()

    syndrome_map = {}
    for (he, error) in cs.decoder().syndrome_table().items():
        syndrome_map[ZZ(list(he), base=2)] = tuple(error)

    info['syndrome_map'] = syndrome_map
    info['info_set'] = cs.information_set()

    testcases = []
    if 'might-error' in cs.decoder().decoder_type():
        max_error -= 3
    for _ in range(200):
        randvec = random_vector(GF(2), code.length())
        codeword = cs.decode_to_code(randvec)
        testcase = {
            'randvec': randvec,
            'codeword': codeword,
        }
        testcases.append(testcase)

    info['testcases'] = testcases

    with open('syndrome_code_implementation.rs.j2', 'r') as templatefile:
        template = ENVIRONMENT.from_string(templatefile.read())
    with open('{name}/{name}_{n}_{k}.rs'.format(name=name.lower(),
                                                n=code.length(), k=k),
              'w') as outputfile:
        outputfile.write(template.render(**info))


if __name__ == "__main__":
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 2))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 3))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 4))
    print("Hamming code 5")
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 5))
    print("Hamming code 6")
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 6))

    print("Golay code")
    generate_code_implementation("Golay", codes.GolayCode(GF(2), extended=False))
    print("Golay code ext")
    generate_code_implementation("Golay", codes.GolayCode(GF(2), extended=True))
    print("Hamming code 7")
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 7))

if False:
    print("Bogos code [18, 6]")
    generate_code_implementation(
        "Bogosrnd",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1],
                    [0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0],
                    [1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0],
                    [0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
                    [0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1],
                ])))
    print("Bogos code [19, 7]")
    generate_code_implementation(
        "Bogosrnd",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0],
                    [1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0],
                    [0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0],
                    [0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1],
                    [0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1],
                    [0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                ])))
    print("Bogos code [19, 6]")
    generate_code_implementation(
        "Bogosrnd",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1],
                    [1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
                    [1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
                    [1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0],
                    [1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0],
                ])))
