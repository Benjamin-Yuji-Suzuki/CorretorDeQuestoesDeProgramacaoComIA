# 👨‍⚖️ Juiz Automático de Programação (Desktop)

O **Juiz Automático** é uma ferramenta desktop desenvolvida para automatizar a correção de exercícios de programação em lote. Ele foi criado especificamente para auxiliar monitores e professores na avaliação de grandes volumes de arquivos `.zip` de forma rápida, justa e eficiente.

Construído com **Tauri** e **Rust**, o aplicativo utiliza inteligência artificial local para garantir a privacidade dos dados e custo zero de processamento.

---

## 🤖 Uso de Inteligência Artificial

Este projeto é um exemplo real de "IA auxiliando na educação":
1. **No Desenvolvimento:** A arquitetura, a lógica em Rust (Tokio) e a interface foram desenvolvidas com o auxílio da IA **Gemini 3 Flash**, garantindo um backend performático e seguro.
2. **Na Operação:** O "cérebro" que avalia os códigos dos alunos utiliza o modelo **Qwen 2.5 Coder (7B)** via Ollama, processando tudo localmente na GPU do usuário (testado em uma RTX 4050).

---

## 🚀 Funcionalidades

- **Correção em Lote:** Selecione uma pasta inteira; o app identifica os ZIPs dos alunos e processa um por um automaticamente.
- **Multilinguagem:** Suporte para análise de arquivos em **C, Java e Python**.
- **Prompt Sanduíche:** Backend otimizado para evitar a "amnésia" da IA em arquivos de código muito longos (700+ linhas).
- **Interface Nativa:** Aplicativo desktop leve projetado para **Pop!_OS / Ubuntu**.

## 🛠️ Tecnologias Utilizadas

- **Frontend:** HTML5, CSS3 (Catppuccin Theme), JavaScript.
- **Backend:** [Rust](https://www.rust-lang.org/) + [Tauri](https://tauri.app/).
- **Runtime:** [Tokio](https://tokio.rs/) (Processamento assíncrono).
- **IA Local:** [Ollama](https://ollama.com/) (Modelo `qwen2.5-coder:7b`).

## 📋 Pré-requisitos

1. **Ollama:** [Instale o Ollama](https://ollama.com/).
2. **Modelo:** Baixe o modelo via terminal:
   ```bash
   ollama pull qwen2.5-coder:7b
   ```
3. **Rust:** Tenha o `cargo` instalado para compilação.

## 🔧 Como Rodar e Compilar

**Desenvolvimento:**
```bash
npm install
npm run tauri dev
```

**Gerar instalador (.deb):**
```bash
cargo tauri build
```
O arquivo final ficará em: `src-tauri/target/release/bundle/deb/`

---
