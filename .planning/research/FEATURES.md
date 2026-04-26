# Pesquisa de Features

**Domínio:** correção investigativa brownfield de bug visual de respawn no workspace Korangar + rAthena
**Pesquisado em:** 2026-04-26
**Confiança:** ALTA

## Panorama de Capacidades

Este não é um roadmap de produto novo. O v1 precisa provar, com evidência prática, que o bug conhecido foi reproduzido, entendido o suficiente para uma correção mínima, corrigido no ponto certo e validado manualmente no fluxo real `@kill -> Respawn`.

As capacidades abaixo estão organizadas para alimentar REQ IDs de `REQUIREMENTS.md`. A recomendação é manter quatro grupos de requisitos: **Reprodução**, **Evidência**, **Correção** e **Verificação Manual**. Qualquer coisa que não ajude diretamente esses quatro grupos deve ser diferida.

### Table Stakes (Usuários Esperam Isso)

Recursos indispensáveis para considerar o v1 completo. Se faltar algum, a correção não fica comprovada.

| Capacidade | Por que é esperada | Complexidade | Notas |
|------------|--------------------|--------------|-------|
| Reprodução controlada do bug atual | Antes de alterar código, o usuário precisa confirmar que o workspace ainda manifesta o problema real: personagem com HP/movimento, mas visualmente morto | MÉDIA | Usar o ambiente local documentado: Docker rAthena, Korangar via `play.bat`, conta de teste e fluxo `@kill -> Respawn` |
| Roteiro de reprodução explícito | A correção só é confiável se outra sessão conseguir repetir o mesmo cenário antes/depois | BAIXA | Registrar passos, conta usada, mapa/personagem se relevante, comando GM usado e resultado visual observado |
| Evidência do fluxo de respawn recebido pelo cliente | O bug pode estar em `ResurrectPlayer`, `ChangeMap`, status/HP ou ordem de pacotes; o v1 precisa distinguir essas possibilidades | MÉDIA | Capturar logs temporários, inspeção debug ou packet history suficiente para saber se houve `ZC_RESURRECTION`, mudança de mapa/posição e/ou status HP=0 pós-respawn |
| Evidência da entidade correta | Tentativas anteriores mexeram em `entities().first_mut()`, mas o player real pode precisar ser localizado por `this_entity()` | MÉDIA | Confirmar qual entidade recebe `set_dead`, `set_idle`, `stop_movement` e qual entidade é renderizada como player local |
| Evidência da transição visual interna | Não basta HP cheio; o estado de animação/renderização precisa voltar de morto para vivo | MÉDIA | Observar ou logar estado antes/depois de morte, respawn e qualquer pacote posterior que possa sobrescrever a animação |
| Correção mínima e localizada | O workspace é frágil, especialmente `main.rs`; o v1 deve reduzir risco de regressão em login, mapa, inventário e combate | MÉDIA | Preferir helper pequeno de ciclo de vida do player ou ajuste pontual no handler confirmado, sem refatorar o hub inteiro |
| Fechamento correto da janela Respawn | A janela persistente é parte visível do bug e sinal de estado morto residual | BAIXA | A validação deve exigir que `WindowClass::Respawn` não permaneça aberta nem reabra indevidamente após o respawn |
| Verificação manual final do fluxo real | O critério principal é visual e interativo; testes unitários isolados não bastam para provar o bug | MÉDIA | Validar `@kill -> Respawn`: player em pé/vivo, HP restaurado, movimento funcional, janela fechada |
| Registro do resultado e da causa provável | A próxima fase precisa saber o que foi comprovado, não apenas que "parece funcionar" | BAIXA | Documentar se a causa ficou no cliente, no servidor, ou na integração/pacote ausente, com arquivos alterados e evidências usadas |

### Diferenciadores (Vantagem Prática)

Não são todos obrigatórios para o v1, mas aumentam muito a qualidade da investigação sem transformar o trabalho em uma plataforma permanente.

| Capacidade | Proposta de valor | Complexidade | Notas |
|------------|-------------------|--------------|-------|
| Instrumentação temporária com prefixos claros | Acelera a identificação da ordem real de eventos sem poluir o código final | BAIXA | Usar logs removíveis ou gated por build/debug; remover ou reduzir antes de finalizar se não forem úteis permanentemente |
| Pequeno teste unitário para helper de ciclo de vida | Protege a regra local se a correção extrair lógica pura do player lifecycle | MÉDIA | Só vale se a correção criar função testável; não bloquear v1 tentando automatizar o cliente inteiro |
| Comparação entre respawn por botão e `@alive` | Ajuda a saber se a falha é específica de save point, resurrection packet ou status update | MÉDIA | Tratar como validação ampliada se o fluxo principal já estiver claro |
| Evidência antes/depois em texto ou screenshot curta | Facilita revisão humana do resultado visual | BAIXA | Útil para documentação da fase; não precisa virar ferramenta automática |

