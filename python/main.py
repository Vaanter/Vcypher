import Vcypher
import sys
import base64
from pprint import pprint


def main():
    payload = ""
    if len(sys.argv) >= 2:
        payload = sys.argv[1]
    result = Vcypher.vcypher(payload)
    encode_base64 = False
    if len(sys.argv) == 3:
        if sys.argv[2] != "--base64":
            pprint("Invalid second parameter!")
            return
        else:
            encode_base64 = True
    if encode_base64:
        base64_bytes = base64.b64encode(result.encode("utf-8"))
        base64_result = base64_bytes.decode("utf-8")
        pprint(base64_result)
    else:
        pprint(result)


if __name__ == '__main__':
    main()
