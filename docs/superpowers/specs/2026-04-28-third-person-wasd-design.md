# Third-Person WASD Movement Design

## Contexto

O Korangar hoje usa câmera de jogador com rotação por mouse e movimento por clique em tiles. O novo modo deve conviver com esse estilo clássico: quando a configuração estiver desligada, o comportamento atual permanece. Quando ligada, o cliente passa a oferecer uma câmera em terceira pessoa atrás do player e movimento por `W/A/S/D`, sem alterar protocolo, `PACKETVER` ou servidor rAthena.

## Objetivo

Adicionar um modo opcional de terceira pessoa no cliente Korangar em que:

- a câmera fica em perspectiva atrás do player, usando orientação/FOV equivalentes à DebugCamera em vez de enquadramento isométrico/top-down;
- a câmera fica 1 célula/metro de jogo acima do ponto de foco do player;
- a câmera gira somente com botão direito segurado;
- a câmera segue o player com suavização mais lenta que a câmera atual;
- a distância da câmera é suavizada e limitada entre cerca de 1 e 5 células/metros de jogo;
- `W/A/S/D` envia movimento por tile com base na direção horizontal da câmera;
- diagonais como `W+A`, `W+D`, `S+A` e `S+D` são suportadas;
- clique no chão não envia movimento enquanto o modo estiver ativo;
- todo o resto do gameplay continua enviando pacotes normalmente.

## Fora de Escopo

- Alterar `rathena-master/`.
- Mudar `PACKETVER=20220406`.
- Criar movimento analógico, colisão client-authoritative ou gameplay de volante.
- Bloquear interação com NPCs, monstros, skills, itens, UI, chat ou atalhos.
- Remover o estilo clássico de clique para andar.

## Configuração

Adicionar uma opção global em `InterfaceSettings`, por exemplo `third_person_movement_enabled`, exposta na janela de Interface Settings.

Valor padrão: `false`.

Quando `false`, o cliente usa o comportamento clássico atual.
Quando `true`, o cliente ativa câmera terceira pessoa e entrada WASD para movimento.

## Câmera

Criar uma câmera nova, por exemplo `ThirdPersonCamera`, separada de `PlayerCamera` para manter o comportamento antigo intacto.

A câmera deve:

- seguir o player com foco suavizado usando atraso maior que o `PlayerCamera`;
- manter distância suavizada com clamp entre `GAT_TILE_SIZE` e `GAT_TILE_SIZE * 5.0`;
- usar yaw horizontal controlado pelo arrasto do mouse com botão direito segurado;
- usar pitch vertical com clamp simétrico para impedir olhar demais para cima ou para baixo;
- calcular `view_direction()` normalmente para renderização e para o input WASD;
- não girar o player diretamente quando a câmera gira.

Valores exatos de distância, pitch e suavização podem começar conservadores e ser ajustados manualmente depois.

## Movimento WASD

Enquanto o modo estiver ativo, `W/A/S/D` deve gerar `InputEvent::PlayerMove { destination }`.

Regras:

- `W`: 5 tiles para frente em relação à direção horizontal da câmera.
- `S`: 5 tiles para trás.
- `A`: 5 tiles para esquerda.
- `D`: 5 tiles para direita.
- Combinações diagonais normalizam o vetor antes de calcular o destino, mantendo alvo de 5 tiles em vez de somar distâncias.
- O primeiro envio acontece imediatamente quando uma tecla de movimento começa a ser pressionada.
- Enquanto a tecla continua segurada, novos destinos são enviados de forma adaptativa: imediatamente no primeiro envio, depois somente se o destino mudou e ao menos 40ms se passaram.
- Ao soltar todas as teclas de movimento, o cliente não envia pacote de parada; apenas para de gerar novos destinos.
- O player segue até o último destino aceito pelo servidor, como acontece ao parar de segurar o clique do mouse.
- Se a câmera girar enquanto uma tecla está segurada, o próximo envio após o throttle usa a nova direção da câmera.

O envio final continua usando o caminho existente:

```rust
networking_system.player_move(WorldPosition {
    x: destination.x,
    y: destination.y,
    direction: Direction::North,
});
```

Assim, o servidor continua recebendo movimento normal por tile.

## Bloqueio do Clique no Chão

Quando `third_person_movement_enabled` estiver ativo, o clique esquerdo em `PickerTarget::Tile` não deve gerar `InputEvent::PlayerMove`.

O bloqueio é específico para movimento por clique no chão. Devem continuar funcionando:

- clique em NPC para diálogo;
- clique em monstro para ataque;
- clique em item de chão para pickup;
- skills em alvo ou no chão;
- UI e janelas;
- chat;
- hotbar;
- demais atalhos e pacotes.

## Pontos de Código Esperados

- `korangar/korangar/src/settings/interface.rs`: nova configuração persistida com default compatível.
- `korangar/korangar/src/interface/windows/interface_settings.rs`: toggle visual.
- `korangar/korangar/src/input/event.rs`: evento de movimento existente pode ser reaproveitado; se necessário, adicionar evento interno para movimento WASD.
- `korangar/korangar/src/input/mod.rs`: leitura de `W/A/S/D`, estado pressionado e geração throttled de eventos.
- `korangar/korangar/src/world/cameras/`: nova câmera terceira pessoa.
- `korangar/korangar/src/main.rs`: seleção da câmera ativa, bloqueio do clique no chão e integração do evento de movimento.

## Validação

Validação manual mínima:

1. Com a opção desligada, o movimento por clique e a câmera clássica continuam como antes.
2. Com a opção ligada, segurar `W` faz o player continuar recebendo destinos de 5 tiles na direção da câmera em intervalo mínimo de 40ms quando o destino muda.
3. Soltar `W` faz o player parar de receber novos destinos e terminar no último destino.
4. `W+A` e outras diagonais funcionam sem distância exagerada.
5. Girar a câmera com botão direito enquanto anda muda os próximos destinos.
6. Clique no chão não move o player no modo terceira pessoa.
7. Clique em NPC, monstro, item, UI e uso de skill continuam funcionando.

Testes automatizados desejáveis:

- unidade para conversão de input WASD + direção da câmera em vetor de tile;
- unidade para intervalo adaptativo mínimo de 40ms;
- unidade para configuração antiga carregar com `third_person_movement_enabled = false`.

## Decisões

- O modo é cliente-only e não altera servidor.
- O pacote de movimento continua sendo o pacote normal existente.
- A câmera não controla a rotação do player como volante.
- O clique no chão é o único fluxo bloqueado para não enviar movimento ao servidor.
- Diagonal é suportada e normalizada.
- O movimento segurado usa envio adaptativo com intervalo mínimo de 40ms.
