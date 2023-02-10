#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait DatabaseHandler {
    fn ejecutar_query(&self, query: String);
}

#[cfg_attr(test, automock)]
pub trait ApiHandler {
    // El endpoint "retornara" la suma de ambos valores
    fn resultado_endpoint_calculos(&self, a: i32, b: i32) -> i32;

    // El endpoint "retornara" el "resultado" de una validacion
    fn resultado_endpoint_validacion(&self, valor: i32) -> bool;
}

pub fn obtener_usuario_de_base_de_datos(db: Box<dyn DatabaseHandler>, id: i32) {
    let query = format!("SELECT * FROM usuarios WHERE id={}", id);
    db.ejecutar_query(query);
}

pub fn llamar_endpoint_calculos(http: Box<dyn ApiHandler>, a: i32, b:i32) -> i32 {
    // En la practica, probablemente este call sea asincrono (async/await)
    http.resultado_endpoint_calculos(a, b)
}

pub fn llamar_endpoint_validacion(http: Box<dyn ApiHandler>, valor: i32) -> bool {
    // En la practica, probablemente este call sea asincrono (async/await)
    http.resultado_endpoint_validacion(valor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probar_que_el_query_se_ejecuta_unicamente_una_vez() {
        let mut mock_db = Box::new(MockDatabaseHandler::new());

        mock_db.expect_ejecutar_query()
               .with(
                   eq("SELECT * FROM usuarios WHERE id=22".to_owned())
               )
               .once()
               .returning(|_query| ());

        obtener_usuario_de_base_de_datos(mock_db, 22);
    }

    #[test]
    fn probar_que_obtenga_el_calculo_correcto_del_api_handler() {
        let mut mock_api_handler = Box::new(MockApiHandler::new());

        let a = 1;
        let b = 1;

        mock_api_handler.expect_resultado_endpoint_calculos()
                        .once()
                        .returning(|a, b| a + b);

        let observado = llamar_endpoint_calculos(mock_api_handler, a, b);
        let esperado = 2;
        assert_eq!(observado, esperado);
    }

    #[test]
    fn probar_que_el_endpoint_de_validacion_devuelve_un_true_si_se_envia_un_2() {
        let mut mock_api_handler = Box::new(MockApiHandler::new());

        let valor = 2;

        mock_api_handler.expect_resultado_endpoint_validacion()
                        .with(eq(2))
                        .return_once(|_valor| true);

        let observado = llamar_endpoint_validacion(mock_api_handler, valor);
        assert!(observado);
    }

    #[test]
    fn probar_que_el_endpoint_de_validacion_devuelve_un_true_si_se_envia_un_4() {
        let mut mock_api_handler = Box::new(MockApiHandler::new());

        let valor = 4;

        mock_api_handler.expect_resultado_endpoint_validacion()
                        .with(eq(4))
                        .return_once(|_valor| false);

        let observado = llamar_endpoint_validacion(mock_api_handler, valor);
        assert!(!observado);
    }
}

