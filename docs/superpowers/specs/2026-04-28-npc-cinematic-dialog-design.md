# NPC Cinematic Dialog Design

**Data:** 2026-04-28
**Projeto:** Ragnarok Client Modernization
**Status:** Aprovado em conversa; aguardando revisao do arquivo antes do plano de implementacao.

## Objetivo

Adicionar ao cliente Korangar um modo opcional de dialogo cinematico com NPCs, inspirado em MMORPGs modernos. O modo deve deixar a conversa mais expressiva com camera contextual, texto em typewriter, som por letra e opcoes em baloes empilhados, sem remover o dialogo classico existente.

## Decisoes Aprovadas

- A feature sera ativada por uma configuracao global em Interface Settings.
- Quando ativa, sera aplicada a todos os dialogos de NPC.
- Durante o dialogo cinematico, movimento do personagem e controle manual de camera ficarao temporariamente travados.
- A camera sera dinamica, entrara sem cortes bruscos e tera configuracao simples de ligar/desligar.
- O texto usara typewriter; mouse esquerdo, Enter e Espaco revelam a fala inteira antes de avancar.
- O som por letra tera toggle simples ligado/desligado, usando volume de efeitos e variacao automatica de pitch.
- As opcoes de resposta aparecerao como baloes empilhados acima da caixa de dialogo, centralizadas perto da parte inferior.
- Quando o modo cinematico estiver ativo, a UI cinematica substitui visualmente a janela classica.
- A implementacao deve criar um caminho novo de UI sobre os mesmos eventos/protocolo, preservando a `DialogWindow` classica como fallback.

## Arquitetura

O protocolo e o rAthena nao mudam. Os eventos atuais de NPC continuam sendo a fonte de verdade:

- `NetworkEvent::OpenDialog { text, npc_id }`
- `NetworkEvent::AddNextButton { npc_id }`
- `NetworkEvent::AddCloseButton { npc_id }`
- `NetworkEvent::AddChoiceButtons { choices, npc_id }`

Quando o modo cinematico estiver desligado, o fluxo atual abre `DialogWindow` como hoje. Quando estiver ligado, os mesmos eventos alimentam um novo estado de dialogo cinematico e abrem uma janela nova, por exemplo `CinematicDialogWindow`.

A feature deve ser separada em tres responsabilidades:

- Settings persistentes em `InterfaceSettings`.
- Estado de dialogo cinematico em um tipo novo, por exemplo `CinematicDialogWindowState`.
- UI de renderizacao e eventos em uma janela nova, por exemplo `CinematicDialogWindow`.

A camera cinematica pertence ao runtime do `Client`, porque precisa coordenar entidades do mundo, input e render frame.

## Camera E Input

Ao iniciar um dialogo cinematico, o cliente tenta resolver:

- O player local via `this_entity()`.
- O NPC alvo pelo `npc_id` do dialogo.

Se ambos existirem, a camera entra suavemente em modo cinematico. O enquadramento deve ficar atras de onde o personagem esta olhando, levemente deslocado para a direita, com a direcao apontando para o NPC. O foco deve considerar player e NPC para manter os dois legiveis na cena.

Com a camera dinamica ligada, ela continua fazendo ajustes suaves enquanto o dialogo estiver ativo. A configuracao deve ser simples: ligada/desligada, sem controles granulares de distancia, offset ou suavizacao.

Durante o modo cinematico:

- Movimento do personagem fica bloqueado.
- Rotacao e zoom manuais da camera ficam bloqueados.
- Mouse esquerdo, Enter e Espaco controlam revelar/avancar.
- Ao fechar o dialogo, a camera solta o controle e retorna suavemente ao comportamento normal do player.

Se player ou NPC nao puderem ser resolvidos com seguranca, o cliente deve usar o dialogo classico como fallback.

## UI E Texto

A UI cinematica substitui visualmente a janela classica enquanto o modo estiver ativo. A composicao fica na parte inferior da tela:

- Caixa de dialogo cinematica larga para o texto.
- Opcoes de resposta como baloes empilhados acima da caixa, centralizados perto da parte inferior.
- Acoes de Next/Close podem ser representadas de forma discreta pela propria caixa, ja que mouse esquerdo, Enter e Espaco controlam o fluxo.

