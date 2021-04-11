const arrow_wasm = require("../pkg/arrow_wasm");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./unsd-citypopulation-year-fm.csv");
const file = fs.readFileSync(filePath, "utf-8");

const schema = arrow_wasm.infer_schema_from_csv(file, 50);
console.log(JSON.stringify(schema, undefined, 4));
