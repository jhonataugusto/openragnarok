# English Asset Migration Design

Data: 2026-04-28
Status: aprovado em conversa; aguardando revisao do arquivo antes do plano de implementacao
Projeto: Ragnarok / Korangar / rAthena

## Objetivo

Migrar os assets e dados visiveis do jogo de coreano para ingles, incluindo nomes internos de arquivos/pastas quando viavel, sem quebrar as referencias cruzadas entre Korangar, assets Ragnarok e rAthena.

O valor central e transformar a traducao em um processo controlado por manifesto: subagents simples traduzem lotes pequenos, mas nao editam o jogo diretamente. O agente orquestrador valida os manifestos, aplica as mudancas por scripts, preserva aliases temporarios e executa checkpoints de integridade.

## Escopo

Incluido:

- Caminhos de assets com Hangul em `korangar/korangar/archive/data/`.
- Textos e nomes carregados por Korangar em RON, Lua e LUB.
- Imagens, cutins, loading screens, livros, cards e texturas com texto coreano.
- Referencias internas em mapas/modelos/sprites/actions quando tecnicamente suportado.
- Referencias rAthena em NPC scripts, mensagens, DBs e configs quando afetarem texto exibido ou asset referenciado pelo cliente.
- Compatibilidade com `PACKETVER=20220406`.

Fora do escopo inicial:

- Alterar protocolo ou packet version.
- Reescrever renderizacao, networking ou arquitetura central do Korangar.
- Alterar rAthena C++ preventivamente.
- Remover assets coreanos antes de existir validacao automatica e manual.
- Garantir traducao perfeita de lore em um primeiro passe; o primeiro alvo e consistencia tecnica e ingles compreensivel.

## Principios

1. Manifesto antes de mudanca fisica.
2. Subagents traduzem; scripts aplicam; orquestrador decide.
3. Nunca renomear arquivo sem atualizar todas as referencias conhecidas.
4. Manter aliases coreanos temporarios ate os checkpoints passarem.
5. Preferir alteracoes reversiveis e por lote.
6. Separar traducao de texto visivel de renomeacao de path tecnico.
7. Tratar assets binarios como risco alto: ler, mapear e validar antes de editar.

## Arquitetura

O fluxo tera quatro camadas.

### 1. Inventory Scanner

Responsavel por gerar inventario completo de arquivos, paths coreanos e referencias.

Entradas:

- `korangar/korangar/archive/data/`
- `korangar/korangar/client/game_archives.ron`
- `korangar/korangar/src/world/library/`
- `rathena-master/npc/`
- `rathena-master/db/`
- `rathena-master/conf/`

Saidas:

- `docs/asset-migration/inventory/files.csv`
- `docs/asset-migration/inventory/korean-paths.csv`
- `docs/asset-migration/inventory/references.csv`
- `docs/asset-migration/inventory/impact-summary.md`

### 2. Translation Manifest

Formato canonico para os subagents preencherem.

Campos minimos:

```csv
batch_id,kind,source_path,target_path,source_text,target_text,confidence,needs_human_review,notes
```

Regras:

- `source_path` nunca muda depois de gerado.
- `target_path` deve ser ASCII, lowercase quando possivel, sem espacos novos desnecessarios.
- `target_text` deve ser ingles natural, mas tecnicamente conservador.
- `confidence` deve ser `low`, `medium` ou `high`.
- `needs_human_review=true` quando houver nome proprio, lore, ambiguidade ou colisao.

### 3. Manifest Validator

Responsavel por rejeitar manifestos perigosos antes de qualquer escrita.

Validacoes:

- Colisoes case-insensitive no Windows.
- Caracteres invalidos em path.
- Duplicatas de `target_path`.
- Extensao alterada indevidamente.
- Path de destino fora de `archive_en/` ou area autorizada.
- Referencias conhecidas sem plano de atualizacao.
- Baixa confianca em lote que seria aplicado automaticamente.

### 4. Migration Applier

Responsavel por aplicar mudancas de forma reversivel.

Fases de aplicacao:

1. Criar `archive_en/` com arquivos renomeados/traduzidos.
2. Configurar overlay em `client/game_archives.ron` somente quando a fase pedir.
3. Atualizar referencias textuais e scripts.
4. Atualizar referencias binarias apenas com ferramentas especificas e validacao posterior.
5. Regenerar ou remover `lua_files.7z` e `cache.7z` quando necessario.
6. Gerar relatorio de tudo que mudou.

## Papel Dos Subagents

Subagents trabalham em lotes pequenos e independentes. Eles nao devem editar arquivos do repo.

Tarefas permitidas:

- Traduzir path ou nome visivel.
- Classificar risco.
- Sinalizar ambiguidades.
- Sugerir glossario.
- Marcar entradas que precisam de revisao humana.

Tarefas proibidas:

- Renomear arquivos diretamente.
- Editar `.spr`, `.act`, `.rsm`, `.rsw`, `.lub`, `.grf` ou scripts.
- Decidir remocao de aliases.
- Alterar rAthena C++.
- Alterar `PACKETVER`.

## Lotes Iniciais

1. `client-ui-and-config`
   - RON, XML, loading config, UI propria do Korangar.

2. `lua-lub-visible-text`
   - `itemInfo.lub`, `SkillInfoList.lub`, `SkillDescript.lub`, `jobName.lub`, `AccName.lub`, `msgstring_kr.lub`.

