import 'dart:collection';

void main(List<String> args) {
  if (args.isEmpty) {
    print("No input specified!");
    return;
  }
  vcypherEncode(args.first);
}

void vcypherEncode(final String payload) {
  var binaries = List<String>();
  var oneCounts = List<int>();

  for (var char in payload.codeUnits) {
    var binaryValue = char.toRadixString(2).padLeft(8, "0");
    var oneCount = binaryValue.replaceAll("0", "").length;
    binaries.add(binaryValue);
    oneCounts.add(oneCount);
  }

  var zeroSplit = binaries
      .join()
      .split("1")
      .map((e) => e.length)
      .where((element) => element > 0)
      .toList();

  var zeroCurve = List<int>();
  for (var zeros in zeroSplit) {
    var length = zeros;
    if (zeroCurve.length > 0) {
      length += zeroCurve.elementAt(zeroCurve.length - 1);
    }
    zeroCurve.add(length);
  }

  var preRot = Queue.from(zeroCurve
      .map((e) => String.fromCharCodes(e.toString().runes.toList().reversed))
      .toList());

  if (oneCounts.isNotEmpty) {
    var p1 = BigInt.one;
    for (var number in oneCounts) {
      p1 *= BigInt.from(number);
    }
    preRot.add(String.fromCharCodes(p1.toString().runes.toList().reversed));
  }

  var cipher = List<String>()..add(preRot.removeLast());
  var i = 1;
  while (preRot.isNotEmpty) {
    for (var j = 0; j < 2; j++) {
      if (preRot.isEmpty) {
        break;
      }
      if (i % 2 == 0) {
        cipher.add(preRot.removeLast());
      } else {
        cipher.add(preRot.removeFirst());
      }
    }
    i = i % 2 - 1;
  }

  print(BigInt.parse(cipher.join("")));
}
