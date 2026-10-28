def validar_lote(quantidade_pecas:int, taxa_defeitos_percentual:float)->bool:
    if quantidade_pecas<=0 or taxa_defeitos_percentual<0.0:
        raise ValueError("Parametros invalidos!")
    return 100<=quantidade_pecas<=5000 and taxa_defeitos_percentual<=2.5