### Anti-Features (Comumente Pedidas, Mas Problemáticas)

Itens que parecem bons, mas ampliam o escopo e atrasam a correção mínima comprovada.

| Capacidade | Por que seria pedida | Por que é problemática | Alternativa |
|------------|----------------------|------------------------|-------------|
| Suite E2E completa de login, mapa, morte e respawn | Daria confiança ampla no jogo inteiro | Alto custo, sem harness existente, depende de Docker, assets, UI gráfica e estado de servidor | Fazer verificação manual v1 e, no máximo, teste unitário pequeno se a correção permitir |
| Instrumentação permanente de packet tracing | Ajudaria futuras investigações de protocolo | Pode poluir UI/código, exigir UX própria e manutenção contínua | Usar logs temporários ou feature debug existente apenas durante a investigação |
| Refatoração ampla de `main.rs` por domínio | O arquivo é grande e concentra handlers demais | Risco alto de regressão fora do bug de respawn | Criar helper mínimo apenas se necessário para corrigir o ciclo morte/respawn |
| Corrigir o bug de login-server disconnect junto | Também é relevante no workspace | Já foi mitigado e não bloqueia a prova do respawn visual | Manter fora do v1; reabrir em fase própria se necessário |
| Resolver todos os pacotes não implementados | Melhoraria robustez geral de protocolo | Escopo amplo e sem relação direta com o sintoma visual | Implementar ou tratar somente pacote diretamente necessário ao respawn, se comprovado |
| Alterar rAthena preventivamente para sempre enviar resurrection | Pode parecer uma correção direta | Pode mascarar causa no cliente e divergir do fluxo real de save-point respawn | Só alterar servidor se a evidência provar que o contrato necessário está ausente ou incompatível |
| Criar sistema de replay/recording de pacotes | Facilitaria regressões futuras | É infraestrutura nova, maior que o bug | Guardar evidência textual/manual no v1; avaliar replay em v2 |

## Dependências de Capacidades

```text
Reprodução controlada do bug atual
    -> Roteiro de reprodução explícito
        -> Evidência do fluxo de respawn recebido pelo cliente
            -> Evidência da entidade correta
                -> Evidência da transição visual interna
                    -> Correção mínima e localizada
                        -> Verificação manual final do fluxo real
                            -> Registro do resultado e da causa provável

Fechamento correto da janela Respawn
    -> Verificação manual final do fluxo real

Instrumentação temporária
    -> Evidência do fluxo de respawn recebido pelo cliente
    -> Evidência da transição visual interna

Suite E2E completa
    - conflita com -> Escopo mínimo do v1
```

### Notas de Dependência

- **Reprodução antes de correção:** tentativas anteriores em `ResurrectPlayer` e `ChangeMap` não resolveram; mudar código sem reproduzir pode repetir tentativa cega.
- **Evidência antes de escolher cliente ou servidor:** o rAthena pode representar respawn por save point como mudança de mapa/posição, enquanto o cliente pode manter estado visual morto por entidade errada ou status posterior.
- **Entidade correta antes de reset visual:** aplicar `set_idle()` em `entities().first_mut()` não comprova que o player local foi resetado.
- **Verificação visual depois da correção:** HP e movimento funcionando não são critério suficiente; o bug é visual e de UI.

## Definição de MVP

### Entregar no v1

Mínimo necessário para validar que o bug foi corrigido no workspace local.

- [ ] **REP-001 Reprodução controlada:** subir rAthena local, abrir Korangar pelo caminho correto e reproduzir o bug atual com `@kill -> Respawn`.
- [ ] **REP-002 Roteiro de reprodução:** registrar passos exatos e resultado observado antes da correção.
- [ ] **EVI-001 Fluxo de pacotes/eventos:** capturar evidência suficiente para saber se o respawn chegou como `ResurrectPlayer`, `ChangeMap`, status/HP ou combinação.
- [ ] **EVI-002 Entidade do player local:** confirmar se a entidade alterada é a mesma renderizada como player local, preferindo identificação por `this_entity()` quando aplicável.
- [ ] **EVI-003 Ordem de sobrescrita:** confirmar se algum handler posterior reexecuta `set_dead()` ou reabre `WindowClass::Respawn` após o respawn.
- [ ] **FIX-001 Correção mínima:** aplicar mudança pequena no ponto comprovado, seja cliente, servidor ou integração, sem refatoração ampla.
- [ ] **FIX-002 UI de respawn consistente:** garantir que a janela de respawn fecha e não fica persistente depois do retorno vivo.
- [ ] **VER-001 Verificação manual principal:** validar que depois de `@kill -> Respawn` o personagem aparece em pé/vivo, com movimento funcional e janela fechada.
- [ ] **VER-002 Evidência pós-correção:** registrar resultado antes/depois e arquivos alterados para revisão da fase.

