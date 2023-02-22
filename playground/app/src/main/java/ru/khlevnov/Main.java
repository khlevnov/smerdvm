package ru.khlevnov;

public class Main {
    static {
        PrintStream.print(1);
    }

    public static void main(String[] args) {
        Foo.fooOuter();
        Bar.barOuter();
    }
}
