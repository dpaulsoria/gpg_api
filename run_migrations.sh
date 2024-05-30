# !/bin/bash

# Cargar variables de entorno
source .env

# Ejecutar las migraciones pendientes
echo "Running pending migrations..."
sqlx migrate run
echo "Pending migrations applied."

