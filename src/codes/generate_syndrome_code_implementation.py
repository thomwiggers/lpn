#!/usr/bin/env sage
"""Script to generate small, lookup-table hamming code implementations"""
from __future__ import print_function
import itertools
import sys
from jinja2 import Environment
from collections import defaultdict
import sage
from sage.all import *

from sage.features.gap import GapPackage

feature = GapPackage("GUAVA")
print(feature)
if not feature:
    sys.exit(1)

# The databases module can be annoying to load
import sage.coding as coding
from sage.coding import databases
coding.databases.best_linear_code_in_guava


def boollist(lst):
    """Convert (1, 0) into '[true, false]'"""
    return (', '.join(map(lambda x: str(bool(x)), lst))).lower()


def intlist(lst):
    return (', '.join(map(lambda x: str(x), lst)))


ENVIRONMENT = Environment()
ENVIRONMENT.filters['boollist'] = boollist
ENVIRONMENT.filters['intlist'] = intlist


MODULE_TEMPLATE = """
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

"""

def render_module(name, codes):
    with open('{}/mod.rs'.format(name.lower()), 'w') as f:
        f.write(MODULE_TEMPLATE)
        for (n, k) in codes:
            f.write("useit!({namelower}_{n}_{k});\n"
                    .format(namelower=name.lower(), n=n, k=k))

rendered_codes = defaultdict(list)


def vectors_up_to(weight, n):
    assert weight <= n, "Weight should be less than n"

    poses = list(range(n))
    for w in range(weight+1):
        for combination in itertools.combinations(poses, w):
            v = vector(GF(2), n)
            for i in combination:
                v[i] = 1
            yield (v, w)


def bools_to_binvec(bools):
    u64s = list(0 for _ in range(len(bools)//64 + (1 if len(bools) % 64 != 0 else 0)))
    for (i, bit) in enumerate(bools):
        u64s[i//64] |= bool(bit) << (i % 64)
    return u64s

def wagner_to_code(m, k, rows):
    assert k == len(rows), "Doesn't match"
    mat_rows = []
    for row in [list(bin(row)[2:]) for row in rows]:
        row_len = len(row)
        mat_rows.append(
            [GF(2)(i) for i in (['0']*(m-row_len) + row)]
        )   
    P = Matrix(GF(2), mat_rows)
    return codes.LinearCode(matrix.identity(GF(2),k).augment(P))


def generate_code_implementation(name, code, comment=None):
    """Generate a code implementation"""
    k = code.dimension()
    cs, _p = code.standard_form()
    info = {
        'name': name,
        'n': code.length(),
        'k': k,
        'generator': [bools_to_binvec(row) for row in cs.systematic_generator_matrix()],
        'generator_bools': cs.systematic_generator_matrix(),
        'parity_matrix': [bools_to_binvec(row) for row in cs.parity_check_matrix()],
        "comment": comment,
    }

    max_error = code.decoder().maximum_error_weight()

    syndrome_map = {}
    for (he, error) in cs.decoder().syndrome_table().items():
        syndrome_map[ZZ(list(he), base=2)] = bools_to_binvec(error)

    info['syndrome_map'] = syndrome_map
    info['syndrome_map_itemlen'] = len(list(syndrome_map.values())[0])

    assert max(syndrome_map) < 2**64, "sydrome map too big!"
    
    info['info_set'] = cs.information_set()

    testcases = []
    if 'might-error' in cs.decoder().decoder_type():
        max_error -= 3
    for _ in range(20):
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

    rendered_codes[name].append((code.length(), code.dimension()))


if __name__ == "__main__":
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 2))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 3))
    #generate_code_implementation("Hamming", codes.HammingCode(GF(2), 4))
    rendered_codes["Hamming"].append((3, 1))
    rendered_codes["Hamming"].append((7, 4))
    rendered_codes["Hamming"].append((15, 11))
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

    print("MDS codes")
    generate_code_implementation(
        "Mds",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [1, 0, 1],
                    [0, 1, 0],
                ])))
    generate_code_implementation(
        "Mds",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [1, 0, 0, 1],
                    [0, 1, 0, 1],
                    [0, 0, 1, 1],
                ])))
    generate_code_implementation(
        "Mds",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [1, 0, 0, 0, 1],
                    [0, 1, 0, 0, 1],
                    [0, 0, 1, 0, 1],
                    [0, 0, 0, 1, 1],
                ])))
    generate_code_implementation(
        "Custom",
        codes.LinearCode(
            matrix(
                GF(2),
                [
                    [1, 0, 0, 1, 1],
                    [0, 1, 0, 1, 1],
                    [0, 0, 1, 1, 1],
                ])))

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
    
