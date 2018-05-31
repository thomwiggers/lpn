#!/usr/bin/env sage
"""Script to generate small, lookup-table hamming code implementations"""
from jinja2 import Environment
from sage.all import codes, GF, VectorSpace


def boollist(lst):
    """Convert (1, 0) into '[true, false]'"""
    return (', '.join(map(lambda x: str(bool(x)), lst))).lower()


ENVIRONMENT = Environment()
ENVIRONMENT.filters['boollist'] = boollist


def generate_hamming_implementation(exponent):
    """Generate a hamming code"""
    code = codes.HammingCode(GF(2), exponent)
    info = {
        'n': code.length(),
        'k': code.dimension(),
        'generator': code.systematic_generator_matrix(),
        'parity_matrix': code.parity_check_matrix(),
    }

    info['syndromes'] = [code.decode_to_message(v)
                         for v in VectorSpace(GF(2), code.length())]
    info['encodings'] = [code.encode(v)
                         for v in VectorSpace(GF(2), code.dimension())]

    with open('hamming_n_k.rs.j2', 'r') as templatefile:
        template = ENVIRONMENT.from_string(templatefile.read())
    with open('hamming_{n}_{k}.rs'.format(n=code.length(), k=code.dimension()),
              'w') as outputfile:
        outputfile.write(template.render(**info))


if __name__ == "__main__":
    generate_hamming_implementation(2)
    generate_hamming_implementation(3)
    generate_hamming_implementation(4)
