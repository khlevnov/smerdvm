package ru.khlevnov;

public class Foo extends FooParent {
    static {
        PrintStream.print(3);
    }

    static void fooOuter() {
        Bar.BarInner.bar(42);
    }

    static class FooInner {
        static void foo(int x) {
            PrintStream.print(x);
        }
    }
}
