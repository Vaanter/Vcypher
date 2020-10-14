def vcypher_new(inputs: str) -> str:
    separated_inputs = list(inputs)
    binaries = []
    one_count_per_binary = []
    primary_buffer = ""
    secondary_buffer = ""
    buffer_counter = 0

    for i in range(len(separated_inputs)):
        binary = f"{ord(separated_inputs[i]):08b}"
        primary_buffer += binary
        buffer_counter += 1
        if buffer_counter == 5:
            secondary_buffer += primary_buffer
            primary_buffer = ""
        if buffer_counter == 10 or i == len(separated_inputs) - 1:
            if secondary_buffer == "":
                secondary_buffer = "0" * len(primary_buffer)
            xor = f"{(int(secondary_buffer, 2) ^ int(primary_buffer, 2)):040b}"
            binaries.append(xor)
            buffer_counter = 0

    print(binaries)
    return ""


if __name__ == "__main__":
    vcypher_new("test")
