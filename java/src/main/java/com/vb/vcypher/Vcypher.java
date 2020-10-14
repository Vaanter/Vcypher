package com.vb.vcypher;

import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.base.Strings;
import com.google.common.base.CharMatcher;
import java.math.BigInteger;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class Vcypher {

  public static final String encode(String payload) {
    ArrayList<String> binaries = new ArrayList<>();
    ArrayList<Integer> oneCounts = new ArrayList<>();

    // Get binary values
    for (char b : payload.toCharArray()) {
      String binaryValue = Strings.padStart(Integer.toBinaryString(b), 8, '0');
      int oneCount = CharMatcher.is('1').countIn(binaryValue);
      binaries.add(binaryValue);
      oneCounts.add(oneCount);
    }

    // concatenate all binary values
    String allBin = Joiner.on("").join(binaries);

    List<String> zeroSplit = Splitter.on("1").omitEmptyStrings().splitToList(allBin);

    // count zeros
    ArrayList<Integer> zeroCurve = new ArrayList<>();
    for (String s : zeroSplit) {
      int zeroCount = s.length();
      if (!zeroCurve.isEmpty()) {
        zeroCount += zeroCurve.get(zeroCurve.size() - 1);
      }
      zeroCurve.add(zeroCount);
    }

    ArrayList<String> preRot = zeroCurve.stream().map(String::valueOf).collect(Collectors.toCollection(ArrayList::new));

    if (oneCounts.size() > 0) {
      BigInteger x = BigInteger.valueOf(1);
      for (Integer anOneCount : oneCounts) {
        x = x.multiply(BigInteger.valueOf(anOneCount));
      }
      preRot.add(x.toString());
    }

    // reverse values
    ArrayDeque<String> rotValues = preRot.stream().map(x -> new StringBuilder(x).reverse().toString())
        .collect(Collectors.toCollection(ArrayDeque::new));

    ArrayList<String> cipher = new ArrayList<>();
    cipher.add(rotValues.pollLast());
    for (int i = 0, l = rotValues.size() / 2 + 1; i < l; i++) {
      for (int j = 0; j < 2; j++) {
        if (i % 2 != 0) {
          cipher.add(rotValues.pollLast());
        } else {
          cipher.add(rotValues.poll());
        }
      }
    }

    // concatenate cipher
    String ok = Joiner.on("").skipNulls().join(cipher);

    // return the cipher
    return ok.equals("") ? "" : new BigInteger(ok).toString();
  }
}
