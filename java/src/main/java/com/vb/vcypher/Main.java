package com.vb.vcypher;

public class Main {
  public static void main(String[] args) {
    if (args.length < 1) {
      System.out.println("No input specified!");
    } else {
      System.out.println(Vcypher.encode(args[0]));
    }
  }
}
