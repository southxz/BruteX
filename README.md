# ForceVortex -- Log Scanner & Credential Checker

Um utilitário avançado para **processamento massivo de logs**, extração
de credenciais e testes automáticos de login para múltiplos serviços.

## 🚀 Recursos Principais

-   Leitura de arquivos de log em blocos (alto desempenho)
-   Extração automática de credenciais encontradas
-   Testes de autenticação:
    -   FTP
    -   MySQL
    -   SSH
    -   WordPress
    -   cPanel / Webmail
    -   phpMyAdmin
-   Multi-thread com **Rayon**
-   Relatórios automáticos salvos em arquivos `.txt`

⚠️ **Aviso Legal**\
Este software deve ser utilizado **somente** em sistemas onde você tem
permissão explícita. Uso indevido pode ser ilegal.

------------------------------------------------------------------------

# 📂 Estrutura do Projeto

    .
    ├── config.json
    ├── main.rs
    ├── ForceVortex/
    ├── mysql_logade_sucess.txt
    ├── ftp_logade_sucess.txt
    ├── wp_sucess.txt
    ├── phpmyadmin_sucess.txt
    ├── cpanel_webmail.txt
    ├── sshlogade.txt

------------------------------------------------------------------------

# ⚙️ Configuração (`config.json`)

``` json
{
    "default_threads": 10,
    "openfile_chuks_mb": 10,
    "keys_search_logs": ["login", "user", "pass"],
    "default_login": true
}
```

### Campos

  Campo                 Função
  --------------------- ----------------------------------------------
  `default_threads`     Número de threads usadas pelo Rayon
  `openfile_chuks_mb`   Tamanho do bloco de leitura do arquivo (MB)
  `keys_search_logs`    Palavras-chave para busca dentro dos logs
  `default_login`       Se deve testar logins padrão automaticamente

------------------------------------------------------------------------

# ▶️ Como Usar

## **1. Execute o programa**

O programa abrirá uma janela pedindo um arquivo `.txt` com logs:

    Select File Logs

## **2. O sistema irá:**

-   Ler o arquivo em pedaços
-   Encontrar credenciais automaticamente
-   Testar em paralelo nos serviços suportados
-   Salvar qualquer sucesso nas saídas:

```{=html}
<!-- -->
```
    ftp_logade_sucess.txt
    mysql_logade_sucess.txt
    sshlogade.txt
    wp_sucess.txt
    cpanel_webmail.txt
    phpmyadmin_sucess.txt

------------------------------------------------------------------------

# 🧠 Funcionamento Interno

-   **`openfile_return_buffer()`**\
    Lê o arquivo em blocos e coleta credenciais únicas em
    `HashSet<Login>`.

-   **Tentativas Automáticas**\
    Cada credencial roda em threads paralelas:

```{=html}
<!-- -->
```
    mysql_login_
    ftp_login_
    sshLogin
    cp_panel_and_webmail
    wp_login
    phpmyadmin

-   **Filtragem de IPs internos**\
    Linhas contendo:
    -   192.168.x.x\
    -   127.0.x.x\
    -   localhost\
    -   10.x.x.x\
        são automaticamente ignoradas.

------------------------------------------------------------------------

# 📥 Download

O arquivo README.md foi gerado e salvo como:

**`/mnt/data/README.md`**

------------------------------------------------------------------------

# ðŸ“‚ Estrutura do Projeto

    .
    â”œâ”€â”€ config.json
    â”œâ”€â”€ main.rs
    â”œâ”€â”€ ForceVortex/
    â”œâ”€â”€ mysql_logade_sucess.txt
    â”œâ”€â”€ ftp_logade_sucess.txt
    â”œâ”€â”€ wp_sucess.txt
    â”œâ”€â”€ phpmyadmin_sucess.txt
    â”œâ”€â”€ cpanel_webmail.txt
    â”œâ”€â”€ sshlogade.txt

------------------------------------------------------------------------

# âš™ï¸ ConfiguraÃ§Ã£o (`config.json`)

``` json
{
    "default_threads": 10,
    "openfile_chuks_mb": 10,
    "keys_search_logs": ["login", "user", "pass"],
    "default_login": true
}
```

### Campos

  Campo                 FunÃ§Ã£o
  --------------------- ----------------------------------------------
  `default_threads`     NÃºmero de threads usadas pelo Rayon
  `openfile_chuks_mb`   Tamanho do bloco de leitura do arquivo (MB)
  `keys_search_logs`    Palavras-chave para busca dentro dos logs
  `default_login`       Se deve testar logins padrÃ£o automaticamente

------------------------------------------------------------------------

# â–¶ï¸ Como Usar

## **1. Execute o programa**

O programa abrirÃ¡ uma janela pedindo um arquivo `.txt` com logs:

    Select File Logs

## **2. O sistema irÃ¡:**

-   Ler o arquivo em pedaÃ§os
-   Encontrar credenciais automaticamente
-   Testar em paralelo nos serviÃ§os suportados
-   Salvar qualquer sucesso nas saÃ­das:

```{=html}
<!-- -->
```
    ftp_logade_sucess.txt
    mysql_logade_sucess.txt
    sshlogade.txt
    wp_sucess.txt
    cpanel_webmail.txt
    phpmyadmin_sucess.txt

------------------------------------------------------------------------

# ðŸ§  Funcionamento Interno

-   **`openfile_return_buffer()`**\
    LÃª o arquivo em blocos e coleta credenciais Ãºnicas em
    `HashSet<Login>`.

-   **Tentativas AutomÃ¡ticas**\
    Cada credencial roda em threads paralelas:

```{=html}
<!-- -->
```
    mysql_login_
    ftp_login_
    sshLogin
    cp_panel_and_webmail
    wp_login
    phpmyadmin

-   **Filtragem de IPs internos**\
    Linhas contendo:
    -   192.168.x.x\
    -   127.0.x.x\
    -   localhost\
    -   10.x.x.x\
        sÃ£o automaticamente ignoradas.

------------------------------------------------------------------------

# ðŸ“¥ Download

O arquivo README.md foi gerado e salvo como:

**`/mnt/data/README.md`**
