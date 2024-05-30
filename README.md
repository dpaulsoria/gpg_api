# API - Parque Nacional Galapagos (PNG)
Backend para la plataforma personalizada SIG del PNG
# How to Use
Instalar CARGO  
Ejecutar API
# Scripts
## Init
1. Verifica si el usuario ADMIN existe
2. Crea el usuario ADMIN en caso de que no exista
3. Verifica si la base de datos GPG exista
4. Crea la base de datos GPG en caso de que no exista
5. Asigna todos los privilegios al usuario ADMIN sobre la
### Respuesta esperado
Creating user gpg_admin...
CREATE ROLE
Creating database gpg_db...
CREATE DATABASE
Granting all privileges on database gpg_db to gpg_admin...
GRANT
Privileges granted for gpg_admin on gpg_db
Setup complete.
## Reset DB

