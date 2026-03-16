const invoke = window.__TAURI__.core.invoke;

window.addEventListener("DOMContentLoaded", () => {
    document.getElementById('btn-selecionar').addEventListener('click', avaliarTurma);
});

async function avaliarTurma() {
    const respostaDiv = document.getElementById('resposta-ia');
    
    // Aviso atualizado para processamento em lote
    respostaDiv.innerHTML = "<em style='color: #f9e2af;'>Analisando a pasta inteira. O Qwen está corrigindo os alunos um por um. Pegue um café, isso pode levar alguns minutos...</em>";

    try {
        // Chama a nova função do Rust
        const resultadoRaw = await invoke('avaliar_turma');
        
        let resultadoFormatado = resultadoRaw
            .replace(/```[a-zA-Z]*\n([\s\S]*?)```/g, '<pre><code>$1</code></pre>') 
            .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>') 
            .replace(/\*(.*?)\*/g, '<em>$1</em>') 
            .replace(/\n/g, '<br>'); 

        respostaDiv.innerHTML = resultadoFormatado;

    } catch (error) {
        respostaDiv.innerHTML = `<strong style="color: #f38ba8;">Erro do Sistema:</strong> ${error}`;
    }
}