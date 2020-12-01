// MIT/Apache2 License

const fs = require("fs");
const sass = require("node-sass");
const process = require("process");

const scss_path = "tutorials/style.scss";
const out_path = "tutorials/style.css";

sass.render({
  file: scss_path,
}, function(err, result) {
  if (err) {
    console.error(`FATAL ERROR: ${err}`);
    process.exit(1);
  }

  fs.writeFileSync(out_path, result.css);
});
