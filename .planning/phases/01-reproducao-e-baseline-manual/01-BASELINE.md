# Phase 01 - Baseline Manual de Respawn

**Data/hora:** 2026-04-26T22:29:41.1910592-03:00  
**Executor:** Codex + verificacao humana do cliente real  
**Git/status relevante:** HEAD `bca0738`; `.planning/STATE.md` atualizado pelo inicio de execucao; arquivos de codigo/infra existentes permanecem fora do escopo desta fase.  
**Objetivo:** reproduzir o estado atual de `@kill -> Respawn` antes de qualquer fix permanente.

## Ambiente

| Item | Valor observado |
|------|-----------------|
| Windows cwd | `D:\ragnarok` |
| Docker services | `rathena-db`, `rathena-login`, `rathena-char`, `rathena-map` ativos; MariaDB healthy |
| Portas locais | `6900=True`, `6121=True`, `5121=True` em `127.0.0.1` |
| PACKETVER | `20220406` |
| Cliente | `korangar\target\release\korangar.exe` existe |
| Launcher | `.\play.bat` existe |
| Assets | `korangar\korangar\data.grf` e `korangar\korangar\rdata.grf` existem |
| Conta/personagem | Pendente de verificacao manual; primeira tentativa recomendada: `admin` / `123` |

## Comandos Executados

```powershell
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
docker compose logs --no-color --tail=80 login char map
Test-NetConnection -ComputerName 127.0.0.1 -Port 6900 -InformationLevel Quiet
Test-NetConnection -ComputerName 127.0.0.1 -Port 6121 -InformationLevel Quiet
Test-NetConnection -ComputerName 127.0.0.1 -Port 5121 -InformationLevel Quiet
Test-Path .\play.bat
Test-Path .\korangar\target\release\korangar.exe
Test-Path .\korangar\korangar\data.grf
Test-Path .\korangar\korangar\rdata.grf
```

## Smoke Operacional

### Docker Compose

```text
NAME            SERVICE   STATUS                 PORTS
rathena-char    char      Up                     0.0.0.0:6121->6121/tcp
rathena-db      mariadb   Up (healthy)           0.0.0.0:3306->3306/tcp
rathena-login   login     Up                     0.0.0.0:6900->6900/tcp
rathena-map     map       Up                     0.0.0.0:5121->5121/tcp
```

### Portas

```text
127.0.0.1:6900=True
127.0.0.1:6121=True
127.0.0.1:5121=True
```

### Arquivos Locais

```text
.\play.bat=True
.\korangar\target\release\korangar.exe=True
.\korangar\korangar\data.grf=True
.\korangar\korangar\rdata.grf=True
```

### Logs Relevantes

- `rathena-login`: servidor pronto na porta `6900`; autenticacoes recentes para `admin` e `jonato` aparecem nos logs.
- `rathena-char`: selecoes recentes de personagem `jonato` aparecem nos logs.
- `rathena-map`: servidor pronto na porta `5121`; map-server conectado ao char-server.

## Fluxo Manual

Status: pendente de execucao humana no cliente real.

Passos planejados:

1. Abrir o cliente via `.\play.bat` a partir de `D:\ragnarok`.
2. Entrar com conta local GM, preferencialmente `admin` / `123`.
3. Selecionar personagem e entrar no mapa.
4. Executar `@kill` no chat.
5. Confirmar que a janela Respawn aparece.
6. Clicar Respawn.
7. Registrar pose visual final, janela Respawn, HP e movimento.

## Resultado Observado

Pendente de checkpoint humano.

Preencher apos observacao:

| Criterio | Resultado |
|----------|-----------|
| Conta/personagem usados | Pendente |
| `@kill` executado | Pendente |
| Janela Respawn apareceu | Pendente |
| Respawn clicado | Pendente |
| Player visualmente vivo/em pe | Pendente |
| Player visualmente morto/deitado | Pendente |
| Janela Respawn fechada | Pendente |
| HP restaurado | Pendente |
| Movimento funcional | Pendente |
| Bug atual reproduzido | Pendente |

## Evidencias Disponiveis

- Smoke de Docker/portas/arquivos registrado acima.
- Logs de servidor mostram stack local operacional e logins recentes.
- Evidencia visual do cliente ainda depende do checkpoint humano.

## Conclusao

REPR-01: coberto, ambiente local necessario esta pronto para executar o fluxo de respawn.  
REPR-02: pendente, requer observacao humana do bug no cliente real.  
REPR-03: parcial, roteiro e ambiente registrados; resultado visual ainda pendente.
