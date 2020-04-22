import init, { run_app } from './pkg/lunch_list_frontend.js';
async function main() {
   await init('/pkg/lunch_list_frontend_bg.wasm');
   run_app();
}
main()
