require('dotenv').config();
const fs = require('fs');

const abisDir = process.env.ABI_DIR;
const buildDir = './build/contracts/';
console.log(process.env.ABI_DIR, fs.existsSync(abisDir))
if (!fs.existsSync(abisDir)) {
	throw new Error('ABI directory not exists');
}
if (!fs.existsSync(buildDir)) {
	throw new Error('No truffle build/contracts directory');
}
const files = fs.readdirSync(buildDir);
console.log(files.length )
for (var i = files.length - 1; i >= 0; i--) {
	var name = files[i];
	var contract = JSON.parse(fs.readFileSync(buildDir + name), 'utf8');
	var abi = JSON.stringify(contract.abi);
	fs.writeFileSync(abisDir + name.toLocaleLowerCase(), abi, 'utf8');
	console.log(name + ': Done');
}
