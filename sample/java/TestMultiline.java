public class TestMultiline {
    public static void main(String[] args) {
        // Single line
        System.out.println("debug: single line");

        // Multiline statement
        System.out.printf(
            "debug: multiline message with value: %d\n",
            computeValue()
        );

        // Complex expression
        System.out.println(
            "Results: x=" + 10 + ", y=" + 20 + ", sum=" + (10 + 20)
        );

        // Nested function calls
        System.err.printf("debug: nested call result: %d\n", calculate(5, multiply(2, 3)));

        // Very long multiline
        System.out.println(
            "DEBUG: This is a very long debug message " +
            "that spans multiple lines and contains " +
            "important information: " + formatData()
        );
    }

    static int computeValue() {
        return 42;
    }

    static int calculate(int a, int b) {
        return a + b;
    }

    static int multiply(int a, int b) {
        return a * b;
    }

    static String formatData() {
        return "formatted";
    }
}
