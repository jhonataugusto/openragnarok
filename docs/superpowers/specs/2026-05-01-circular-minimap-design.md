# Circular Minimap Design

Data: 2026-05-01
Projeto: Ragnarok Client Modernization / Korangar
Status: Aguardando revisao do usuario antes do plano de implementacao

## Objetivo

Adicionar um minimapa circular no HUD do cliente Korangar, no canto superior direito, usando as texturas de mapa ja existentes nos assets do Ragnarok. O minimapa funciona como radar: o jogador fica fixo no centro, o mapa desliza por baixo conforme a posicao muda, e o usuario pode ajustar o zoom com scroll quando o cursor estiver sobre o circulo.

A rotacao de bussola deve acontecer somente no modo terceira pessoa. Quando a rotacao estiver habilitada e a camera terceira pessoa estiver ativa, o disco do mapa gira conforme a direcao para onde a camera esta olhando. Nos outros modos de camera, o minimapa permanece norte-fixo.

## Escopo

Incluido:

- Configuracao para ligar/desligar o minimapa circular.
- Configuracao para permitir rotacao do minimapa em terceira pessoa.
- HUD no canto superior direito.
- Recorte circular local ao redor do jogador.
- Jogador fixo no centro com marcador circular simples.
- Zoom por scroll quando o cursor estiver sobre o minimapa.
- Uso das texturas existentes no diretorio de UI de mapas dos assets do Ragnarok, com caminho real resolvido via `TextureLoader`.
- Direcoes `NORTH`, `SOUTH`, `EAST`, `WEST` desenhadas como parte do disco, girando junto com o mapa quando aplicavel.
- Fallback gracioso quando a textura do mapa atual nao existir.

Fora de escopo para a primeira entrega:

- Marcadores de NPCs, monstros, party, warps ou quests.
- Janela grande de mapa.
- Edicoes em `rathena-master/`.
- Reestrutura ampla do renderer ou de `main.rs`.
- Dependencia em `entities()[0]` para identificar o player local sem usar o caminho seguro existente.

## Configuracao

Adicionar dois campos em `InterfaceSettings`:

- `circular_minimap_enabled: bool`
- `circular_minimap_rotate_in_third_person: bool`

Os campos devem ter defaults compativeis com arquivos antigos de `client/interface_settings.ron` via `serde(default = "...")`.

Recomendacao de defaults:

- `circular_minimap_enabled`: `false`, para nao mudar HUD de quem ja usa o cliente.
- `circular_minimap_rotate_in_third_person`: `true`, para que a feature tenha o comportamento esperado quando o minimapa for ativado.

Adicionar os toggles em `InterfaceSettingsWindow`:

- `Circular minimap`
- `Rotate minimap in third person`

## Comportamento

Quando `Circular minimap` estiver desligado:

- Nenhum elemento de minimapa e desenhado.
- Scroll do mouse segue o comportamento normal do jogo/UI.

Quando `Circular minimap` estiver ligado:

- O minimapa aparece no canto superior direito.
- O mapa e renderizado como um recorte circular local ao redor do jogador.
- O marcador do jogador fica sempre no centro.
- Movimento do personagem desloca a textura do mapa por baixo do marcador.
- Scroll sobre o circulo altera o zoom dentro de limites definidos.
- Scroll fora do circulo nao altera o zoom do minimapa.

Quando `Rotate minimap in third person` estiver ligado:

- Se o modo terceira pessoa estiver ativo, o disco do minimapa gira usando a direcao horizontal de `ThirdPersonCamera::view_direction()`.
- `NORTH`, `SOUTH`, `EAST`, `WEST` giram junto com o mapa.
- O marcador central do jogador nao gira.

Quando o cliente nao estiver em terceira pessoa:

- O minimapa fica norte-fixo, mesmo que a config de rotacao esteja ligada.

## Arquitetura

Implementar como uma camada HUD propria, nao como `CustomWindow`.

Motivos:

- O minimapa precisa de recorte circular real.
- O mapa precisa rotacionar e deslocar por UV.
- O marcador central deve ficar fixo acima do disco.
- O scroll deve ser consumido apenas dentro da area circular.
- Uma janela tradicional do sistema de UI atual favorece retangulos e controles, mas nao esse comportamento de radar.

Componentes propostos:

- Estado pequeno de runtime para zoom e textura do mapa atual.
- Helpers puros para:
  - converter posicao do player em UV do mapa;
  - limitar zoom;
  - calcular se o cursor esta dentro do circulo;
  - calcular angulo de rotacao a partir da direcao horizontal da camera.
