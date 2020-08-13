import Vcypher
import sys
import base64
from pprint import pprint


if __name__ == '__main__':
    payload = sys.argv[1]
    result = Vcypher.vcypher(payload)
    pprint(int(result))
    base64_bytes = base64.b64encode(result.encode("utf-8"))
    base64_result = base64_bytes.decode("utf-8")
    # pprint(base64_result)
