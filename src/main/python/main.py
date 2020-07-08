import Vcypher
import sys
import base64
from pprint import pprint


if __name__ == '__main__':
    result = Vcypher.vcypher(sys.argv[1])
    pprint(int(result))
    result_bytes = result.encode("utf-8")
    base64_bytes = base64.b64encode(result_bytes)
    base64_result = base64_bytes.decode("utf-8")
    pprint(base64_result)