- Renderizador HUD de minimapa que recebe:
  - textura do mapa atual;
  - tamanho do mapa;
  - posicao do player;
  - zoom atual;
  - flag de rotacao;
  - direcao da camera terceira pessoa quando disponivel.

Se o renderer retangular atual nao suportar mascara circular e rotacao de UV de forma limpa, adicionar uma instrucao/pass pequena e especifica para minimapa. Essa extensao deve ficar isolada e nao virar refatoracao ampla do pipeline.

## Assets

Fonte principal das texturas:

Diretorio de UI de mapas dos assets do Ragnarok, no formato logico `<ui-map-directory>\<map>.bmp`. O codigo deve centralizar esse prefixo em uma constante para evitar espalhar caminho com caracteres nao ASCII.

Exemplos encontrados:

- `prontera.bmp`
- `geffen.bmp`
- `izlude.bmp`
- `alberta.bmp`
- `payon.bmp`

O nome base do mapa deve ser derivado do mapa atual removendo extensoes como `.gat` quando necessario.

Fallback:

- Se a textura nao existir, o cliente nao deve quebrar.
- A primeira entrega deve ocultar a textura ausente e manter apenas um disco neutro discreto.
- Logs sobre asset ausente devem ser debug-only ou silenciosos para nao gerar ruido permanente.

## Renderizacao

Layout inicial:

- Canto superior direito.
- Tamanho fixo responsivo razoavel, por exemplo 180 px, ajustado pela escala de interface se isso combinar com o renderer existente.
- Margem da borda da tela seguindo o estilo do HUD atual.

Ordem de desenho:

1. Fundo/aro do minimapa.
2. Textura do mapa com offset, zoom e rotacao.
3. Letras `NORTH`, `SOUTH`, `EAST`, `WEST` no mesmo espaco rotacionado do disco.
4. Mascara/recorte circular.
5. Marcador circular do jogador no centro.

O recorte local deve usar o jogador como centro. O zoom define quantos tiles ou unidades de mapa cabem no raio visivel.

## Input

Scroll sobre o minimapa:

- Detectar se o cursor esta dentro do circulo.
- Aplicar delta de zoom ao estado do minimapa.
- Consumir o evento para evitar conflito com zoom da camera.

Scroll fora do minimapa:

- Manter comportamento atual.

Limites de zoom:

- Definir minimo e maximo para evitar uma visao inutilmente fechada ou aberta.
- Os limites devem ser constantes simples na primeira entrega.

## Testes E Verificacao

Testes unitarios recomendados:

- Conversao de posicao do jogador para UV do mapa.
- Clamp de zoom minimo/maximo.
- Deteccao de cursor dentro/fora do circulo.
- Angulo de rotacao a partir de `view_direction`.
- Regra de rotacao: so rotaciona quando terceira pessoa esta ativa e a config esta ligada.

Verificacao manual:

- Em mapa com textura conhecida, ativar `Circular minimap`.
- Andar para norte, sul, leste e oeste e confirmar que o marcador fica no centro e o mapa desliza.
- Usar scroll sobre o minimapa e confirmar mudanca de zoom.
- Usar scroll fora do minimapa e confirmar que o comportamento normal permanece.
- Ativar terceira pessoa, girar a camera e confirmar que o disco gira seguindo a direcao de visao da camera.
- Sair da terceira pessoa e confirmar que o disco fica estatico/norte-fixo.
- Desligar `Circular minimap` e confirmar que o HUD desaparece sem reiniciar o cliente.
- Desligar `Rotate minimap in third person` e confirmar que o mapa fica estatico mesmo em terceira pessoa.

## Riscos

- Alguns mapas podem nao ter textura correspondente no diretorio de UI de mapas.
- A orientacao dos BMPs pode exigir ajuste de eixo Y ou calibragem entre coordenadas do mapa e UV.
- O renderer atual pode precisar de uma instrucao nova para mascara circular e rotacao.
- A posicao local do player deve vir de um caminho confiavel ja existente, nao de uma suposicao fragil sobre a ordem de entidades.

## Criterios De Aceite

- O minimapa pode ser ligado/desligado em `Interface Settings`.
- O minimapa aparece no canto superior direito quando ligado.
- O player aparece como circulo simples fixo no centro.
- O mapa se move sob o marcador conforme o player anda.
- Scroll sobre o minimapa altera zoom dentro de limites.
- Fora da terceira pessoa, o mapa permanece norte-fixo.
- Em terceira pessoa, com rotacao ligada, o disco gira conforme a direcao visual da camera.
- `NORTH`, `SOUTH`, `EAST`, `WEST` giram junto com o disco.
- Ausencia de textura de mapa nao causa crash.
