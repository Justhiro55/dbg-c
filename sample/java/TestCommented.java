public class TestCommented {
    public static void main(String[] args) {
        // Already commented out debug statements
        // System.out.println("debug: this is commented");
        // System.err.println("DEBUG: error message");

        System.out.println("Active debug message");

        // System.out.println("debug: another commented line");

        int x = 10;
        // System.out.printf("debug: x = %d\n", x);

        // Multiline commented
        // System.out.println(
        //     "debug: multiline"
        // );

        compute();
    }

    static void compute() {
        // System.err.println("debug: computing");
        int result = 42;
        System.out.println("debug: result is " + result);
    }
}
