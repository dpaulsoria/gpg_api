#!/bin/bash

# Cargar variables de entorno
source .env

# Verificar si las variables de entorno están definidas
if [[ -z "${DB_USER}" || -z "${DB_PASS}" || -z "${DB_HOST}" || -z "${DB_PORT}" || -z "${DB_NAME}" || -z "${DATABASE_URL}" ]]; then
  echo "Error: Una o más variables de entorno no están definidas."
  exit 1
fi

DB_USER=${DB_USER}
DB_PASS=${DB_PASS}
DB_HOST=${DB_HOST}
DB_PORT=${DB_PORT}
DB_NAME=${DB_NAME}
DB_URL=${DB_URL}

ADMIN_DATABASE_URL="postgres://postgres@${DB_HOST}:${DB_PORT}/postgres"

# Verificar si el usuario existe
USER_EXISTS=$(psql $ADMIN_DATABASE_URL -tAc "SELECT 1 FROM pg_roles WHERE rolname='$DB_USER'")

# Crear el usuario si no existe
if [ "$USER_EXISTS" != "1" ]; then
  echo "Creating user $DB_USER..."
  psql $ADMIN_DATABASE_URL -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASS';"
else
  echo "User $DB_USER already exists."
fi

# Verificar si la base de datos existe
DB_EXISTS=$(psql $ADMIN_DATABASE_URL -tAc "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'")

# Crear la base de datos si no existe
if [ "$DB_EXISTS" != "1" ]; then
  echo "Creating database $DB_NAME..."
  psql $ADMIN_DATABASE_URL -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;"
else
  echo "Database $DB_NAME already exists."
fi

# Asignar todos los privilegios al usuario sobre la base de datos
echo "Granting all privileges on database $DB_NAME to $DB_USER..."
psql $ADMIN_DATABASE_URL -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

# Verificar si se otorgaron los privilegios correctamente
PRIVILEGES_GRANTED=$(psql $ADMIN_DATABASE_URL -tAc "SELECT has_database_privilege('$DB_USER', '$DB_NAME', 'CONNECT')")

if [ "$PRIVILEGES_GRANTED" = "t" ]; then
  echo "Privileges granted for $DB_USER on $DB_NAME"
else
  echo "Privileges were not granted for $DB_USER on $DB_NAME"
fi

echo "Setup complete."
