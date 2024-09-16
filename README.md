Software Regressiva é um software livre.

## Objetivo

Um software simples para realizar a regressiva de um debate da eleições.

## Requisitos

- Rust
- Biblioteca sdl2 
- Biblioteca sdl2_ttf 
- arquivo de font no formato ttf, no diretorio 'fonts' e o nome do arquivo deve ser especificado no arquivo Config.toml

## Como Usar

1. **Clone o repositório**
   ```powershell
   git clone "https://github.com/seu-usuario/software-regressiva.git"
   ```
2. **Navegue até o diretório do projeto**
   ```powershell
   cd software-regressiva
   ```
3. **Instale as dependências necessárias**
   - **Rust (edição 2021)**
     - [Instalação do Rust](https://www.rust-lang.org/tools/install)
   - **Bibliotecas SDL2 e SDL2_ttf** 
        - [SDL2](https://www.libsdl.org/)
        - [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/)
4. **Configure o arquivo de fontes**
   - Coloque o arquivo de fonte no formato `.ttf` dentro da pasta `fonts`.
   - Especifique o nome do arquivo de fonte no arquivo `Config.toml`:
     ```toml
     font = "fonts/nome_da_fonte.ttf"
     ```
5. **Teste o software**
   ```powershell
   cargo run
   ```
6. **Compile o software em modo release para produção**
   ```powershell
   cargo build --release
   ```
7. **Preparação para Produção**
   - **Diretório de Fontes**
   - **Arquivo de Configuração**
     - Inclua o arquivo de configuração `Config.toml` no diretório do executável.
     - Assegure-se de que a pasta `fonts` esteja no mesmo diretório do executável.
   - **DLLs Necessárias**
     - Inclua as DLLs das bibliotecas `sdl2` e `sdl2_ttf` no diretório do executável.
     - Você pode copiar estas DLLs manualmente ou configurar um script de build para automatizar este processo.
   - **Configuração Final**
     - Verifique se todas as configurações no `Config.toml` estão corretas e apontando para os recursos necessários.


## Licença

GPLv3

## Contribuição

Contribuições são bem-vindas!

## Créditos

- [Rust](https://www.rust-lang.org/)
- [SDL2](https://www.libsdl.org/)
- [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/)