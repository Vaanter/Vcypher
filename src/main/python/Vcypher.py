import math


def vcypher(inputs: str) -> str:
    binaries = []
    one_count_per_binary = []

    for char in inputs:
        binary = f"{ord(char):08b}"
        binaries.append(binary)
        one_count = binary.count("1")
        one_count_per_binary.append(one_count)

    all_bin = "".join(binaries)
    
    zero_counter = 0
    pre_rot = []
    for pos, char in enumerate(all_bin):
        if char == "0":
            zero_counter += 1
        else:
            if zero_counter != 0:
                if pre_rot:
                    zero_counter += pre_rot[-1]
                pre_rot.append(zero_counter)
                zero_counter = 0
        if (pos == len(all_bin) - 1) and zero_counter != 0:
            if pre_rot:
                zero_counter += pre_rot[-1]
            pre_rot.append(zero_counter)

    fp1 = math.prod(one_count_per_binary)
    
    pre_rot.append(fp1)

    rot_values = []
    for pos, value in enumerate(pre_rot):
        rot_values.append(str(value)[::-1])

    OK = rot_values.pop(-1)

    for i in range(1, len(rot_values) // 2 + 1):
        for _ in range(2):
            check = i % 2 - 1
            OK += rot_values.pop(check)

    return str(int(OK))
