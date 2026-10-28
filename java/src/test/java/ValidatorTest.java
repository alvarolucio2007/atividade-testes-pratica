import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import static org.junit.jupiter.api.Assertions.*;

public class ValidatorTest{
    @ParameterizedTest
    @CsvSource({"100,2.5,true","5000,0.0,true","99,1.0,false","500,2.6,false"})
    void testLote(int q,double t,boolean exp){
        assertEquals(exp,Validator.validar(q,t));
    }
    @Test void testExceptions(){
        assertThrows(IllegalArgumentException.class,()->Validator.validar(0,1.0));
    }
}
