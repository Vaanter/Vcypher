package com.vb.vcypher;

import com.google.common.base.Charsets;
import io.micronaut.configuration.picocli.PicocliRunner;
import java.util.Base64;
import javax.inject.Inject;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

@Command(name = "vcypher", description = "CLI App for payload encoding using vcypher algorithm",
    mixinStandardHelpOptions = true)
public class VcypherCommand implements Runnable {

  @Parameters(index = "0", description = "payload to be encoded", arity = "1")
  String payload;

  @Option(names = {"-b", "--base64"}, description = "encodes output in base64")
  boolean base64;

  @Inject
  Vcypher vcypher;

  public static void main(String[] args) {
    PicocliRunner.run(VcypherCommand.class, args);
  }

  @Override
  public void run() {
    String encodedPayload = vcypher.encode(payload);
    if (base64) {
      encodedPayload = Base64
          .getEncoder()
          .encodeToString(encodedPayload.getBytes(Charsets.UTF_8));
    }
    System.out.println(encodedPayload);
  }
}
