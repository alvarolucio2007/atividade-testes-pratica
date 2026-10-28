public class Validator{
    public static boolean validar(int q, double t){
        if (q<=0 | t<0.0) {
            throw new IllegalArgumentException();
        }
        return (q>=100 && q<=5000) && (t<=2.5);
    }
}
