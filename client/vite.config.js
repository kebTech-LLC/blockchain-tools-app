import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
// import wasmPack from 'vite-plugin-wasm-pack';
import { exec } from 'child_process';
// Determine which publish script to run based on environment variable
var target = process.env.PUBLISH_TARGET || 'stg'; // Default to 'stg' if not set
export default defineConfig({
    plugins: [
        vue(),
        // wasmPack('./src/modules/wasm/rust'),
        {
            name: 'run-commands',
            buildStart: function () {
                // Run "cnctd bump" before the build starts
                exec('cnctd bump', function (err, stdout, stderr) {
                    if (err) {
                        console.error("Error executing cnctd bump: ".concat(stderr));
                        return;
                    }
                    console.log("cnctd bump output: ".concat(stdout));
                });
                exec('node set-version.cjs', function (err, stdout, stderr) {
                    if (err) {
                        console.error("Error executing set-version: ".concat(stderr));
                        return;
                    }
                    console.log("set-version output: ".concat(stdout));
                });
            },
            // closeBundle: function () {
            //     exec("sh ./publish_client.sh -t ".concat(target), function (err, stdout, stderr) {
            //         if (err) {
            //             console.error("Error executing publish script: ".concat(stderr));
            //             return;
            //         }
            //         console.log("Script output: ".concat(stdout));
            //     });
            //     exec('npx cap sync', function (err, stdout, stderr) {
            //         if (err) {
            //             console.error("Error executing cap ios: ".concat(stderr));
            //             return;
            //         }
            //         console.log("cap ios output: ".concat(stdout));
            //     });
            // }
        }
    ],
    resolve: {
        alias: {
            '@': '/src',
        }
    },
    optimizeDeps: {
        include: ['mic-recorder-to-mp3']
    }
});
