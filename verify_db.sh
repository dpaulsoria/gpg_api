#!/bin/bash

# Cargar variables de entorno 
source .env

# Verificar la estructura de la base de datos
echo "Verifying database structure..."
if sqlx migrate info | grep -q "Missing"; then
  echo "Database structure is incorrect."
  echo "Some migrations are missing."
else
  echo "Database structure is correct."
  echo "All migrations are applied."
fi