### Adicionar após validação (v1.x)

Recursos úteis depois que o bug principal estiver comprovadamente resolvido.

- [ ] **TEST-001 Teste unitário de helper de lifecycle:** adicionar se a correção criar função pura ou boundary testável.
- [ ] **VER-003 Cenários secundários:** validar também `@alive`, PvP e troca de mapa quando o fluxo principal estiver estável.
- [ ] **DOC-001 Nota de diagnóstico permanente:** documentar a causa raiz e o contrato esperado entre rAthena e Korangar.
- [ ] **CFG-001 Comando de rebuild/verificação:** facilitar repetir o ambiente se a correção tocar rAthena ou packet definitions.

### Considerar no futuro (v2+)

Itens explicitamente diferidos para não expandir o v1.

- [ ] **E2E-001 Smoke test automatizado de login -> mapa -> morte -> respawn:** alto valor futuro, mas depende de harness gráfico/servidor que não existe.
- [ ] **OBS-001 Packet tracing permanente:** útil para protocolo, mas deve ser desenhado como ferramenta própria.
- [ ] **ARCH-001 Refatoração de handlers de `main.rs`:** desejável para manutenção, mas não deve ser pré-requisito da correção mínima.
- [ ] **PROTO-001 Matriz de compatibilidade de packet version:** importante para evolução, fora do bug pontual.
- [ ] **SERVER-001 Perfil reproduzível de patches rAthena:** valioso para colaboração, mas não necessário para provar a correção visual.

## Matriz de Priorização

| Capacidade | Valor para o usuário | Custo de implementação | Prioridade |
|------------|----------------------|------------------------|------------|
| Reprodução controlada do bug atual | ALTO | MÉDIO | P1 |
| Roteiro de reprodução explícito | ALTO | BAIXO | P1 |
| Evidência do fluxo de respawn | ALTO | MÉDIO | P1 |
| Evidência da entidade correta | ALTO | MÉDIO | P1 |
| Evidência de sobrescrita por status/HP | ALTO | MÉDIO | P1 |
| Correção mínima e localizada | ALTO | MÉDIO | P1 |
| Fechamento correto da janela Respawn | ALTO | BAIXO | P1 |
| Verificação manual final | ALTO | MÉDIO | P1 |
| Registro do resultado e causa provável | MÉDIO | BAIXO | P1 |
| Teste unitário de helper | MÉDIO | MÉDIO | P2 |
| Validação de `@alive` e PvP | MÉDIO | MÉDIO | P2 |
| Instrumentação permanente | MÉDIO | ALTO | P3 |
| Suite E2E completa | ALTO | ALTO | P3 |
| Refatoração ampla de `main.rs` | MÉDIO | ALTO | P3 |

**Chave de prioridade:**
- **P1:** obrigatório para v1.
- **P2:** adicionar se surgir naturalmente ou depois da prova principal.
- **P3:** futuro; não bloquear a correção mínima.

## Análise de Referência Interna

| Capacidade | Estado atual no workspace | Abordagem recomendada |
|------------|---------------------------|-----------------------|
| Reproduzir bug | Repro básico documentado em `AI_CONTEXT.md`, mas ainda não validado como baseline da fase | Reexecutar e registrar antes de alterar |
| Reset visual em `ResurrectPlayer` | Tentativa já existe, mas não resolveu | Verificar entidade e ordem de eventos antes de mexer |
| Reset visual em `ChangeMap` | Tentativa usa `entities().first_mut()` e pode atingir entidade errada | Usar evidência para resetar o player local correto |
| Fechar janela Respawn | Tentativas existem, mas janela pode reabrir | Identificar handler que abre/reabre por HP/status |
| Testes automatizados | Há muitos testes Rust unitários, mas nenhum E2E do fluxo Korangar/rAthena | Não tentar criar E2E no v1; usar manual + teste unitário somente se barato |

## Fontes

- `.planning/PROJECT.md` - contexto, valor central, requisitos ativos e fora de escopo.
- `.planning/codebase/CONCERNS.md` - bug conhecido, áreas frágeis, lacunas de teste e riscos de escopo.
- `.planning/codebase/TESTING.md` - padrões atuais de teste e ausência de E2E para Korangar/rAthena.
- `AI_CONTEXT.md` - reprodução, tentativas anteriores, hipóteses e comandos operacionais.
- `C:/Users/jhonata/.codex/get-shit-done/templates/research-project/FEATURES.md` - estrutura de saída solicitada.

---
*Pesquisa de features para: correção investigativa do respawn visual Korangar + rAthena*
*Pesquisado em: 2026-04-26*