3. `npc-dialog-and-cutin`
   - `rathena-master/npc/**/*.txt`, comandos `mes`, `select`, `cutin` e mensagens relacionadas.

4. `texture-visible-text`
   - Loading screens, cards, collection, UI, effects com texto renderizado.

5. `sprite-job-monster-npc-paths`
   - Paths tecnicos de sprite/action por job, monster e NPC.

6. `map-model-reference-paths`
   - `.rsw`, `.rsm`, `.rsm2`, `.gnd`, `.gat`, texturas e modelos.

## Iteracoes

### Iteracao 1: Baseline e Inventario

Criar scanners read-only e gerar o primeiro mapa de impacto. A saida precisa separar:

- Texto visivel.
- Path tecnico.
- Asset visual com texto embutido.
- Referencia rAthena.
- Binario que exige parser/ferramenta.

Criterio de aceite: inventario com contagens por pasta, extensao, risco e top referencias.

### Iteracao 2: Glossario e Manifesto

Criar glossario inicial para classes, jobs, generos, cidades, mobs, skills e termos comuns. Gerar lotes CSV pequenos para subagents.

Criterio de aceite: manifestos validam sem colisao e sem paths fora do escopo.

### Iteracao 3: Overlay Ingles Sem Remover Coreano

Aplicar apenas traducoes seguras em `archive_en/`, mantendo os arquivos coreanos originais. Configurar o loader para testar overlay quando necessario.

Criterio de aceite: cliente abre, login funciona, mapa carrega, inventario/skill tree/dialogos principais continuam operando.

### Iteracao 4: Migracao De Referencias Textuais

Atualizar referencias em arquivos texto, scripts rAthena, RON, XML e Lua source quando houver. Para LUB, usar processo controlado de conversao/geracao ou manter overlay equivalente se nao houver ferramenta segura.

Criterio de aceite: nenhuma referencia textual conhecida aponta para target inexistente.

### Iteracao 5: Migracao De Referencias Binarias

Atacar assets binarios por tipo, com parser ou ferramenta dedicada. Cada tipo tem checkpoint proprio:

- Mapas: carregar mapa e texturas.
- Modelos: carregar `.rsm`/`.rsm2`.
- Sprites/actions: carregar entidade e animacoes.
- Efeitos: executar skill/efeito em ambiente local.

Criterio de aceite: nenhum lote binario entra sem relatorio antes/depois.

### Iteracao 6: Remocao Gradual De Aliases Coreanos

Remover aliases somente quando:

- Inventario nao encontra referencias ao path antigo.
- Cliente passa smoke test manual.
- rAthena sobe sem erro de script.
- Relatorio lista fallback zero para o lote.

Criterio de aceite: paths coreanos removidos por lote, nao em massa.

## Impactos No Korangar

Areas sensiveis:

- `GameFileLoader` e ordem de arquivos em `client/game_archives.ron`.
- `FolderArchive` e normalizacao case-insensitive.
- `lua_files.7z`, que e gerado a partir dos LUB/Lua carregados.
- `cache.7z`, que depende do hash dos assets.
- `world/library`, que carrega nomes de itens, skills, jobs e recursos.
- Loaders de mapas, modelos, sprites, textures, effects e audio.
- Strings hardcoded com nomes mojibake, como sons de clique.

Risco principal: o cliente pode continuar compilando, mas falhar em runtime quando uma referencia antiga apontar para asset renomeado.

## Impactos No rAthena

Areas sensiveis:

- `npc/**/*.txt`, especialmente `mes`, `select`, `cutin` e nomes de arquivos.
- `conf/msg_conf/*.conf` e sistema `@langtype`.
- `db/re/item_db.yml`, `db/re/mob_db.yml`, `db/re/skill_db.yml`.
- `conf/battle/monster.conf`, especialmente `override_mob_names`.
- Imports locais em `docker/import/`.

Risco principal: scripts podem carregar, mas referenciar cutins ou nomes que o cliente nao encontra.

## Validacao

Checks automaticos:

- Manifesto sem colisoes.
- Todos os target paths existem.
- Nenhum source path removido antes da fase de cleanup.
- Busca por Hangul restante por pasta e extensao.
- Busca por mojibake obvio em textos visiveis.
- Smoke de rAthena: login, char e map sobem.
- Smoke de Korangar: cliente inicia ate tela de login.

Checks manuais:

- Login com `admin` / `123`.
- Entrada em mapa.
- Player sprite e animacoes basicas.
- Hover em item no chao.
- Inventario e skill tree.
- NPC dialog e escolhas.
- Cenario com cutin/loading.
- Um mob comum e um NPC visivel.

## Estimativa

Inventario e pipeline base: 2 a 4 dias.
Traducoes paralelas e manifestos: 3 a 7 dias, dependendo do volume revisado.
Overlay e referencias textuais: 3 a 5 dias.
Referencias binarias e cleanup: 1 a 3 semanas, dependendo das ferramentas e falhas encontradas.

Estimativa total realista para escopo C: 2 a 4 semanas.

## Decisao

Prosseguir com uma migracao por manifestos e subagents. O primeiro plano de implementacao deve comecar por ferramentas read-only de inventario e validacao, antes de qualquer rename.
