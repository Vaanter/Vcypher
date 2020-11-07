package com.vb.vcypher;

import com.google.common.base.CharMatcher;
import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.base.Strings;
import java.math.BigInteger;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public final class Vcypher {

  /**
   * Encodes input with vcypher algorithm.
   * @param payload String to be encoded.
   * @return Encoded payload.
   */
  public static String encode(String payload) {
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

    ArrayDeque<String> preRot = zeroCurve
        .stream()
        .map(String::valueOf)
        .collect(Collectors.toCollection(ArrayDeque::new));

    if (oneCounts.size() > 0) {
      BigInteger x = BigInteger.valueOf(1);
      for (Integer anOneCount : oneCounts) {
        x = x.multiply(BigInteger.valueOf(anOneCount));
      }
      preRot.add(new StringBuilder(x.toString()).reverse().toString());
    }

    ArrayList<String> cipher = new ArrayList<>();
    cipher.add(preRot.pollLast());
    for (int i = 0, l = preRot.size() / 2 + 1; i < l; i++) {
      for (int j = 0; j < 2; j++) {
        String value;
        if (i % 2 != 0) {
          value = preRot.pollLast();
        } else {
          value = preRot.poll();
        }
        if (value != null) {
          cipher.add(new StringBuilder(value).reverse().toString());
        }
      }
    }

    // concatenate cipher
    String ok = Joiner.on("").skipNulls().join(cipher);

    // return the cipher
    return ok.equals("") ? "" : new BigInteger(ok).toString();
  }
}
