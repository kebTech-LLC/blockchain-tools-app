const fs = require('fs');
const path = require('path');

// Read package.json
const packageJsonPath = path.resolve(__dirname, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

const version = packageJson.version;
const buildDate = new Date().toISOString();
const envFilePath = '.env';
const versionFilePath = path.resolve(__dirname, 'public', 'version.txt');

// Read existing .env content if any
let envContent = '';
if (fs.existsSync(envFilePath)) {
    envContent = fs.readFileSync(envFilePath, 'utf8');
}

// Append or update the version and build date variables
const versionLine = `VITE_APP_VERSION=${version}`;
const buildDateLine = `VITE_BUILD_DATE=${buildDate}`;

let newEnvContent = envContent.includes('VITE_APP_VERSION')
    ? envContent.replace(/VITE_APP_VERSION=.*/, versionLine)
    : `${envContent}\n${versionLine}`;

newEnvContent = newEnvContent.includes('VITE_BUILD_DATE')
    ? newEnvContent.replace(/VITE_BUILD_DATE=.*/, buildDateLine)
    : `${newEnvContent}\n${buildDateLine}`;

fs.writeFileSync(envFilePath, newEnvContent.trim(), 'utf8');

console.log(`Version ${version} and build date ${buildDate} added to ${envFilePath}`);

// Create or update the version.txt file in the public directory
const versionTextContent = `${version}`;
fs.writeFileSync(versionFilePath, versionTextContent, 'utf8');

console.log(`Version and build date added to ${versionFilePath}`);
