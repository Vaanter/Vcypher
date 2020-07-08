import functools
import operator


def vcypher(inputs: str) -> str:
    separated_inputs = list(inputs)
    binaries = []
    one_count_per_binary = []

    for i in range(len(separated_inputs)):
        binary = f"{ord(separated_inputs[i]):08b}"
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
                zero_counter += pre_rot[-1] if pre_rot else zero_counter
                pre_rot.append(zero_counter)
                zero_counter = 0
        if (pos == len(all_bin) - 1) and zero_counter != 0:
            if pre_rot:
                zero_counter += pre_rot[-1]
            pre_rot.append(zero_counter)

    fp1 = functools.reduce(operator.mul, one_count_per_binary, 1)
    pre_rot.append(fp1)

    rot_values = []
    for pos, pos_int in enumerate(pre_rot):
        char = str(pos_int)
        if len(char) >= 2:
            char = str(pos_int)[::-1]
        rot_values.append(char)

    OK = ""

    for i in range(len(rot_values) // 2 + 1):
        if i == 0:
            OK += rot_values.pop(-1)
        else:
            if len(rot_values) >= 2:
                for _ in range(2):
                    check: int = i % 2 - 1
                    OK += rot_values.pop(check)
            else:
                OK += rot_values.pop(0)
    for i in OK:
        if i == "0":
            OK = OK[1:]
            continue
        break
    return OK


def bit_not(n, numbits=8):
    return (1 << numbits) - 1 - n
