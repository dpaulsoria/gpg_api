#!/bin/bash

# Cargar variables de entorno
source .env

# Verificar que las variables de entorno están definidas
if [[ -z "${DB_USER}" || -z "${DB_PASS}" || -z "${DB_HOST}" || -z "${DB_PORT}" || -z "${DB_NAME}" || -z "${DATABASE_URL}" ]]; then
  echo "Error: Una o más variables de entorno no están definidas."
  exit 1
fi

DB_USER=${DB_USER}
DB_PASS=${DB_PASS}
DB_HOST=${DB_HOST}
DB_PORT=${DB_PORT}
DB_NAME=${DB_NAME}
DB_URL=${DATABASE_URL}

ADMIN_DATABASE_URL="postgres://postgres@${DB_HOST}:${DB_PORT}/postgres"

# Borrar la base de datos si existe
echo "Dropping database if exists..."
psql $ADMIN_DATABASE_URL -c "DROP DATABASE IF EXISTS $DB_NAME;"

# Crear una nueva base de datos
echo "Creating new database..."
psql $ADMIN_DATABASE_URL -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;"

# Ejecutar las migraciones desde 0
echo "Running migrations..."
sqlx migrate run --database-url $DB_URL

echo "Database reset complete."

