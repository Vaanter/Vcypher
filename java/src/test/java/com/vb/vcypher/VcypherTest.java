package com.vb.vcypher;

import static com.google.common.truth.Truth.assertThat;

import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import javax.inject.Inject;
import org.junit.jupiter.api.Test;

@MicronautTest
public class VcypherTest {

  @Inject
  private Vcypher vcypher;

  @Test
  public void testOutput() {
    assertThat(vcypher.encode("BFC")).containsMatch("81156121701");
    assertThat(vcypher.encode("*()")).containsMatch("8123614147318");
  }

  @Test
  public void testEdgeCasesOutput() {
    assertThat(vcypher.encode("")).containsMatch("");
  }

  @Test
  public void testNonEnglishLettersOutputs() {
    assertThat(vcypher.encode("ľ")).containsMatch("623");
    assertThat(vcypher.encode("š")).containsMatch("415");
    assertThat(vcypher.encode("č")).containsMatch("445");
    assertThat(vcypher.encode("ť")).containsMatch("5134");
    assertThat(vcypher.encode("ž")).containsMatch("712");
    assertThat(vcypher.encode("ý")).containsMatch("71");
    assertThat(vcypher.encode("á")).containsMatch("44");
    assertThat(vcypher.encode("í")).containsMatch("612");
    assertThat(vcypher.encode("é")).containsMatch("513");
  }

  @Test
  public void testNonLatinCharacters() {
    assertThat(vcypher.encode("磨")).containsMatch("8347");
  }
}
