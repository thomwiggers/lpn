#!/usr/bin/env sage
"""Script to generate small, lookup-table hamming code implementations"""
from jinja2 import Environment
from sage.all import codes, GF, VectorSpace


def boollist(lst):
    """Convert (1, 0) into '[true, false]'"""
    return (', '.join(map(lambda x: str(bool(x)), lst))).lower()

def intlist(lst):
    return (', '.join(map(lambda x: str(x), lst)))


ENVIRONMENT = Environment()
ENVIRONMENT.filters['boollist'] = boollist
ENVIRONMENT.filters['intlist'] = intlist

def bools_to_binvec(bools):
    u64s = list(0 for _ in range(len(bools)/64 + (1 if len(bools) % 64 != 0 else 0)))
    for (i, bit) in enumerate(bools):
        u64s[i/64] |= bool(bit) << (i % 64)
    return u64s


def generate_hamming_implementation(exponent):
    """Generate a hamming code"""
    code = codes.HammingCode(GF(2), exponent)
    info = {
        'n': code.length(),
        'k': code.dimension(),
        'generator': [bools_to_binvec(row) for row in code.systematic_generator_matrix()],
        'parity_matrix': [bools_to_binvec(row) for row in code.parity_check_matrix()],
    }

    info['syndromes'] = [bools_to_binvec(code.decode_to_message(v))
                         for v in VectorSpace(GF(2), code.length())]
    info['syndrome_itemlen'] = len(info['syndromes'][0])

    info['encodings'] = [bools_to_binvec(code.encode(v))
                         for v in VectorSpace(GF(2), code.dimension())]
    info['encoding_itemlen'] = len(info['encodings'][0])

    with open('hamming_n_k.rs.j2', 'r') as templatefile:
        template = ENVIRONMENT.from_string(templatefile.read())
    with open('hamming_{n}_{k}.rs'.format(n=code.length(), k=code.dimension()),
              'w') as outputfile:
        outputfile.write(template.render(**info))


if __name__ == "__main__":
    generate_hamming_implementation(2)
    generate_hamming_implementation(3)
    generate_hamming_implementation(4)
