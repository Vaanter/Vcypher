import math


def vcypher(payload: str) -> str:
    binaries = [f"{ord(x):08b}" for x in payload]
    one_count_per_binary = [x.count("1") for x in binaries]

    all_bin = "".join(binaries)
    
    pre_rot = []

    for zeros in [x for x in all_bin.split("1") if x != ""]:
        length = len(zeros)
        if len(pre_rot) > 0:
            length += pre_rot[-1]
        pre_rot.append(length)

    fp1 = math.prod(one_count_per_binary)
    
    pre_rot.append(fp1)

    rot_values = [str(x)[::-1] for x in pre_rot]

    OK = rot_values.pop(-1)

    for i in range(1, len(rot_values) // 2 + 2):
        for _ in range(2):
            if len(rot_values) > 0:
                check = i % 2 - 1
                OK += rot_values.pop(check)

    return str(int(OK))
