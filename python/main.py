import base64

import Vcypher
import click


@click.command()
@click.argument("payload")
@click.option("--base64/--no-base64", "base64_", default=False)
def main(payload: str, base64_: bool):
    result = Vcypher.vcypher(payload)
    if base64_:
        base64_bytes = base64.b64encode(result.encode("utf-8"))
        result = base64_bytes.decode("utf-8")
    print(result)


if __name__ == '__main__':
    main()
