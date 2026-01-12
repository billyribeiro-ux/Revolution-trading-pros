#!/bin/bash
# Generate Argon2id hash for password and update database

PASSWORD="Jesusforevero1!"

# Use Python to generate Argon2id hash (if argon2-cffi is available)
HASH=$(python3 -c "
import sys
try:
    from argon2 import PasswordHasher
    ph = PasswordHasher(time_cost=3, memory_cost=65536, parallelism=4)
    print(ph.hash('$PASSWORD'))
except ImportError:
    print('ERROR: argon2-cffi not installed', file=sys.stderr)
    sys.exit(1)
" 2>&1)

if [[ $HASH == ERROR* ]]; then
    echo "Failed to generate hash: $HASH"
    echo "Installing argon2-cffi..."
    pip3 install argon2-cffi
    HASH=$(python3 -c "
from argon2 import PasswordHasher
ph = PasswordHasher(time_cost=3, memory_cost=65536, parallelism=4)
print(ph.hash('$PASSWORD'))
")
fi

echo "Generated hash: $HASH"

# Update the database
echo "Updating database..."
flyctl postgres connect -a revolution-db <<SQL
UPDATE users SET password_hash = '$HASH', updated_at = NOW() WHERE email = 'welberribeirodrums@gmail.com';
SELECT id, email, substring(password_hash, 1, 50) as hash_check FROM users WHERE email = 'welberribeirodrums@gmail.com';
\q
SQL

echo "Password updated successfully!"
