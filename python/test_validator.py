import pytest
from validator import validar_lote


@pytest.mark.parametrize("q,t,exp",[
    (100,2.5,True),
    (10,2.5,False),
    (5001,2.5,False),
    (1000,2.6,False),
])
def test_lotes(q,t,exp):
    assert validar_lote(q,t)==exp

def test_excecoes():
    with pytest.raises(ValueError):
        validar_lote(0,1.0)
    with pytest.raises(ValueError):
        validar_lote(150,-0.1)
