public class TestBasic {
    public static void main(String[] args) {
        System.out.println("Starting application");
        System.out.println("debug: initialization started");

        int x = 42;
        System.out.printf("Value: %d\n", x);

        System.err.println("error: something went wrong");
        System.err.println("DEBUG: detailed error info");

        System.out.print("inline message");

        int result = calculate(10, 20);
        System.out.println("Result: " + result);
    }

    static int calculate(int a, int b) {
        System.out.printf("debug: calculating %d + %d\n", a, b);
        return a + b;
    }
}
