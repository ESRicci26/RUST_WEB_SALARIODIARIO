use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define as rotas
    let app = Router::new()
        .route("/", get(render_form)) // Renderiza o formulário HTML
        .route("/calculate", post(calculate_salary)); // Processa os cálculos

    // Configura o endereço e porta do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Inicializa o servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Renderiza o formulário HTML
async fn render_form() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Calculadora de Salário</title>
        </head>
        <body>
            <h1>Calculadora de Salário Diário</h1>
            <form action="/calculate" method="post">
                <label for="salario_mensal">Salário Mensal:</label>
                <input type="number" id="salario_mensal" name="salario_mensal" step="0.01" required>
                <br><br>
                <label for="qtd_dias">Quantidade de Dias:</label>
                <input type="number" id="qtd_dias" name="qtd_dias" required>
                <br><br>
                <button type="submit">Calcular</button>
            </form>
        </body>
        </html>
        "#,
    )
}

// Estrutura para capturar os dados do formulário
#[derive(Deserialize)]
struct SalaryForm {
    salario_mensal: f64,
    qtd_dias: u32,
}

// Processa os cálculos e retorna o HTML com o resultado
async fn calculate_salary(Form(data): Form<SalaryForm>) -> Html<String> {
    let salario_dia = if data.qtd_dias > 0 {
        data.salario_mensal / data.qtd_dias as f64
    } else {
        0.0
    };

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Resultado</title>
        </head>
        <body>
            <h1>Resultado do Cálculo</h1>
            <p><strong>Salário Mensal:</strong> {:.2}</p>
            <p><strong>Quantidade de Dias:</strong> {}</p>
            <p><strong>Salário Diário:</strong> {:.2}</p>
            <a href="/">Voltar</a>
        </body>
        </html>
        "#,
        data.salario_mensal, data.qtd_dias, salario_dia
    ))
}

//http://localhost:3000