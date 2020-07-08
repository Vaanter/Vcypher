import com.vb.vcypher.Vcypher;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class VcypherTest {

  @Test
  public void testOutput() {
    Assertions.assertEquals(Vcypher.encode("BFC"), "81156121701");
    Assertions.assertEquals(Vcypher.encode(""), "");
  }
}
