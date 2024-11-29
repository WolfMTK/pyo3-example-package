from example.numeric import Number

def test_repr() -> None:
    num = Number(1)
    assert repr(num) == "Number(1)"
    assert str(num) == "1"
    assert bool(num) is True
    num = Number(0)
    assert bool(num) is False
