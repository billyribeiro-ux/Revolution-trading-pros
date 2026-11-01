const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

async function runMigrations() {
    if (!process.env.DATABASE_URL) {
        console.error('Error: DATABASE_URL environment variable is required');
        process.exit(1);
    }

    const client = new Client({
        connectionString: process.env.DATABASE_URL
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
