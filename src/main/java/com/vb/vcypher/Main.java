package com.vb.vcypher;

import java.util.logging.Level;
import lombok.extern.flogger.Flogger;

@Flogger
public class Main {
  public static void main(String[] args) {
    String result = Vcypher.encode("BFC");
    log.at(Level.INFO).log(result);
  }
}