if False:
    print("Wagner codes")
    # https://www.sciencedirect.com/science/article/pii/S0019995866901288
    generate_code_implementation(
        "Wagner",
        wagner_to_code(9, 11, [0o774, 0o763, 0o717, 0o477, 0o377, 0o650, 0o622, 0o605, 0o330, 0o243, 0o631]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(9, 13, [0o760, 0o714, 0o525, 0o256, 0o702, 0o650, 0o621, 0o511, 0o243, 0o445, 0o126, 0o572, 0o675]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(9, 14, [0o760, 0o714, 0o525, 0o256, 0o702, 0o650, 0o621, 0o511, 0o243, 0o445, 0o126, 0o572, 0o675, 0o337]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 15, [0o1216, 0o73, 0o664, 0o1550, 0o1777, 0o1321, 0o507, 0o1643, 0o1435, 0o1166, 0o1524, 0o1422, 0o612, 0o530, 0o467]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 16, [0o1216, 0o73, 0o664, 0o1550, 0o1777, 0o1321, 0o507, 0o1643, 0o1435, 0o1166, 0o1524, 0o1422, 0o612, 0o530, 0o262, 0o123]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 17, [0o525, 0o1252, 0o377, 0o1477, 0o1717, 0o1763, 0o1774, 0o1640, 0o1510, 0o1422, 0o1405, 0o1304, 0o1203, 0o720, 0o611, 0o226, 0o164]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 18, [0o525, 0o1252, 0o377, 0o1477, 0o1717, 0o1763, 0o1774, 0o1640, 0o1510, 0o1422, 0o1405, 0o1304, 0o1203, 0o720, 0o611, 0o226, 0o164, 0o47]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 19, [0o525, 0o1252, 0o377, 0o1477, 0o1717, 0o1763, 0o1774, 0o1640, 0o1510, 0o1422, 0o1405, 0o1304, 0o1203, 0o720, 0o611, 0o216, 0o66, 0o1171, 0o1547]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(10, 20, [0o525, 0o1252, 0o377, 0o1477, 0o1717, 0o1763, 0o1774, 0o1640, 0o1510, 0o1422, 0o1405, 0o1304, 0o1203, 0o720, 0o231, 0o164, 0o1166, 0o1066, 0o436, 0o1171]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )
    generate_code_implementation(
        "Wagner",
        wagner_to_code(11, 21, [0o3550, 0o3321, 0o2643, 0o1507, 0o3216, 0o2435, 0o1664, 0o732, 0o355, 0o2166, 0o1073, 0o3044, 0o3022, 0o2700, 0o2775, 0o1767, 0o3717, 0o3477, 0o227, 0o526, 0o2322]),
        comment="https://www.sciencedirect.com/science/article/pii/S0019995866901288",
    )

if True:
    guava_version = gap("guava_version();")
    for n in range(25):
        for k in range(10, n):
            code = coding.databases.best_linear_code_in_guava(n, k, GF(2))
            if code:
                print("Rendering GUAVA-{}-found [{}, {}] code".format(guava_version, n, k))
                generate_code_implementation(
                    "Guava",
                    code,
                    comment="Best code found from the GUAVA database version {}".format(guava_version)
                )
            else:
                print("Guava doesn't have [{}, {}]".format(n, k))

for (name, codes) in rendered_codes.items():
    render_module(name, codes)
