
if __name__ == "__main__":
    length = 7
    dimension = 4
    c = codes.HammingCode(GF(2), 3)
    print("static SYNDROME: [[bool; {dimension}]; {length}] = [".format(dimension=dimension, length=2**length))
    for v in VectorSpace(GF(2), length):
        print(("\t[" + ", ".join(map(lambda x: str(bool(x)), c.decode_to_message(v))) + "],").lower())

    print("];")
