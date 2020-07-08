package com.vb.vcypher;

import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.base.Strings;
import com.google.common.base.CharMatcher;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import javax.annotation.Nonnull;

public class Vcypher {

  public static String encode(String input) {
    ArrayList<String> binaries = new ArrayList<>();
    ArrayList<Integer> oneCounts = new ArrayList<>();

    //Get binary values
    for (char b : input.toCharArray()) {
      String binaryValue = Strings.padStart(Integer.toBinaryString(b), 8, '0');
      int oneCount = CharMatcher.is('1').countIn(binaryValue);
      binaries.add(binaryValue);
      oneCounts.add(oneCount);
    }

    //concatenate all binary values
    String allBytes = Joiner.on("").join(binaries);

    List<String> zeroSplit = Splitter.on("1")
        .omitEmptyStrings()
        .splitToList(allBytes);

    //count zeros
    ArrayList<Integer> zeroCurve = new ArrayList<>();
    for (String s : zeroSplit) {
      int zeroCount = s.length();
      if (!zeroCurve.isEmpty()) {
        zeroCount += zeroCurve.get(zeroCurve.size() - 1);
      }
      zeroCurve.add(zeroCount);
    }
    ArrayList<String> preRot = zeroCurve
        .stream()
        .map(String::valueOf)
        .collect(Collectors.toCollection(ArrayList::new));

    BigInteger x = BigInteger.valueOf(1);
    for (Integer anOneCount : oneCounts) {
      x = x.multiply(BigInteger.valueOf(anOneCount));
    }
    preRot.add(x.toString());

    //rotate values x >= 10
    ArrayList<String> rotValues = new ArrayList<>();
    for (String value : preRot) {
      if (value.length() >= 2) {
        rotValues.add(new StringBuilder(value).reverse().toString());
      } else {
        rotValues.add(value);
      }
    }

    ArrayList<String> cipher = new ArrayList<>();
    for (int i = 0, l = rotValues.size() / 2 + 1; i < l; i++) {
      if (i == 0) {
        cipher.add(rotValues.remove(rotValues.size() - 1));
      } else {
        if (rotValues.size() >= 2) {
          for (int j = 0; j < 2; j++) {
            if (i % 2 == 0) {
              cipher.add(rotValues.remove(rotValues.size() - 1));
            } else {
              cipher.add(rotValues.remove(0));
            }
          }
        } else {
          cipher.add(rotValues.remove(0));
        }
      }
    }

    //concatenate cipher
    String ok = Joiner.on("").join(cipher);

    //Remove zeros at the beginning
    for (int i = 0; i < ok.length(); i++) {
      if (ok.charAt(0) == "0".charAt(0)) {
        ok = ok.substring(1);
      } else {
        break;
      }
    }

    //return the cipher
    return ok;
  }
}