O texto usa typewriter sempre no modo cinematico. Se o usuario acionar enquanto o texto ainda esta aparecendo, a fala inteira aparece imediatamente. Se o texto ja estiver completo, a mesma acao avanca, fecha ou confirma a opcao selecionada.

As opcoes de resposta devem continuar enviando `InputEvent::ChooseDialogOption { npc_id, option }`, preservando a numeracao de opcoes usada pelo servidor.

Defaults de ritmo:

- Velocidade inicial: cerca de 35 caracteres por segundo.
- Espacos nao precisam tocar som.
- Pontuacao pode ter pausa levemente maior, desde que nao deixe NPC repetitivo lento demais.
- A revelacao instantanea deve completar o texto sem disparar sons adicionais para as letras restantes.

## Audio

O som por letra deve ser controlado por um toggle simples em Interface Settings. Ele usa o volume normal de efeitos sonoros.

Durante typewriter, cada letra visivel pode tocar um som curto com variacao pequena de pitch para evitar repeticao mecanica. Quando o texto for revelado instantaneamente, nao deve tocar um som para cada letra restante.

Defaults de audio:

- Tocar som apenas para caracteres visiveis nao-espaco.
- Limitar o disparo de sons para evitar rajadas em textos muito rapidos.
- Pitch alvo entre aproximadamente 0.94x e 1.06x.
- Volume segue o canal normal de efeitos; nao ha volume separado nesta primeira versao.

Dependencia tecnica: o `AudioEngine` atual expoe `play_sound_effect(key)` sem parametro publico de pitch. A implementacao final de pitch pode exigir uma pequena extensao no `korangar-audio`. Se isso se provar grande demais, o MVP deve ainda manter a arquitetura pronta para pitch e documentar qualquer entrega intermediaria sem variacao real.

## Configuracoes

Adicionar configuracoes em `InterfaceSettings`:

- `npc_cinematic_dialog_enabled`: liga/desliga o modo cinematico de NPC.
- `npc_cinematic_dynamic_camera_enabled`: liga/desliga a camera dinamica dentro do modo cinematico.
- `npc_cinematic_text_sound_enabled`: liga/desliga som por letra.

Os nomes finais podem seguir a convencao local do projeto, mas devem preservar essa separacao funcional.

## Fallbacks E Erros

O modo classico deve ser usado quando:

- A opcao global de dialogo cinematico estiver desligada.
- O player local nao puder ser identificado.
- O NPC do `npc_id` nao puder ser encontrado entre as entidades atuais.
- Algum estado necessario do dialogo cinematico estiver inconsistente.

O fallback nao deve alterar pacotes enviados ao servidor. Ele apenas escolhe a apresentacao visual/input local.

## Testes E Verificacao

Verificacao minima:

- Com a opcao desligada, dialogos de NPC continuam usando a janela classica atual.
- Com a opcao ligada, `OpenDialog`, Next, Close e escolhas aparecem na UI cinematica.
- Mouse esquerdo, Enter e Espaco revelam texto parcial e depois avancam.
- Opcoes enviam o indice correto ao servidor.
- Movimento e camera manual ficam bloqueados durante o dialogo cinematico e voltam ao normal ao fechar.
- Quando NPC/player nao sao encontrados, o dialogo classico aparece como fallback.

Testes automatizados devem priorizar partes puras do estado de dialogo, como typewriter, revelacao imediata, transicao de botoes e escolha de fallback. A validacao de camera/UI deve incluir teste manual no cliente real.

## Fora De Escopo

- Alterar `rathena-master/`.
- Remover definitivamente a `DialogWindow` classica.
- Criar controles granulares de camera.
- Criar sistema de cinematicas geral para quests/cutscenes.
- Criar suite E2E completa de NPC/dialogo nesta primeira iteracao.

## Perguntas Fechadas

Nenhuma pergunta de produto permanece aberta neste momento. A principal incerteza tecnica e o suporte a pitch no audio engine.
