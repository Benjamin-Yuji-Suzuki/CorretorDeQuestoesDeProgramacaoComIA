// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::io::Read;
use zip::ZipArchive;
use reqwest::Client;
use serde_json::json;

#[tauri::command]
async fn avaliar_turma() -> Result<String, String> {
    // Agora o seletor busca uma PASTA (Diretório) e não um arquivo solto
    let pasta = rfd::FileDialog::new().pick_folder();

    match pasta {
        Some(caminho_pasta) => {
            let mut relatorio_final = String::new();
            let client = Client::new();
            let url = "http://localhost:11434/api/chat";

            let regra_sistema = "Você é um rigoroso juiz automático de programação. \
                VOCÊ ESTÁ PROIBIDO DE REESCREVER O CÓDIGO. \
                VOCÊ DEVE RESPONDER ESTRITAMENTE EM PORTUGUÊS. \
                Responda EXATAMENTE neste formato:\n\
                **RESUMO DA LÓGICA:** [1 parágrafo curto]\n\
                **BUGS LÓGICOS:** [Lista de erros ou 'Nenhum erro grave']\n\
                **NOTA DE FUNCIONALIDADE:** [0 a 10]";

            // Lê todo o conteúdo da pasta selecionada
            let entradas = fs::read_dir(&caminho_pasta).map_err(|e| format!("Erro ao ler pasta: {}", e))?;

            for entrada in entradas {
                let entrada = entrada.map_err(|e| format!("Erro no arquivo: {}", e))?;
                let path = entrada.path();

                // Se o arquivo for um .zip (um aluno)
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("zip") {
                    let nome_aluno = path.file_name().unwrap().to_string_lossy().into_owned();
                    
                    // Cria um cabeçalho bonitão para o aluno no HTML
                    relatorio_final.push_str(&format!(
                        "<div style='background-color: #313244; padding: 10px; border-radius: 8px; margin-top: 20px;'>\n\
                        <h2 style='color: #89b4fa; margin: 0;'>🧑‍🎓 Aluno/Equipe: {}</h2>\n\
                        </div>\n", 
                        nome_aluno
                    ));

                    let file = File::open(&path).map_err(|e| format!("Erro ao abrir o ZIP: {}", e))?;
                    let mut archive = ZipArchive::new(file).map_err(|e| format!("Erro ao ler o ZIP: {}", e))?;
                    
                    let mut codigos_para_avaliar = Vec::new();

                    for i in 0..archive.len() {
                        let mut arquivo_interno = archive.by_index(i).unwrap();
                        let outpath = match arquivo_interno.enclosed_name() {
                            Some(p) => p.to_owned(),
                            None => continue,
                        };

                        let ext = outpath.extension().and_then(|s| s.to_str()).unwrap_or("");
                        
                        // O SEGREDO MULTI-LINGUAGEM: Aceita C, Java e Python
                        if arquivo_interno.is_file() && (ext == "c" || ext == "java" || ext == "py") {
                            let mut conteudo = String::new();
                            arquivo_interno.read_to_string(&mut conteudo).unwrap_or_default();
                            let nome_arquivo = outpath.file_name().unwrap().to_string_lossy().into_owned();
                            codigos_para_avaliar.push((nome_arquivo, conteudo));
                        }
                    }

                    if codigos_para_avaliar.is_empty() {
                        relatorio_final.push_str("<p style='color:#f38ba8;'>Nenhum código .c, .java ou .py encontrado neste ZIP!</p>\n");
                        continue; // Pula para o próximo aluno
                    }

                    // Avalia os arquivos deste aluno
                    for (nome_arquivo, conteudo) in codigos_para_avaliar {
                        // Prompt Sanduíche para evitar a amnésia em códigos gigantes
                        let entrada_usuario = format!(
                            "Abaixo está o código do aluno:\n\n\
                            {}\n\n\
                            [FIM DO CÓDIGO DO ALUNO]\n\n\
                            ATENÇÃO: Não importa o tamanho do código acima, VOCÊ DEVE OBEDECER AS SEGUINTES REGRAS:\n\
                            1. RESPONDA ESTRITAMENTE EM PORTUGUÊS.\n\
                            2. IDENTIFIQUE A LINGUAGEM AUTOMATICAMENTE.\n\
                            3. NÃO REESCREVA O CÓDIGO.\n\
                            4. USE EXATAMENTE O FORMATO ABAIXO:\n\n\
                            **RESUMO DA LÓGICA:** [1 parágrafo]\n\
                            **BUGS LÓGICOS:** [Lista de erros ou 'Nenhum']\n\
                            **NOTA DE FUNCIONALIDADE:** [0 a 10]",
                            conteudo
                        );

                        let body = json!({
                            "model": "qwen2.5-coder:7b",
                            "messages": [
                                { "role": "system", "content": regra_sistema },
                                { "role": "user", "content": entrada_usuario }
                            ],
                            "stream": false
                        });

                        let res = client.post(url).header("Content-Type", "application/json").json(&body).send().await;

                        match res {
                            Ok(resposta) if resposta.status().is_success() => {
                                let json_res: serde_json::Value = resposta.json().await.unwrap_or_default();
                                let resposta_ia = json_res["message"]["content"].as_str().unwrap_or("Falha ao ler texto.").to_string();
                                
                                relatorio_final.push_str(&format!(
                                    "<h3>📄 Arquivo: <span style='color:#f9e2af;'>{}</span></h3>\n{}\n<hr style='border: 1px solid #45475a;'>\n", 
                                    nome_arquivo, 
                                    resposta_ia
                                ));
                            }
                            _ => {
                                relatorio_final.push_str(&format!("<p style='color:red;'>Erro ao comunicar com a IA para {}</p>", nome_arquivo));
                            }
                        }
                    }
                }
            }

            if relatorio_final.is_empty() {
                Ok("Nenhum arquivo ZIP foi encontrado na pasta selecionada.".to_string())
            } else {
                Ok(relatorio_final)
            }
        }
        None => Err("Seleção de pasta cancelada pelo usuário.".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        // Registra o novo nome da função
        .invoke_handler(tauri::generate_handler![avaliar_turma])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o motor Tauri");
}