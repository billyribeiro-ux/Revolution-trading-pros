const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

async function runMigrations() {
    const client = new Client({
        connectionString: process.env.DATABASE_URL || 'postgres://revolution_trading_pros:iSovKZXmJMYAQzm@localhost:5433/revolution_trading_pros'
    });

    try {
        await client.connect();
        console.log('Connected to database');

        const migrationFile = path.join(__dirname, 'migrations', '001_initial_schema.sql');
        const sql = fs.readFileSync(migrationFile, 'utf8');
        
        console.log('Running migrations...');
        await client.query(sql);
        console.log('Migrations completed successfully!');
    } catch (err) {
        console.error('Migration error:', err.message);
    } finally {
        await client.end();
    }
}

runMigrations();
