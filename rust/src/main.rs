fn main() {
    println!("Hello, world!");
}
fn validar_lote(quantidade: i32, taxa: f32) -> Result<bool, String> {
    if quantidade <= 0 {
        Err(String::from("Erro: Quantidade invalida!"))
    } else if taxa < 0.0 {
        Err(String::from("Erro: Taxa invalida!"))
    } else {
        let result = 100 <= quantidade && quantidade <= 5000 && taxa <= 2.5;
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deve_retornar_erro_quando_quantidade_for_zero_ou_negativa() {
        let resultado = validar_lote(0, 1.5);
        assert_eq!(resultado, Err(String::from("Erro: Quantidade invalida!")));

        let resultado_negativo = validar_lote(-5, 1.5);
        assert!(resultado_negativo.is_err()); // Outra forma de testar se deu erro
    }

    #[test]
    fn deve_retornar_erro_quando_taxa_for_negativa() {
        let resultado = validar_lote(500, -0.5);
        assert_eq!(resultado, Err(String::from("Erro: Taxa invalida!")));
    }

    #[test]
    fn deve_retornar_ok_true_quando_lote_estiver_dentro_das_regras() {
        // Quantidade entre 100 e 5000, taxa <= 2.5
        let resultado = validar_lote(1000, 2.0);
        assert_eq!(resultado, Ok(true));
    }

    #[test]
    fn deve_retornar_ok_false_quando_lote_violar_regras_de_negocio() {
        // Quantidade fora do intervalo (ex: 50) ou taxa alta (ex: 3.0)
        let resultado_qtd = validar_lote(50, 1.5);
        assert_eq!(resultado_qtd, Ok(false));

        let resultado_taxa = validar_lote(1000, 3.0);
        assert_eq!(resultado_taxa, Ok(false));
    }
}
