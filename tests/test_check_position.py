import pytest

from example import check_position


def test_check_position() -> None:
    assert check_position(1) is None
    with pytest.raises(ValueError) as message:
        check_position(-1)
    assert str(message.value) == "x is negative"
