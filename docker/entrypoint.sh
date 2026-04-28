#!/bin/bash
# Entrypoint for the rathena image. Compiles the servers on first run, then
# launches whichever service was requested (build / login / char / map).
set -euo pipefail

cd /rathena

DB_HOST="${DB_HOST:-mariadb}"
DB_PORT="${DB_PORT:-3306}"
DB_USER="${DB_USER:-ragnarok}"
DB_PASS="${DB_PASS:-ragnarok}"
DB_NAME="${DB_NAME:-ragnarok}"
PACKETVER="${PACKETVER:-20220406}"

log() { echo ">>> [rathena-docker] $*"; }

# Mirror config overrides from /docker-import into rAthena's conf/import/.
# We cannot bind-mount individual files into a folder that is itself bind-mounted
# (Docker on Windows is unreliable about it), so we copy on every container start.
sync_config_imports() {
    if [ -d /docker-import ]; then
        mkdir -p /rathena/conf/import
        cp -f /docker-import/*.txt /rathena/conf/import/ 2>/dev/null || true
    fi
    # rAthena writes pid/log files into these, make sure they exist.
    mkdir -p /rathena/log /rathena/save
}

build_servers() {
    if [ -x /rathena/login-server ] && [ -x /rathena/char-server ] && [ -x /rathena/map-server ]; then
        log "Server binaries already present, skipping build."
        return
    fi

    log "Building rAthena (this takes ~5-10 minutes the first time)..."
    if [ ! -f /rathena/Makefile ]; then
        log "Running ./configure --enable-packetver=${PACKETVER}"
        ./configure --enable-packetver="${PACKETVER}"
    fi
    # Serial build: rAthena's Makefile dependencies don't survive `make -j`
    # cleanly (rapidyaml/httplib race), and the official builder.sh uses
    # serial `make clean server` for the same reason.
    log "Running: make clean server"
    make clean
    make server
    log "Build complete."
}

force_rebuild() {
    log "Force rebuild: wiping binaries, Makefile and configure artifacts."
    rm -f /rathena/login-server /rathena/char-server /rathena/map-server
    rm -f /rathena/Makefile /rathena/config.log /rathena/config.status
    rm -rf /rathena/autom4te.cache
    build_servers
}

wait_for_db() {
    log "Waiting for ${DB_HOST}:${DB_PORT} to accept TCP..."
    /usr/local/bin/wait-for "${DB_HOST}:${DB_PORT}" -t 180 -- true
    log "Waiting for MariaDB to accept queries..."
    local tries=0
    until mariadb -h "${DB_HOST}" -P "${DB_PORT}" -u "${DB_USER}" -p"${DB_PASS}" \
            -D "${DB_NAME}" -e 'SELECT 1' >/dev/null 2>&1; do
        tries=$((tries + 1))
        if [ "${tries}" -gt 60 ]; then
            log "MariaDB did not become ready in time."
            exit 1
        fi
        sleep 2
    done
    log "Database is ready."
}

wait_for_tcp() {
    local host="$1" port="$2"
    log "Waiting for ${host}:${port}..."
    /usr/local/bin/wait-for "${host}:${port}" -t 180 -- true
}

case "${1:-help}" in
    build)
        sync_config_imports
        build_servers
        ;;
    rebuild)
        sync_config_imports
        force_rebuild
        ;;
    login)
        sync_config_imports
        build_servers
        wait_for_db
        log "Starting login-server"
        exec ./login-server
        ;;
    char)
        sync_config_imports
        build_servers
        wait_for_db
        wait_for_tcp "${LOGIN_HOST:-login}" "${LOGIN_PORT:-6900}"
        log "Starting char-server"
        exec ./char-server
        ;;
    map)
        sync_config_imports
        build_servers
        wait_for_db
        wait_for_tcp "${CHAR_HOST:-char}" "${CHAR_PORT:-6121}"
        log "Starting map-server"
        exec ./map-server
        ;;
    shell|bash|sh)
        exec /bin/bash
        ;;
    help|"")
        cat <<'USAGE'
rathena docker entrypoint

usage: <build|rebuild|login|char|map|shell>

  build    - compile the server binaries (login/char/map) and exit
  rebuild  - wipe binaries + Makefile and run a clean build (forces the
             current PACKETVER and any source edits to be picked up)
  login    - run the login-server (waits for the database)
  char     - run the char-server  (waits for the database + login)
  map      - run the map-server   (waits for the database + char)
  shell    - drop into an interactive bash shell inside the container
USAGE
        ;;
    *)
        # Allow `docker compose run rathena make clean` etc.
        exec "$@"
        ;;
esac
