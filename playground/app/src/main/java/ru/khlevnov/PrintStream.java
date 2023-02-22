package ru.khlevnov;

public class PrintStream {
    static {
        print(5);
    }

    static native void print(int x);
}
