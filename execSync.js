import { execSync } from "child_process";

const logs = execSync(`echo foo | nodejs-pipes --flag-one --flag-two flag-two-value`, {
  stdio: 'pipe'
});

console.log(`>>> logs:\n${logs}`);
console.log(`<<< logs`);