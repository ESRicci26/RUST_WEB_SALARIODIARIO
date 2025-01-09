# Calculadora de Salário Diário com Axum

Este é um exemplo de aplicação Web simples construída em Rust utilizando o framework **Axum**.
A aplicação permite calcular o salário diário com base no salário mensal e na quantidade de dias fornecidos.

## Funcionalidades

- Formulário HTML para entrada de dados:
  - Salário Mensal.
  - Quantidade de Dias.
- Cálculo do Salário Diário:
  - Fórmula: `Salário Diário = Salário Mensal / Quantidade de Dias`.
- Exibição dos resultados em uma nova página.
- Link para retornar ao formulário principal.

---

## Como Executar o Projeto

### Pré-requisitos

Certifique-se de ter o Rust instalado em seu ambiente. Caso não tenha, siga as instruções em [rust-lang.org](https://www.rust-lang.org/).

### Configuração do Projeto

1. Certifique-se de que o arquivo `Cargo.toml` está configurado corretamente:
   ```toml
   [package]
   name = "calculadora-salario"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   axum = "0.6"
   tokio = { version = "1.33", features = ["full"] }
   hyper = "0.14"
   tower = "0.4"
   serde = { version = "1.0", features = ["derive"] }
   ```

2. Compile e execute o servidor:
   ```bash
   cargo run
   ```

3. Acesse a aplicação em seu navegador no endereço: [http://localhost:3000](http://localhost:3000).

---

## Estrutura do Projeto

- **Rota `/`**:
  - Exibe o formulário HTML para entrada de dados.
- **Rota `/calculate`**:
  - Recebe os dados enviados pelo formulário e realiza o cálculo do salário diário.
  - Retorna uma página HTML com os resultados.

### Fórmula de Cálculo

O cálculo do salário diário é realizado no backend usando a seguinte fórmula:
```rust
salario_dia = salario_mensal / qtd_dias;
```

---

## Tecnologias Utilizadas

- **[Axum](https://github.com/tokio-rs/axum)**: Framework Web assíncrono e eficiente para Rust.
- **[Tokio](https://tokio.rs/)**: Runtime assíncrono para operações de rede e IO.
- **Serde**: Biblioteca para serialização e deserialização de dados.

---

## Exemplo de Uso

1. Acesse a aplicação em [http://localhost:3000](http://localhost:3000).
2. Preencha o campo **Salário Mensal** com, por exemplo, `3000.00`.
3. Preencha o campo **Quantidade de Dias** com, por exemplo, `30`.
4. Clique no botão **Calcular**.
5. O resultado será exibido:
   - Salário Mensal: `3000.00`
   - Quantidade de Dias: `30`
   - Salário Diário: `100.00`

---

## Estrutura de Arquivos

```plaintext
.
├── Cargo.toml       # Configuração do projeto e dependências
└── src
    └── main.rs      # Código principal do servidor
```

---

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests.

---

## Licença

Este projeto é licenciado sob a [MIT License](LICENSE).

---

## Autor

Desenvolvido por [EDILSON SALVADOR RICCI](https://github.com/ESRicci26/RUST_WEB_SALARIODIARIO).
```
