package ru.khlevnov;

public class Bar extends BarParent {
    static {
        PrintStream.print(4);
    }

    static void barOuter() {
        Foo.FooInner.foo(43);
    }

    static class BarInner {
        static void bar(int x) {
            PrintStream.print(x);
        }
    }
}
