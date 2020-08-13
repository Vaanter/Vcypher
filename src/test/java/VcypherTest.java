import com.vb.vcypher.Vcypher;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class VcypherTest {

  @Test
  public void testOutput() {
    Assertions.assertEquals(Vcypher.encode("BFC"), "81156121701");
    Assertions.assertEquals(Vcypher.encode("*()"), "8123614147318");
  }

  @Test
  public void testEdgeCasesOutput() {
    Assertions.assertEquals(Vcypher.encode(""), "");
  }

  @Test
  public void testNonEnglishLettersOutputs() {
    Assertions.assertEquals(Vcypher.encode("ľ"), "623");
    Assertions.assertEquals(Vcypher.encode("š"), "415");
    Assertions.assertEquals(Vcypher.encode("č"), "445");
    Assertions.assertEquals(Vcypher.encode("ť"), "5134");
    Assertions.assertEquals(Vcypher.encode("ž"), "712");
    Assertions.assertEquals(Vcypher.encode("ý"), "71");
    Assertions.assertEquals(Vcypher.encode("á"), "44");
    Assertions.assertEquals(Vcypher.encode("í"), "612");
    Assertions.assertEquals(Vcypher.encode("é"), "513");
  }
}
