#!/usr/bin/env sage
"""Script to generate small, lookup-table hamming code implementations"""
import itertools
from jinja2 import Environment
from sage.all import codes, GF, vector, ZZ, random_vector, channels


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
    info = {
        'name': name,
        'n': code.length(),
        'k': k,
        'generator': code.systematic_generator_matrix(),
        'parity_matrix': code.parity_check_matrix(),
    }

    max_error = code.decoder().maximum_error_weight()

    syndrome_map = {}
    for (error, _) in vectors_up_to(max_error, code.length()):
        he = tuple(code.parity_check_matrix() * error)
        syndrome_map[ZZ(he, base=2)] = tuple(error)

    info['syndrome_map'] = syndrome_map
    info['info_set'] = code.information_set()

    testcases = []
    seen = set()
    for i in range(min(2**k, 200)):
        m = None
        while m is None or m in seen:
            m = random_vector(GF(2), k)
            m.set_immutable()
        seen.add(m)

        encoded = code.encode(m)
        testcase = {
            'm': m,
            'encoded': encoded,
            'errorvecs': [],
        }
        for errors in range(1, max_error + 1):
            chan = channels.StaticErrorRateChannel(
                GF(2)**code.length(), errors)
            testcase['errorvecs'].append(tuple(chan.transmit(encoded)))

        testcases.append(testcase)

    info['testcases'] = testcases

    with open('syndrome_code_implementation.rs.j2', 'r') as templatefile:
        template = ENVIRONMENT.from_string(templatefile.read())
    with open('{name}/{name}_{n}_{k}.rs'.format(name=name.lower(),
                                         n=code.length(), k=k),
              'w') as outputfile:
        outputfile.write(template.render(**info))


if __name__ == "__main__":
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 2))
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 3))
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 4))
    generate_code_implementation("Hamming", codes.HammingCode(GF(2), 5))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 6))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 7))
