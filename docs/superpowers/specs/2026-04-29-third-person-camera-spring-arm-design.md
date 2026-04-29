# Third-Person Camera Spring Arm Collision Design

## Contexto

A camera em terceira pessoa do Korangar deve seguir o jogador como uma camera moderna de engine 3D. A tentativa anterior evitava atravessar obstaculos, mas podia aproximar a camera ate a localizacao do jogador quando o caminho estava bloqueado cedo demais. Esse comportamento e ruim: a camera nao deve colapsar para o player como solucao normal.

## Objetivo

Refazer a fisica de colisao da `ThirdPersonCamera` usando o modelo de Spring Arm com "Do Collision Test":

- manter um braco ideal entre o pivo do jogador e a camera;
- fazer um trace/sweep entre o pivo e a posicao ideal da camera;
- se nao houver colisao, usar o comprimento total do braco;
- se houver colisao, encurtar o braco para parar antes do ponto de impacto;
- quando o obstaculo sair do caminho, retornar suavemente para a distancia desejada;
- nunca atravessar objetos fisicos do mapa;
- nunca usar a posicao do jogador como fallback visual normal.

## Fora de Escopo

- Alterar `rathena-master/`.
- Mudar `PACKETVER=20220406`.
- Trocar o modo de movimento WASD.
- Criar fisica client-authoritative para o jogador.
- Refatorar cameras que nao sejam afetadas diretamente pela extracao/reuso do resolvedor.

## Design

### Spring Arm

A terceira pessoa passa a tratar a camera como um braco:

- `pivot_position`: foco suavizado do player mais `HEIGHT_OFFSET`;
- `target_arm_length`: distancia desejada atual, controlada por zoom e clamp existente;
- `arm_direction`: direcao do pivo para a camera, igual a `-view_direction`;
- `desired_camera_position`: `pivot_position + arm_direction * target_arm_length`;
- `resolved_arm_length`: distancia final depois do collision test;
- `camera_position`: `pivot_position + arm_direction * resolved_arm_length`.

### Collision Test

O collision test deve consultar objetos fisicos do mapa usando o caminho ja existente em `Map::first_object_intersection_fraction`, que testa o segmento contra AABBs via `object_kdtree`.

Regras:

- Sem hit: `resolved_arm_length = target_arm_length`.
- Com hit: `resolved_arm_length = max(min_arm_length, hit_distance - probe_radius)`.
- `probe_radius` funciona como margem/sweep simples para impedir que a camera encoste ou atravesse quinas.
- `min_arm_length` evita que a camera caia exatamente no pivo/player.
- O teste deve ser deterministico e puro o suficiente para ter unidade sem depender de mapa real.

### Suavizacao

O foco do jogador e a distancia desejada continuam usando os `SmoothedValue` existentes. O retorno da camera para a distancia total tambem deve ser suave, mas a aproximacao causada por colisao pode ser imediata ou mais rapida para impedir clipping.

Comportamento esperado:

- obstaculo entra no caminho: camera encurta rapido antes do impacto;
- obstaculo sai do caminho: camera alonga suavemente ate a distancia desejada;
- player perto de parede: camera fica numa distancia curta util, nao dentro do player;
- player em area aberta: camera volta ao enquadramento normal.

### Integracao

`ThirdPersonCamera::update` deve receber `Option<&Map>` e resolver a camera com o Spring Arm quando houver mapa. Sem mapa, usa a posicao ideal antiga.

Se houver helper compartilhado com camera cinematica, ele precisa preservar o comportamento cinematico existente, mas a terceira pessoa nao deve depender de step sampling por tile como autoridade principal.

## Testes

Testes automatizados esperados:

- sem colisao, o Spring Arm retorna o comprimento total;
- com colisao no meio do caminho, retorna `hit_distance - probe_radius`;
- hit muito proximo respeita `min_arm_length`, nao zero;
- camera terceira pessoa usa a posicao ideal sem mapa;
- camera terceira pessoa encurta o braco quando o resolvedor informa hit;
- camera terceira pessoa nao retorna exatamente o pivo/player como fallback normal.

## Criterio de Aceite Manual

No jogo real, com modo terceira pessoa ativo:

1. Em area aberta, a camera segue atras do jogador na distancia configurada.
2. Ao colocar parede/arvore/predio entre jogador e camera, a camera aproxima apenas o necessario.
3. A camera nao atravessa o objeto fisico.
4. A camera nao vai para a localizacao do jogador como comportamento normal.
5. Ao sair do obstaculo, a camera volta suavemente para tras.
