from example.fib import fibonacci


def test_fibonacci() -> None:
    assert fibonacci(1) == 1
    assert fibonacci(2) == 1
    assert fibonacci(10) == 55
    assert fibonacci(12) == 144
